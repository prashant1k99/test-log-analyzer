# Design Doc

### RuleSets:
- Must use buffered IO
- Must not read the full file into memory
- Avoid allocating new Strings per log line where possible
- Prefer zero-copy parsing using string slices
- The solution must scale with increasing file size

### TODO:
- [ ] Take file path as input
- [ ] Read the file line by line
- [ ] It should load the configs from `config.toml`
- [ ] Config should contain delimiter, log levels, target for log summary and log format position
- [ ] It should have ability to process logs in parallel using threads
