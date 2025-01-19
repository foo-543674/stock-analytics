#[cfg(test)]
pub fn create_mock<T: Default>(setup: impl FnOnce(&mut T)) -> T {
  let mut mock = T::default();
  setup(&mut mock);
  mock
}

#[cfg(test)]
macro_rules! box_ok {
  ($value:expr) => {
    Box::pin(async { Ok($value) })
  };
}

#[cfg(test)]
pub(crate) use box_ok;

#[cfg(test)]
macro_rules! once_returning {
  ($mock:ident, $method:ident, $value:expr) => {
    $mock.$method().times(1).returning(|_| $value);
  };
}

#[cfg(test)]
pub(crate) use once_returning;

#[cfg(test)]
macro_rules! do_not_call {
  ($mock:ident, $method:ident) => {
    $mock.$method().times(0);
  };
}

#[cfg(test)]
pub(crate) use do_not_call;