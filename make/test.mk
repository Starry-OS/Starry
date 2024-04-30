# Test scripts

test_app :=
ifneq ($(filter command line,$(origin A) $(origin APP)),)
  test_app := $(APP)
endif

define app_test
  $(CURDIR)/scripts/test/app_test.sh $(test_app)
endef

define app_test_for_monolithic
  $(CURDIR)/scripts/test/app_test_for_monolithic.sh $(test_app)
endef
