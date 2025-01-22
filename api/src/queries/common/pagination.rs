#[derive(Debug, Clone, Copy)]
pub struct PageNumer(u32);

impl PageNumer {
  pub fn new(page: u32) -> Self {
    if page < 1 {
      panic!("Page number must be greater than 0");
    }

    PageNumer(page)
  }

  pub fn value(&self) -> u32 {
    self.0
  }

  pub fn from_int(value: i32) -> Self {
    if value < 1 {
      PageNumer(1)
    } else {
      PageNumer(value as u32)
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct ItemsPerPage(u32);

const MAX_ITEMS_PER_PAGE: u32 = 100;
const DEFAULT_ITEMS_PER_PAGE: u32 = 10;

impl ItemsPerPage {
  pub fn new(items_per_page: u32) -> Self {
    if items_per_page < 1 {
      panic!("Items per page must be greater than 0");
    }
    if items_per_page > MAX_ITEMS_PER_PAGE {
      panic!("Items per page must be less than or equal to {}", MAX_ITEMS_PER_PAGE);
    }

    ItemsPerPage(items_per_page)
  }

  pub fn value(&self) -> u32 {
    self.0
  }

  pub fn from_int(value: i32) -> Self {
    if 1 <= value && value <= MAX_ITEMS_PER_PAGE as i32 {
      ItemsPerPage(value as u32)
    } else {
      ItemsPerPage(DEFAULT_ITEMS_PER_PAGE)
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use proptest::prelude::*;

  #[test]
  #[should_panic(expected = "Page number must be greater than 0")]
  fn page_number_should_be_panic_when_value_less_than_1() {
    PageNumer::new(0);
  }

  proptest! {
    #[test]
    fn page_number_should_be_ok_when_value_greater_than_0(value in 1u32..=u32::MAX) {
      let page_number = PageNumer::new(value);
      assert_eq!(page_number.value(), value);
    }

    #[test]
    fn page_number_should_be_as_is_when_value_is_positive(value in 1i32..=i32::MAX) {
      let page_number = PageNumer::from_int(value);
      assert_eq!(page_number.value(), value as u32);
    }

    #[test]
    fn page_number_should_be_1_when_value_is_negative(value in i32::MIN..=0i32) {
      let page_number = PageNumer::from_int(value);
      assert_eq!(page_number.value(), 1);
    }
  }

  #[test]
  #[should_panic(expected = "Items per page must be greater than 0")]
  fn items_per_page_should_be_panic_when_value_less_than_1() {
    ItemsPerPage::new(0);
  }

  #[test]
  #[should_panic(expected = "Items per page must be less than or equal to 100")]
  fn items_per_page_should_be_panic_when_value_greater_than_100() {
    ItemsPerPage::new(101);
  }

  proptest! {
    #[test]
    fn items_per_page_should_be_ok_when_value_between_1_and_100(value in 1u32..=100u32) {
      let items_per_page = ItemsPerPage::new(value);
      assert_eq!(items_per_page.value(), value);
    }

    #[test]
    fn items_per_page_should_be_as_is_when_value_between_1_to_100(value in 1i32..=100i32) {
      let items_per_page = ItemsPerPage::from_int(value);
      assert_eq!(items_per_page.value(), value as u32);
    }

    #[test]
    fn items_per_page_should_be_default_when_value_is_negative(value in i32::MIN..=0i32) {
      let items_per_page = ItemsPerPage::from_int(value);
      assert_eq!(items_per_page.value(), DEFAULT_ITEMS_PER_PAGE);
    }

    #[test]
    fn items_per_page_should_be_default_when_value_is_greater_than_100(value in 101i32..=i32::MAX) {
      let items_per_page = ItemsPerPage::from_int(value);
      assert_eq!(items_per_page.value(), DEFAULT_ITEMS_PER_PAGE);
    }
  }
}