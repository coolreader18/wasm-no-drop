import("../crate/pkg").then(module => {
  module.test_no_panic();
  module.test_with_panic();
});
