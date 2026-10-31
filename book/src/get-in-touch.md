# Get In Touch

open-dis-rust is maintained by volunteers in their spare time. Everything happens on
[GitHub](https://github.com/crhowell3/open-dis-rust), so whatever you need, start there.
Please use the channel that fits your request. That way it reaches the right place, and
other users can find the answer later.

| I want to... | Go here |
| --- | --- |
| Ask a question or get help using the crate | [GitHub Discussions](https://github.com/crhowell3/open-dis-rust/discussions) |
| Report a bug or a PDU / data type / enumeration that doesn't match the standard | [Bug report](https://github.com/crhowell3/open-dis-rust/issues/new?template=2-bug-report.yml) |
| Suggest a new feature or API change | [Feature request](https://github.com/crhowell3/open-dis-rust/issues/new?template=3-feature-request.yml) |
| Report a mistake in this book or the API docs | [Documentation issue](https://github.com/crhowell3/open-dis-rust/issues/new?template=4-docs-issue.yml) |
| Report a security vulnerability | [Private security advisory](https://github.com/crhowell3/open-dis-rust/security/advisories/new). **Please don't open a public issue for security vulnerabilities.** |
| Contribute code | [Tips for Contributing](guides/contributing.md) |

## Before you open an issue

- **Search first.** Check [open and closed issues](https://github.com/crhowell3/open-dis-rust/issues?q=is%3Aissue) and Discussions.
  Somebody may already have run into the same problem.
- **Check that you are using the latest release.** Run `cargo update -p open-dis-rust` and try again. It is possible
  your issue has already been addressed in a new update.
- **Check the [API documentation](https://docs.rs/open-dis-rust).** Most questions about a specific type or method could be
  answered there.

## Reporting a bug

A good report allows a maintainer to reproduce the problem without guessing. Please include:

- The **open-dis-rust version** (from `Cargo.lock`) and your **Rust version** (`rustc --version`).
- The **PDU type**, **record**, or **enumeration** involved.
- **What you expected vs. what happened**, including the full `DISError` message if there is one.
- The **smallest code that shows the problem**, if you are able to trim it down.

## Encoding and interoperability problems

If a PDU will not decode, or another DIS application cannot read what open-dis-rust sends, the raw bytes are by
far the most useful thing you can attach:

- A **packet capture** (`.pcap` or `.pcapng`) from [Wireshark](https://www.wireshark.org/), trimmed to
  the PDUs in question, or a **hex dump** of the datagram.
- The **other application** involved and its version.
- If you have it, the **section or table of IEEE 1278.1-2012** that you think the crate gets wrong.

Please remove anything sensitive from captures before attaching them, such as exercise data, IP addresses,
or real-world locations. Also, **NEVER** post anything CUI or classified. Doing so is a violation of
federal law and potentially a danger to the security of the United States (or whichever country controls the data).

## Security issues

If you find a vulnerability such as a malformed PDU that crashes a receiver or uses unbounded memory, report it
privately using [GitHub's private vulnerability reporting](https://github.com/crhowell3/open-dis-rust/security/advisories/new).

A maintainer will acknowledge your report within 5 days. We will work with you on a fix and agree on a disclosure date before
anything is made public. See [`SECURITY.md`](https://github.com/crhowell3/open-dis-rust/blob/main/SECURITY.md) for which
versions receive security fixes.

## What to expect

This is a volunteer project, so response times vary. Most issues get a first reply within a week.
If you post an issue and don't get a response within a couple weeks, a polite follow-up comment is
welcome. Pull requests that include tests, and bug reports with a capture attached, are usually handled fastest.

Everyone taking part is expected to follow the project's [Code of Conduct](https://github.com/crhowell3/open-dis-rust/blob/main/CODE_OF_CONDUCT.md).

## Maintainers

- **Cameron Howell** ([@crhowell3](https://github.com/crhowell3)), creator and lead maintainer

## Supporting the project

Contributing fixes, documentation, and packet captures from real exercises helps make this crate better.
