mod csv;

struct Scv;
impl<W: Write, T> Serializer<W, T> for Scv {}