#[derive(Debug, Clone, Copy)]
pub struct PageNumer(u32);

impl PageNumer {
  pub fn new(page: u32) -> Self {
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
    if items_per_page > MAX_ITEMS_PER_PAGE {
      panic!("Items per page must be less than or equal to {MAX_ITEMS_PER_PAGE}",);
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

pub struct Pagination {
  pub num: PageNumer,
  pub size: ItemsPerPage,
}

impl Pagination {
  pub fn new(num: u32, size: u32) -> Self {
    Pagination {
      num: PageNumer::new(num),
      size: ItemsPerPage::new(size),
    }
  }

  pub fn from_int(num: i32, size: i32) -> Self {
    Pagination {
      num: PageNumer::from_int(num),
      size: ItemsPerPage::from_int(size),
    }
  }

  pub fn from_int_option(num: Option<i32>, size: Option<i32>) -> Self {
    let num = num.unwrap_or(1);
    let size = size.unwrap_or(DEFAULT_ITEMS_PER_PAGE as i32);
    Pagination {
      num: PageNumer::from_int(num),
      size: ItemsPerPage::from_int(size),
    }
  }

  pub fn offset(&self) -> u32 {
    self.size.value() * (self.num.value() - 1)
  }

  pub fn limit(&self) -> u32 {
    self.size.value()
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use proptest::prelude::*;

  // NOTE: i32::MAX occurred overflow error
  const MAX_INT: i32 = 10000;
  const MAX_SIZE_FOR_TEST: i32 = MAX_ITEMS_PER_PAGE as i32;

  proptest! {
    #[test]
    fn pagination_should_create_from_int(num in 1..MAX_INT, size in 1..=MAX_SIZE_FOR_TEST) {
      let pagination = Pagination::from_int(num, size);
      assert_eq!(pagination.num.value(), num as u32);
      assert_eq!(pagination.size.value(), size as u32);
    }

    #[test]
    fn pagination_should_set_1_to_page_num_when_num_is_less_than_1(num in -MAX_INT..=0, size in 1..=MAX_SIZE_FOR_TEST) {
      let pagination = Pagination::from_int(num, size);
      assert_eq!(pagination.num.value(), 1);
      assert_eq!(pagination.size.value(), size as u32);
    }

    #[test]
    fn pagination_should_set_default_to_size_when_size_is_less_than_1(num in 1..MAX_INT, size in -MAX_INT..=0) {
      let pagination = Pagination::from_int(num, size);
      assert_eq!(pagination.num.value(), num as u32);
      assert_eq!(pagination.size.value(), DEFAULT_ITEMS_PER_PAGE);
    }

    #[test]
    fn pagination_should_set_default_to_size_when_size_is_greater_than_100(num in 1..MAX_INT, size in (MAX_SIZE_FOR_TEST + 1)..=MAX_INT) {
      let pagination = Pagination::from_int(num, size);
      assert_eq!(pagination.num.value(), num as u32);
      assert_eq!(pagination.size.value(), DEFAULT_ITEMS_PER_PAGE);
    }

    #[test]
    fn pagination_should_create_from_int_option(num in 1..MAX_INT, size in 1..=MAX_SIZE_FOR_TEST) {
      let pagination = Pagination::from_int_option(Some(num), Some(size));
      assert_eq!(pagination.num.value(), num as u32);
      assert_eq!(pagination.size.value(), size as u32);
    }

    #[test]
    fn pagination_should_set_default_to_num_when_num_is_none(size in 1..=MAX_SIZE_FOR_TEST) {
      let pagination = Pagination::from_int_option(None, Some(size));
      assert_eq!(pagination.num.value(), 1);
      assert_eq!(pagination.size.value(), size as u32);
    }

    #[test]
    fn pagination_should_set_default_to_size_when_size_is_none(num in 1..MAX_INT) {
      let pagination = Pagination::from_int_option(Some(num), None);
      assert_eq!(pagination.num.value(), num as u32);
      assert_eq!(pagination.size.value(), DEFAULT_ITEMS_PER_PAGE);
    }

    #[test]
    fn pagination_should_calculate_offset(num in 1..10000, size in 1..=MAX_SIZE_FOR_TEST) {
      let pagination = Pagination::from_int(num, size);
      assert_eq!(pagination.offset(), (size * (num - 1)) as u32);
    }

    #[test]
    fn pagination_should_calculate_limit(num in 1..10000, size in 1..=MAX_SIZE_FOR_TEST) {
      let pagination = Pagination::from_int(num, size);
      assert_eq!(pagination.limit(), size as u32);
    }
  }
}
