/// Calcule la longueur du payload s'il est present, sinon retourne 0.
pub fn payload_len<T>(payload: &Option<T>) -> usize
where
    T: AsRef<[u8]>,
{
    match payload {
        Some(p) => p.as_ref().len(),
        None => 0,
    }
}