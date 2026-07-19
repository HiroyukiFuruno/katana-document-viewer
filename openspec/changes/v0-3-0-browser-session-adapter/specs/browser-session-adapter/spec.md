## ADDED Requirements

### Requirement: KDV forwards an unmodified browser document to KRR

KDV SHALL create a KRR browser session from raw HTML, complete document URL origin, viewport, and explicit launch selection without parsing or rewriting HTML.

#### Scenario: relative browser navigation uses the supplied document origin

- **WHEN** the request supplies HTML with a document URL origin and the browser produces a relative navigation
- **THEN** KDV SHALL return KRR's resolved navigation update without computing the URL itself

### Requirement: KDV owns only browser session adaptation

KDV SHALL forward browser commands and updates while leaving HTML/CSS/JavaScript evaluation, hit testing, and resource policy to KRR.

#### Scenario: host input reaches the KRR session

- **WHEN** the host sends text or pointer input through the adapter
- **THEN** KDV SHALL dispatch that input to the KRR session and return its resulting frame or navigation update

### Requirement: adapter updates preserve observable browser events

KDV SHALL coalesce superseded frames while preserving navigation and error updates in order.

#### Scenario: navigation is not dropped behind a newer frame

- **WHEN** KRR emits a navigation and later emits another frame
- **THEN** KDV SHALL return the navigation before the coalesced latest frame

### Requirement: explicit process config supports hermetic testing

KDV SHALL pass a caller-provided `HtmlBrowserProcessConfig` to KRR unchanged and SHALL NOT discover a browser executable through environment-dependent fallback.

#### Scenario: a command error reaches the host

- **WHEN** KRR reports a process or protocol error while dispatching resize, navigation, or refresh
- **THEN** KDV SHALL emit a typed error update and keep the adapter closeable
