.PHONY: help
help: ### Shows this help
	@grep -E '^[0-9a-zA-Z_-]+[.0-9a-zA-Z_-]*:' $(MAKEFILE_LIST) \
	| awk 'BEGIN {FS = ":.*?### "}; {printf "\033[32;1m%-16s\033[0m %s\n", $$1, $$2}'
