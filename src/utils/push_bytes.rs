//////////////////////////////////////////////////////////////////////////
/// On n'utilise pas copy_from_slice, je le redev pour plusieurs raison :
/// copy_from_slice est moins flexible si on manipule des sous-slices.
/// Avec une boucle manuelle, on peut écrire les octets un par un, ce qui permet
/// de facilement transformer et verifier octet.
/// Cela évite l'utilisation de fonctions std
///
/// https://doc.rust-lang.org/std/primitive.slice.html
/////////////////////////////////////////////////////////
 
/// Fonction push_bytes redev
pub fn push_bytes(
    buf: &mut [
        u8
    ], 
    offset: usize, 
    data: &[
        u8
    ]
) -> usize {
    let mut off = offset;
    for &b in data {
        buf[
            off
        ] = b;
        off += 1;
    }
    off
}


