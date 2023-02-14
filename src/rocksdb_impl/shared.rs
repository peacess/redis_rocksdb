use std::ptr;

pub(crate) fn make_field_key(key: &[u8], field: &[u8]) -> Vec<u8> {
    let mut new_key = Vec::with_capacity(key.len() + field.len() + 3);
    unsafe {//这里使用性能更高的 copy_nonoverlapping
        let mut p = new_key.as_mut_ptr();
        ptr::copy_nonoverlapping(key.as_ptr(), p, key.len());
        p = p.offset(key.len() as isize);
        *p = ':' as u8;
        *(p.offset(1)) = '_' as u8;
        *(p.offset(2)) = '_' as u8;
        p = p.offset(3);
        ptr::copy_nonoverlapping(field.as_ptr(), p, field.len());
        new_key.set_len(new_key.capacity());
    }
    return new_key;
}


#[inline]
pub(crate) fn make_head_key(key: &[u8]) -> Vec<u8> {
    return make_field_key(key, &[]);
}

pub(crate) fn get_field_from_key<'a>(key: &[u8], field_key: &'a [u8]) -> &'a [u8] {
    &field_key[key.len() + 3..]
}