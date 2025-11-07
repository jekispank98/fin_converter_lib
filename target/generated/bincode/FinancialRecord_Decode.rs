impl < __Context > :: bincode :: Decode < __Context > for FinancialRecord
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            tx_id : :: bincode :: Decode :: decode(decoder) ?, tx_type : ::
            bincode :: Decode :: decode(decoder) ?, from_user_id : :: bincode
            :: Decode :: decode(decoder) ?, to_user_id : :: bincode :: Decode
            :: decode(decoder) ?, amount : :: bincode :: Decode ::
            decode(decoder) ?, timestamp : :: bincode :: Decode ::
            decode(decoder) ?, status : :: bincode :: Decode ::
            decode(decoder) ?, description : :: bincode :: Decode ::
            decode(decoder) ?,
        })
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for FinancialRecord
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        core :: result :: Result ::
        Ok(Self
        {
            tx_id : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, tx_type : :: bincode :: BorrowDecode ::<
            '_, __Context >:: borrow_decode(decoder) ?, from_user_id : ::
            bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, to_user_id : :: bincode :: BorrowDecode
            ::< '_, __Context >:: borrow_decode(decoder) ?, amount : ::
            bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, timestamp : :: bincode :: BorrowDecode
            ::< '_, __Context >:: borrow_decode(decoder) ?, status : ::
            bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, description : :: bincode :: BorrowDecode
            ::< '_, __Context >:: borrow_decode(decoder) ?,
        })
    }
}