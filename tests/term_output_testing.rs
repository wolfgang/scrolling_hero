

use console::{Term};
use std::io::{Cursor, Write};
use std::str;


#[test]

fn can_pass_term_and_cursor_to_same_function() {
    let mut term = Term::stdout();
    write(&mut term, b"HELLO\n").unwrap();

    let mut buff = Cursor::new(vec![]);

    write(&mut buff, b"HELLO2").unwrap();

    let reference = buff.get_ref();
    assert_eq!("HELLO2", str::from_utf8(reference).unwrap());
}


fn write<W: Write>(writer: &mut W, buf: &[u8]) -> std::io::Result<()> {
    writer.write(buf)?;
    Ok(())
}
