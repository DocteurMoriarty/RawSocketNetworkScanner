use alloc::string::String;


pub fn my_as_deref(
    opt: &Option<String>
) -> 
Option<
    &str
> 
{
    if let Some(
        s
    ) = opt {
        let slice: &str = &s[..];
        Some(
            slice
        )
    } else {
        None
    }
}
