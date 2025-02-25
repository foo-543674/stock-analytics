#[cfg(test)]
macro_rules! assert_result_is_err {
  ($result:expr) => {
    match $result {
      Ok(_) => panic!("Expected Err but got Ok"),
      Err(_) => assert!(true),
    }
  };
}

#[cfg(test)]
macro_rules! assert_result_is_ok {
  ($result:expr) => {
    match $result {
      Ok(_) => assert!(true),
      Err(err) => panic!("Expected Ok but got Err({:?})", err),
    }
  };
}

#[cfg(test)]
pub(crate) use assert_result_is_err;
#[cfg(test)]
pub(crate) use assert_result_is_ok;
