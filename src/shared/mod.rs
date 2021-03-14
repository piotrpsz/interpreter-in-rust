#[allow(dead_code)]

pub fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

macro_rules! fpos {
   () => {{
      fn f() {}
      fn type_name_of<T>(_: T) -> &'static str {
         std::any::type_name::<T>()
      }
      let fname = type_name_of(f);
      let name = match &fname[..fname.len()-3].rfind(':') {
            Some(idx) => &fname[idx+1..fname.len()-3],
            _ => &fname[..fname.len()-3]
         };
      &format!("[{}::{}:{}] Error", file!(), name, line!())[..]
   }}
}

pub mod datime;
pub mod file;
