/// Internet checksum.
/// 
/// This implements the standard RFC 1071 algorithm:
/// 
/*


   in 6
       {
           /* Compute Internet Checksum for "count" bytes
            *         beginning at location "addr".
            */
       register long sum = 0;

        while( count > 1 )  {
           /*  This is the inner loop */
               sum += * (unsigned short) addr++;
               count -= 2;
       }

           /*  Add left-over byte, if any */
       if( count > 0 )
               sum += * (unsigned char *) addr;

           /*  Fold 32-bit sum to 16 bits */
       while (sum>>16)
           sum = (sum & 0xffff) + (sum >> 16);

       checksum = ~sum;
   }



*/




pub fn internet_checksum(
    data: &[u8]
) -> u16 {
    let mut sum: u32 = 0;
    let mut len = data.len();
    let mut i = 0;

    while len > 1 {
        let word = (
            (
                data[i] as u16
            ) << 8
        ) | (
            data[i + 1] as u16
        );
        sum = sum + (
            word as u32
        );
        i += 2;
        len -= 2;
    }

    if len > 0 {
        let word = (
            data[i] as u16
        ) << 8;
        sum = sum + (
            word as u32
        );
    }

    while (
        sum >> 16
    ) != 0 {
        sum = (
            sum & 0xFFFF
        ) + (
            sum >> 16
        );
    }

    let checksum: u16 = (
        !sum & 0xFFFF
    ) as u16;

    checksum
}



