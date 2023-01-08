#[cfg(test)]
pub mod test {
    use crate::byte_buf::{
        ByteBuf,
        ReadByteBuf
    };

    #[test]
    fn test_get_number() {
        let array: [u8; 2] = [0x01, 0x02];
        let  read_byte_buf = ReadByteBuf::new_from(&array);

        println!("{:?}", read_byte_buf.get_u8());
        println!("{:?}", read_byte_buf.get_u16_be());
        println!("{:?}", read_byte_buf.get_u16_le());
        println!("{:?}", read_byte_buf.get_u32_le());
    }

    #[test]
    fn test_get_bytes() {
        let array: [u8; 2] = [0x01, 0x02];
        let  read_byte_buf = ReadByteBuf::new_from(&array);

        println!("{:?}", read_byte_buf.get_bytes_of_length(2));

        let mut write_bytes = [0u8; 2];
        println!("{:?}", read_byte_buf.get_bytes_of_write(&mut write_bytes));
        println!("{:?}", write_bytes);

        let mut write_bytes = [0u8; 3];
        println!("{:?}", read_byte_buf.get_bytes_of_write(&mut write_bytes));
        println!("{:?}", write_bytes);
    }

    #[test]
    fn test_read_number() {
        let array: [u8; 4] = [0x01, 0x02, 0x01, 0x02];
        let mut read_byte_buf = ReadByteBuf::new_from(&array);

        // println!("{:?}", read_byte_buf.read_i8());
        // println!("{:?}", read_byte_buf.read_u8());
        println!("{:?}", read_byte_buf.read_u16_le());
        println!("{:?}", read_byte_buf.read_i16_le());
    }

    #[test]
    fn test_read_bytes() {
        let array: [u8; 2] = [0x01, 0x02];
        let mut read_byte_buf = ReadByteBuf::new_from(&array);

        println!("{:?}", read_byte_buf.read_bytes_of_length(2));

        let mut write_bytes = [0u8; 2];
        println!("{:?}", read_byte_buf.read_bytes_of_write(&mut write_bytes));
        println!("{:?}", write_bytes);

        let mut write_bytes = [0u8; 3];
        println!("{:?}", read_byte_buf.read_bytes_of_write(&mut write_bytes));
        println!("{:?}", write_bytes);
    }


}