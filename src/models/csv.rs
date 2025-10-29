use std::io::{BufRead, Read};
use csv::{ReaderBuilder, StringRecord, Trim};
use crate::handler::Parser;
use crate::models::financial_record::FinancialRecord;
use crate::error::ParserError;
use crate::result::Result;

pub struct CsvParser;

fn parse_amount(s: &str) -> Option<f64> {
    let s = s.replace('\u{00A0}', "");
    let s = s.trim().replace(',', ".");
    if s.is_empty() { None } else { s.parse::<f64>().ok() }
}

fn strip_bom(s: &str) -> &str {
    s.strip_prefix('\u{feff}').unwrap_or(s)
}

fn find_header_indices(header: &StringRecord) -> Option<(usize, usize, usize, usize)> {
    let mut date_idx = None;
    let mut debit_idx = None;
    let mut credit_idx = None;
    let mut descr_idx = None;

    for (i, col) in header.iter().enumerate() {
        let name = strip_bom(col).trim().to_lowercase();
        if name.contains("дата проводки") || name.eq("date") {
            date_idx = Some(i);
        } else if name.contains("сумма по дебету") || name.eq("debit") {
            debit_idx = Some(i);
        } else if name.contains("сумма по кредиту") || name.eq("credit") {
            credit_idx = Some(i);
        } else if name.contains("назначение платежа") || name.contains("description") {
            descr_idx = Some(i);
        }
    }

    match (date_idx, debit_idx, credit_idx, descr_idx) {
        (Some(d), Some(deb), Some(cred), Some(desc)) => Some((d, deb, cred, desc)),
        _ => None,
    }
}

impl<R: BufRead> Parser<R> for CsvParser {
    type Item = FinancialRecord;
    type Error = ParserError;

    fn parse(&mut self, mut reader: R) -> Result<Vec<Self::Item>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).map_err(ParserError::Io)?;

        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .trim(Trim::All)
            .delimiter(b',')
            .from_reader(&buf[..]);

        let mut started = false;
        let mut idxs: Option<(usize, usize, usize, usize)> = None;
        let mut out = Vec::new();

        for rec_res in rdr.records() {
            let rec = rec_res.map_err(|e| ParserError::Format(e.to_string()))?;
            if rec.iter().all(|f| f.trim().is_empty()) {
                continue;
            }

            if !started {
                if let Some(found) = find_header_indices(&rec) {
                    idxs = Some(found);
                    started = true;
                }
                continue;
            }

            let (date_i, debit_i, credit_i, descr_i) = match idxs {
                Some(v) => v,
                None => continue,
            };

            let get = |i: usize| rec.get(i).map(|s| strip_bom(s).trim());
            let date = match get(date_i) {
                Some(d) if !d.is_empty() => d.to_string(),
                _ => continue,
            };

            let debit = get(debit_i).unwrap_or("").to_string();
            let credit = get(credit_i).unwrap_or("").to_string();
            let description = get(descr_i).unwrap_or("").to_string();

            let amount = match (parse_amount(&credit), parse_amount(&debit)) {
                (Some(c), _) => c,
                (None, Some(d)) => -d,
                (None, None) => continue,
            };

            out.push(FinancialRecord {
                date,
                amount,
                description,
            });
        }

        if !started {
            return Err(ParserError::Format(
                "Не удалось найти строку заголовка с колонками «Дата проводки / Сумма по дебету / Сумма по кредиту / Назначение платежа»"
                    .into(),
            ));
        }

        Ok(out)
    }
}