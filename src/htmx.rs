use phf::phf_map;

pub static ATTRIBUTES: &[&str] = &[
"hx-get",
    "hx-post",
    "hx-on*",
    "hx-push-url",
    "hx-select",
    "hx-select-oob",
    "hx-swap",
    "hx-swap-oob",
    "hx-target",
    "hx-trigger",
    "hx-vals",
    "hx-boost",
    "hx-confirm",
    "hx-delete",
    "hx-disable",
    "hx-disabled-elt",
    "hx-disinherit",
    "hx-encoding",
    "hx-ext",
    "hx-headers",
    "hx-history",
    "hx-history-elt",
    "hx-include",
    "hx-indicator",
    "hx-inherit",
    "hx-params",
    "hx-patch",
    "hx-preserve",
    "hx-prompt",
    "hx-put",
    "hx-replace-url",
    "hx-request",
    "hx-sync",
    "hx-validate",
    "hx-vars",
    ];

pub static DESCRIPTIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "hx-get" =>
r###"issues a GET to the specified URL
description = """\
  The hx-get attribute in htmx will cause an element to issue a GET request to the specified URL and swap the returned \
  HTML into the DOM using a swap strategy."""

The `hx-get` attribute will cause an element to issue a `GET` to the specified URL and swap
the HTML into the DOM using a swap strategy:

```html
  <button hx-get="/example">Get Some HTML</button>
```

This example will cause the `button` to issue a `GET` to `/example` and swap the returned HTML into
 the `innerHTML` of the `button`.

### Notes

* `hx-get` is not inherited
* By default `hx-get` usually does not include any parameters.  You can use the [hx-params](@/attributes/hx-params.md)
  attribute to change this
    * NB: If the element with the `hx-get` attribute also has a value, this will be included as a parameter unless explicitly removed
* You can control the target of the swap using the [hx-target](@/attributes/hx-target.md) attribute
* You can control the swap strategy by using the [hx-swap](@/attributes/hx-swap.md) attribute
* You can control what event triggers the request with the [hx-trigger](@/attributes/hx-trigger.md) attribute
* You can control the data submitted with the request in various ways, documented here: [Parameters](@/docs.md#parameters)
* An empty `hx-get:""` will make a get request to the current url and will swap the current HTML page "###,
    "hx-post" =>
r###"issues a POST to the specified URL
description = """\
  The hx-post attribute in htmx will cause an element to issue a POST request to the specified URL and swap the \
  returned HTML into the DOM using a swap strategy."""

The `hx-post` attribute will cause an element to issue a `POST` to the specified URL and swap
the HTML into the DOM using a swap strategy:

```html
<button hx-post="/account/enable" hx-target="body">
  Enable Your Account
</button>
```

This example will cause the `button` to issue a `POST` to `/account/enable` and swap the returned HTML into
 the `innerHTML` of the `body`.
 
## Notes

* `hx-post` is not inherited
* You can control the target of the swap using the [hx-target](@/attributes/hx-target.md) attribute
* You can control the swap strategy by using the [hx-swap](@/attributes/hx-swap.md) attribute
* You can control what event triggers the request with the [hx-trigger](@/attributes/hx-trigger.md) attribute
* You can control the data submitted with the request in various ways, documented here: [Parameters](@/docs.md#parameters)"###,
    "hx-on*" =>
r###"handle events with inline scripts on elements
404: Not Found"###,
    "hx-push-url" =>
r###"push a URL into the browser location bar to create history
description = """\
  The hx-push-url attribute in htmx allows you to push a URL into the browser location history. This creates a new \
  history entry, allowing navigation with the browser's back and forward buttons."""

The `hx-push-url` attribute allows you to push a URL into the browser [location history](https://developer.mozilla.org/en-US/docs/Web/API/History_API).
This creates a new history entry, allowing navigation with the browser’s back and forward buttons.
htmx snapshots the current DOM and saves it into its history cache, and restores from this cache on navigation.

The possible values of this attribute are:

1. `true`, which pushes the fetched URL into history.
2. `false`, which disables pushing the fetched URL if it would otherwise be pushed due to inheritance or [`hx-boost`](/attributes/hx-boost).
3. A URL to be pushed into the location bar.
   This may be relative or absolute, as per [`history.pushState()`](https://developer.mozilla.org/en-US/docs/Web/API/History/pushState).

Here is an example:

```html
<div hx-get="/account" hx-push-url="true">
  Go to My Account
</div>
```

This will cause htmx to snapshot the current DOM to `localStorage` and push the URL `/account' into the browser location bar.

Another example:

```html
<div hx-get="/account" hx-push-url="/account/home">
  Go to My Account
</div>
```

This will push the URL `/account/home' into the location history.

## Notes

* `hx-push-url` is inherited and can be placed on a parent element
* The [`HX-Push-Url` response header](@/headers/hx-push-url.md) has similar behavior and can override this attribute.
* The [`hx-history-elt` attribute](@/attributes/hx-history-elt.md) allows changing which element is saved in the history cache."###,
    "hx-select" =>
r###"select content to swap in from a response
description = "The hx-select attribute in htmx allows you to select the content you want swapped from a response."

The `hx-select` attribute allows you to select the content you want swapped from a response.  The value of
this attribute is a CSS query selector of the element or elements to select from the response.

Here is an example that selects a subset of the response content:

```html
<div>
    <button hx-get="/info" hx-select="#info-detail" hx-swap="outerHTML">
        Get Info!
    </button>
</div>
```

So this button will issue a `GET` to `/info` and then select the element with the id `info-detail`,
which will replace the entire button in the DOM.

## Notes

* `hx-select` is inherited and can be placed on a parent element"###,
    "hx-select-oob" =>
r###"select content to swap in from a response, somewhere other than the target (out of band)
description = """\
  The hx-select-oob attribute in htmx allows you to select content from a response to be swapped in via an out-of-band \
  swap. The value of this attribute is comma separated list of elements to be swapped out of band."""

The `hx-select-oob` attribute allows you to select content from a response to be swapped in via an out-of-band swap.  
The value of this attribute is comma separated list of elements to be swapped out of band.  This attribute is almost
always paired with [hx-select](@/attributes/hx-select.md).

Here is an example that selects a subset of the response content:

```html
<div>
   <div id="alert"></div>
    <button hx-get="/info" 
            hx-select="#info-details" 
            hx-swap="outerHTML"
            hx-select-oob="#alert">
        Get Info!
    </button>
</div>
```

This button will issue a `GET` to `/info` and then select the element with the id `info-details`,
which will replace the entire button in the DOM, and, in addition, pick out an element with the id `alert` 
in the response and swap it in for div in the DOM with the same ID.

Each value in the comma separated list of values can specify any valid [`hx-swap`](@/attributes/hx-swap.md)
strategy by separating the selector and the swap strategy with a `:`, with the strategy otherwise defaulting to `outerHTML`.

For example, to prepend the alert content instead of replacing it:

```html
<div>
   <div id="alert"></div>
    <button hx-get="/info"
            hx-select="#info-details"
            hx-swap="outerHTML"
            hx-select-oob="#alert:afterbegin">
        Get Info!
    </button>
</div>
```

## Notes

* `hx-select-oob` is inherited and can be placed on a parent element"###,
    "hx-swap" =>
r###"controls how content will swap in (outerHTML, beforeend, afterend, …)
description = """\
  The hx-swap attribute in htmx allows you to specify the 'swap strategy', or how the response will be swapped in \
  relative to the target of an AJAX request. The default swap strategy is `innerHTML`."""

The `hx-swap` attribute allows you to specify how the response will be swapped in relative to the
[target](@/attributes/hx-target.md) of an AJAX request. If you do not specify the option, the default is
`htmx.config.defaultSwapStyle` (`innerHTML`).

The possible values of this attribute are:

* `innerHTML` - Replace the inner html of the target element
* `outerHTML` - Replace the entire target element with the response
* `textContent` - Replace the text content of the target element, without parsing the response as HTML
* `beforebegin` - Insert the response before the target element
* `afterbegin` - Insert the response before the first child of the target element
* `beforeend` - Insert the response after the last child of the target element
* `afterend` - Insert the response after the target element
* `delete` - Deletes the target element regardless of the response
* `none`- Does not append content from response (out of band items will still be processed).

These options are based on standard DOM naming and the 
[`Element.insertAdjacentHTML`](https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentHTML)
specification.

So in this code:

```html
  <div hx-get="/example" hx-swap="afterend">Get Some HTML & Append It</div>
```

The `div` will issue a request to `/example` and append the returned content after the `div`

### Modifiers

The `hx-swap` attributes supports modifiers for changing the behavior of the swap.  They are outlined below.

#### Transition: `transition`

If you want to use the new [View Transitions](https://developer.mozilla.org/en-US/docs/Web/API/View_Transitions_API) API
when a swap occurs, you can use the `transition:true` option for your swap.  You can also enable this feature globally by
setting the `htmx.config.globalViewTransitions` config setting to `true`.

#### Timing: `swap` & `settle`

You can modify the amount of time that htmx will wait after receiving a response to swap the content
by including a `swap` modifier:

```html
  <!-- this will wait 1s before doing the swap after it is received -->
  <div hx-get="/example" hx-swap="innerHTML swap:1s">Get Some HTML & Append It</div>
```

Similarly, you can modify the time between the swap and the settle logic by including a `settle`
modifier:

```html
  <!-- this will wait 1s before doing the swap after it is received -->
  <div hx-get="/example" hx-swap="innerHTML settle:1s">Get Some HTML & Append It</div>
```

These attributes can be used to synchronize htmx with the timing of CSS transition effects.

#### Title: `ignoreTitle`

By default, htmx will update the title of the page if it finds a `<title>` tag in the response content.  You can turn
off this behavior by setting the `ignoreTitle` option to true.

#### Scrolling: `scroll` & `show`

You can also change the scrolling behavior of the target element by using the `scroll` and `show` modifiers, both
of which take the values `top` and `bottom`:

```html
  <!-- this fixed-height div will scroll to the bottom of the div after content is appended -->
  <div style="height:200px; overflow: scroll" 
       hx-get="/example" 
       hx-swap="beforeend scroll:bottom">
     Get Some HTML & Append It & Scroll To Bottom
  </div>
```

```html
  <!-- this will get some content and add it to #another-div, then ensure that the top of #another-div is visible in the 
       viewport -->
  <div hx-get="/example" 
       hx-swap="innerHTML show:top"
       hx-target="#another-div">
    Get Some Content
  </div>
```

If you wish to target a different element for scrolling or showing, you may place a CSS selector after the `scroll:`
or `show:`, followed by `:top` or `:bottom`:

```html
  <!-- this will get some content and swap it into the current div, then ensure that the top of #another-div is visible in the 
       viewport -->
  <div hx-get="/example" 
       hx-swap="innerHTML show:#another-div:top">
    Get Some Content
  </div>
```

You may also use `window:top` and `window:bottom` to scroll to the top and bottom of the current window.


```html
  <!-- this will get some content and swap it into the current div, then ensure that the viewport is scrolled to the
       very top -->
  <div hx-get="/example" 
       hx-swap="innerHTML show:window:top">
    Get Some Content
  </div>
```

For boosted links and forms the default behaviour is `show:top`. You can disable it globally with
[htmx.config.scrollIntoViewOnBoost](@/api.md#config) or you can use `hx-swap="show:none"` on an element basis.

```html
<form action="/example" hx-swap="show:none">
  ...
</form>
```

#### Focus scroll

htmx preserves focus between requests for inputs that have a defined id attribute. By default htmx prevents auto-scrolling to focused inputs between requests which can be unwanted behavior on longer requests when the user has already scrolled away. To enable focus scroll you can use `focus-scroll:true`.

```html
  <input id="name" hx-get="/validation" 
       hx-swap="outerHTML focus-scroll:true"/>
```

Alternatively, if you want the page to automatically scroll to the focused element after each request you can change the htmx global configuration value `htmx.config.defaultFocusScroll` to true. Then disable it for specific requests using `focus-scroll:false`.

```html
  <input id="name" hx-get="/validation" 
       hx-swap="outerHTML focus-scroll:false"/>
```

## Notes

* `hx-swap` is inherited and can be placed on a parent element
* The default value of this attribute is `innerHTML`
* Due to DOM limitations, it’s not possible to use the `outerHTML` method on the `<body>` element.
  htmx will change `outerHTML` on `<body>` to use `innerHTML`.
* The default swap delay is 0ms
* The default settle delay is 20ms"###,
    "hx-swap-oob" =>
r###"mark element to swap in from a response (out of band)
description = """\
  The hx-swap-oob attribute in htmx allows you to specify that some content in a response should be swapped into the \
  DOM somewhere other than the target, that is 'out-of-band'. This allows you to piggyback updates to other elements \
  on a response."""

The `hx-swap-oob` attribute allows you to specify that some content in a response should be
swapped into the DOM somewhere other than the target, that is "Out of Band".  This allows you to piggyback updates to other element updates on a response.

Consider the following response HTML:

```html
<div>
 ...
</div>
<div id="alerts" hx-swap-oob="true">
    Saved!
</div>

```

The first div will be swapped into the target the usual manner.  The second div, however, will be swapped in as a replacement for the element with the id `alerts`, and will not end up in the target.

The value of the `hx-swap-oob` can be:

* `true`
* any valid [`hx-swap`](@/attributes/hx-swap.md) value
* any valid [`hx-swap`](@/attributes/hx-swap.md) value, followed by a colon, followed by a CSS selector

If the value is `true` or `outerHTML` (which are equivalent) the element will be swapped inline.

If a swap value is given, that swap strategy will be used and the encapsulating tag pair will be stripped for all strategies other than `outerHTML`.

If a selector is given, all elements matched by that selector will be swapped.  If not, the element with an ID matching the new content will be swapped.

### Using alternate swap strategies

As mentioned previously when using swap strategies other than `true` or `outerHTML` the encapsulating tags are stripped, as such you need to excapsulate the returned data with the correct tags for the context.

When trying to insert a `<tr>` in a table that uses `<tbody>`:
```html
<tbody hx-swap-oob="beforeend:#table tbody">
	<tr>
		...
	</tr>
</tbody>
```

A "plain" table:
```html
<table hx-swap-oob="beforeend:#table2">
	<tr>
		...
	</tr>
</table>
```

A `<li>` may be encapsulated in `<ul>`, `<ol>`, `<div>` or `<span>`, for example:
```html
<ul hx-swap-oob="beforeend:#list1">
	<li>...</li>
</ul>
```

A `<p>` can be encapsulated in `<div>` or `<span>`:
```html
<span hx-swap-oob="beforeend:#text">
	<p>...</p>
</span>
```

### Troublesome Tables and lists

Note that you can use a `template` tag to encapsulate types of elements that, by the HTML spec, can't stand on their own in the
DOM (`<tr>`, `<td>`, `<th>`, `<thead>`, `<tbody>`, `<tfoot>`, `<colgroup>`, `<caption>`, `<col>` & `<li>`).

Here is an example with an out-of-band swap of a table row being encapsulated in this way:

```html
<div>
    ...
</div>
<template>
    <tr id="row" hx-swap-oob="true">
        ...
    </tr>
</template>
```

Note that these template tags will be removed from the final content of the page.

### Slippery SVGs

Some element types, like SVG, use a specific XML namespace for their child elements. This prevents internal elements from working correctly when swapped in, unless they are encapsulated within a `svg` tag. To modify the internal contents of an existing SVG, you can use both `template` and `svg` tags to encapsulate the elements, allowing them to get the correct namespace applied.

Here is an example with an out-of-band swap of svg elements being encapsulated in this way:

```html
<div>
    ...
</div>
<template><svg>
    <circle hx-swap-oob="true" id="circle1" r="35" cx="50" cy="50" fill="red" /> 
</svg></template>
<template><svg hx-swap-oob="beforebegin:#circle1">
    <circle id="circle2" r="45" cx="50" cy="50" fill="blue" /> 
</svg></template>
```
This will replace circle1 inline and then insert circle2 before circle1. 

Note that these `template` and `svg` wrapping tags will be removed from the final content of the page.

## Nested OOB Swaps

By default, any element with `hx-swap-oob=` attribute anywhere in the response is processed for oob swap behavior, including when an element is nested within the main response element.
This can be problematic when using [template fragments](https://htmx.org/essays/template-fragments/) where a fragment may be reused as an oob-swap target and also as part of a bigger fragment. When the bigger fragment is the main response the inner fragment will still be processed as an oob swap, removing it from the dom.

This behavior can be changed by setting the config `htmx.config.allowNestedOobSwaps` to `false`. If this config option is `false`, OOB swaps are only processed when the element is *adjacent to* the main response element, OOB swaps elsewhere will be ignored and oob-swap-related attributes stripped.

## Notes

* `hx-swap-oob` is not inherited"###,
    "hx-target" =>
r###"specifies the target element to be swapped
description = """\
  The hx-target attribute in htmx allows you to target a different element for swapping than the one issuing the AJAX \
  request."""

The `hx-target` attribute allows you to target a different element for swapping than the one issuing the AJAX
request.  The value of this attribute can be:

* A CSS query selector of the element to target.
* `this` which indicates that the element that the `hx-target` attribute is on is the target.
* `closest <CSS selector>` which will find the [closest](https://developer.mozilla.org/docs/Web/API/Element/closest)
  ancestor element or itself, that matches the given CSS selector
  (e.g. `closest tr` will target the closest table row to the element).
* `find <CSS selector>` which will find the first child descendant element that matches the given CSS selector.
* `next` which resolves to [element.nextElementSibling](https://developer.mozilla.org/docs/Web/API/Element/nextElementSibling)
* `next <CSS selector>` which will scan the DOM forward for the first element that matches the given CSS selector.
  (e.g. `next .error` will target the closest following sibling element with `error` class)
* `previous` which resolves to [element.previousElementSibling](https://developer.mozilla.org/docs/Web/API/Element/previousElementSibling)
* `previous <CSS selector>` which will scan the DOM backwards for the first element that matches the given CSS selector.
  (e.g. `previous .error` will target the closest previous sibling with `error` class)


Here is an example that targets a div:

```html
<div>
    <div id="response-div"></div>
    <button hx-post="/register" hx-target="#response-div" hx-swap="beforeend">
        Register!
    </button>
</div>
```

The response from the `/register` url will be appended to the `div` with the id `response-div`.

This example uses `hx-target="this"` to make a link that updates itself when clicked:
```html
<a hx-post="/new-link" hx-target="this" hx-swap="outerHTML">New link</a>
```

## Notes

* `hx-target` is inherited and can be placed on a parent element"###,
    "hx-trigger" =>
r###"specifies the event that triggers the request
description = """\
  The hx-trigger attribute in htmx allows you to specify what triggers an AJAX request. Supported triggers include \
  standard DOM events, custom events, polling intervals, and event modifiers. The hx-trigger attribute also allows \
  specifying event filtering, timing controls, event bubbling, and multiple trigger definitions for fine-grained \
  control over when and how requests are initiated."""

The `hx-trigger` attribute allows you to specify what triggers an AJAX request.  A trigger
value can be one of the following:

* An event name (e.g. "click" or "my-custom-event") followed by an event filter and a set of event modifiers
* A polling definition of the form `every <timing declaration>`
* A comma-separated list of such events

### Standard Events

Standard events refer to [web API events](https://developer.mozilla.org/en-US/docs/Web/API/Element#events) (e.g. click, keydown, mouseup, load).

A standard event, such as `click` can be specified as the trigger like so:

```html
<div hx-get="/clicked" hx-trigger="click">Click Me</div>
```

#### Standard Event Filters

Events can be filtered by enclosing a boolean javascript expression in square brackets after the event name.  If
this expression evaluates to `true` the event will be triggered, otherwise it will be ignored. Standard event filters [require eval](@/docs.md#configuration-options).

```html
<div hx-get="/clicked" hx-trigger="click[ctrlKey]">Control Click Me</div>
```

This event will trigger if a click event is triggered with the `event.ctrlKey` property set to true.

Conditions can also refer to global functions or state

```html
<div hx-get="/clicked" hx-trigger="click[checkGlobalState()]">Control Click Me</div>
```

And can also be combined using the standard javascript syntax

```html
<div hx-get="/clicked" hx-trigger="click[ctrlKey&&shiftKey]">Control-Shift Click Me</div>
```

Note that all symbols used in the expression will be resolved first against the triggering event, and then next
against the global namespace, so `myEvent[foo]` will first look for a property named `foo` on the event, then look
for a global symbol with the name `foo`

#### Standard Event Modifiers

Standard events can also have modifiers that change how they behave.  The modifiers are:

* `once` - the event will only trigger once (e.g. the first click)
* `changed` - the event will only fire if the value of the element has changed. Please pay attention `change` is the name of the event and `changed` is the name of the modifier.
* `delay:<timing declaration>` - a delay will occur before an event triggers a request.  If the event
is seen again it will reset the delay.
* `throttle:<timing declaration>` - a throttle will occur after an event triggers a request. If the event
is seen again before the delay completes, it is ignored, the element will trigger at the end of the delay.
* `from:<Extended CSS selector>` - allows the event that triggers a request to come from another element in the document (e.g. listening to a key event on the body, to support hot keys)
  * A standard CSS selector resolves to all elements matching that selector. Thus, `from:input` would listen on every input on the page.
  * The CSS selector is only evaluated once and is not re-evaluated when the page changes. If you need to detect dynamically added elements use a [standard event filter](#standard-event-filters), for example `hx-trigger="click[event.target.matches('button')] from:body"` which would [catch](https://developer.mozilla.org/en-US/docs/Learn/JavaScript/Building_blocks/Event_bubbling) click events from every button on the page.
  * The extended CSS selector here allows for the following non-standard CSS values:
    * `document` - listen for events on the document
    * `window` - listen for events on the window
    * `closest <CSS selector>` - finds the [closest](https://developer.mozilla.org/docs/Web/API/Element/closest) ancestor element or itself, matching the given css selector
    * `find <CSS selector>` - finds the closest child matching the given css selector
    * `next` resolves to [element.nextElementSibling](https://developer.mozilla.org/docs/Web/API/Element/nextElementSibling)
    * `next <CSS selector>` scans the DOM forward for the first element that matches the given CSS selector.
      (e.g. `next .error` will target the closest following sibling element with `error` class)
    * `previous` resolves to [element.previousElementSibling](https://developer.mozilla.org/docs/Web/API/Element/previousElementSibling)
    * `previous <CSS selector>` scans the DOM backwards for the first element that matches the given CSS selector.
      (e.g. `previous .error` will target the closest previous sibling with `error` class)
* `target:<CSS selector>` - allows you to filter via a CSS selector on the target of the event.  This can be useful when you want to listen for
triggers from elements that might not be in the DOM at the point of initialization, by, for example, listening on the body,
but with a target filter for a child element
* `consume` - if this option is included the event will not trigger any other htmx requests on parents (or on elements
  listening on parents)
* `queue:<queue option>` - determines how events are queued if an event occurs while a request for another event is in flight.  Options are:
  * `first` - queue the first event
  * `last` - queue the last event (default)
  * `all` - queue all events (issue a request for each event)
  * `none` - do not queue new events

Here is an example of a search box that searches on `input`, but only if the search value has changed
and the user hasn't typed anything new for 1 second:

```html
<input name="q"
       hx-get="/search" hx-trigger="input changed delay:1s"
       hx-target="#search-results"/>
```

The response from the `/search` url will be appended to the `div` with the id `search-results`.

### Non-standard Events

There are some additional non-standard events that htmx supports:

* `load` - triggered on load (useful for lazy-loading something)
* `revealed` - triggered when an element is scrolled into the viewport (also useful for lazy-loading). If you are using `overflow` in css like `overflow-y: scroll` you should use `intersect once` instead of `revealed`.
* `intersect` - fires once when an element first intersects the viewport.  This supports two additional options:
    * `root:<selector>` - a CSS selector of the root element for intersection
    * `threshold:<float>` - a floating point number between 0.0 and 1.0, indicating what amount of intersection to fire the event on

### Triggering via the `HX-Trigger` header

If you're trying to fire an event from <code>HX-Trigger</code> response  header, you will likely want to
use the `from:body` modifier.  E.g. if you send a header like this <code>HX-Trigger: my-custom-event</code>
with a response, an element would likely need to look like this:

```html
  <div hx-get="/example" hx-trigger="my-custom-event from:body">
    Triggered by HX-Trigger header...
  </div>
```

in order to fire.

This is because the header will likely trigger the event in a different DOM hierarchy than the element that you
wish to be triggered.  For a similar reason, you will often listen for hot keys from the body.

### Polling

By using the syntax `every <timing declaration>` you can have an element poll periodically:

```html
<div hx-get="/latest_updates" hx-trigger="every 1s">
  Nothing Yet!
</div>
```

This example will issue a `GET` to the `/latest_updates` URL every second and swap the results into
the innerHTML of this div.

If you want to add a filter to polling, it should be added *after* the poll declaration:

```html
<div hx-get="/latest_updates" hx-trigger="every 1s [someConditional]">
  Nothing Yet!
</div>
```

### Multiple Triggers

Multiple triggers can be provided, separated by commas.  Each trigger gets its own options.
```html
  <div hx-get="/news" hx-trigger="load, click delay:1s"></div>
```
This example will load `/news` immediately on page load, and then again with a delay of one second after each click.

### Via JavaScript

The AJAX request can be triggered via JavaScript [`htmx.trigger()`](@/api.md#trigger), too.

## Notes

* `hx-trigger` is not inherited
* `hx-trigger` can be used without an AJAX request, in which case it will only fire the `htmx:trigger` event
* In order to pass a CSS selector that contains whitespace (e.g. `form input`) to the `from`- or `target`-modifier, surround the selector in parentheses or curly brackets (e.g. `from:(form input)` or `from:closest (form input)`)
* A reset event in hx-trigger (e.g. hx-trigger="change, reset") might not work as intended, since HTMX builds its values and sends a request before the browser resets the form values. As a workaround, add a delay to let the browser reset the form before making the request (e.g. hx-trigger="change, reset delay:0.01s"). "###,
    "hx-vals" =>
r###"add values to submit with the request (JSON format)
description = """\
  The hx-vals attribute in htmx allows you to add to the parameters that will be submitted with an AJAX request."""

The `hx-vals` attribute allows you to add to the parameters that will be submitted with an AJAX request.

By default, the value of this attribute is a list of name-expression values in [JSON (JavaScript Object Notation)](https://www.json.org/json-en.html)
format.

If you wish for `hx-vals` to *evaluate* the values given, you can prefix the values with `javascript:` or `js:`.

```html
  <div hx-get="/example" hx-vals='{"myVal": "My Value"}'>Get Some HTML, Including A Value in the Request</div>

  <div hx-get="/example" hx-vals='js:{myVal: calculateValue()}'>Get Some HTML, Including a Dynamic Value from Javascript in the Request</div>
```

When using evaluated code you can access the `event` object. This example includes the value of the last typed key within the input.

```html
  <div hx-get="/example" hx-trigger="keyup" hx-vals='js:{lastKey: event.key}'>
    <input type="text" />
  </div>
```

You can also use the spread operator to dynamically specify values. This allows you to include all properties from an object returned by a function:

```html
  <div hx-get="/example" hx-vals='js:{...foo()}'>Get Some HTML, Including All Values from foo() in the Request</div>
```

In this example, if `foo()` returns an object like `{name: "John", age: 30}`, both `name` and `age` will be included as parameters in the request.

## Security Considerations

* By default, the value of `hx-vals` must be valid [JSON](https://developer.mozilla.org/en-US/docs/Glossary/JSON).
  It is **not** dynamically computed.  If you use the `javascript:` prefix, be aware that you are introducing
  security considerations, especially when dealing with user input such as query strings or user-generated content,
  which could introduce a [Cross-Site Scripting (XSS)](https://owasp.org/www-community/attacks/xss/) vulnerability.

## Notes

* `hx-vals` is inherited and can be placed on a parent element.
* A child declaration of a variable overrides a parent declaration.
* Input values with the same name will be overridden by variable declarations."###,
    "hx-boost" =>
r###"add progressive enhancement for links and forms
description = """\
  The hx-boost attribute in htmx enables progressive enhancement by converting standard HTML anchors and forms into \
  AJAX requests, maintaining graceful fallback for users without JavaScript while providing modern dynamic page \
  updates for those with JavaScript enabled."""

The `hx-boost` attribute allows you to "boost" normal anchors and form tags to use AJAX instead.  This
has the [nice fallback](https://en.wikipedia.org/wiki/Progressive_enhancement) that, if the user does not 
have javascript enabled, the site will continue to work.

For anchor tags, clicking on the anchor will issue a `GET` request to the url specified in the `href` and
will push the url so that a history entry is created.  The target is the `<body>` tag, and the `innerHTML`
swap strategy is used by default.  All of these can be modified by using the appropriate attributes, except
the `click` trigger.

For forms the request will be converted into a `GET` or `POST`, based on the method in the `method` attribute
and will be triggered by a `submit`.  Again, the target will be the `body` of the page, and the `innerHTML`
swap will be used. The url will _not_ be pushed, however, and no history entry will be created. (You can use the 
[hx-push-url](@/attributes/hx-push-url.md) attribute if you want the url to be pushed.)

Here is an example of some boosted links:

```html
<div hx-boost="true">
  <a href="/page1">Go To Page 1</a>
  <a href="/page2">Go To Page 2</a>
</div>
```
These links will issue an ajax `GET` request to the respective URLs and replace the body's inner content with it.

Here is an example of a boosted form:

```html
<form hx-boost="true" action="/example" method="post">
    <input name="email" type="email" placeholder="Enter email...">
    <button>Submit</button>
</form>
```
This form will issue an ajax `POST` to the given URL and replace the body's inner content with it.


## Notes

* `hx-boost` is inherited and can be placed on a parent element
* Only links that are to the same domain and that are not local anchors will be boosted
* All requests are done via AJAX, so keep that in mind when doing things like redirects
* To find out if the request results from a boosted anchor or form, look for [`HX-Boosted`](@/reference.md#request_headers) in the request header
* Selectively disable boost on child elements with `hx-boost="false"`
* Disable the replacement of elements via boost, and their children, with [`hx-preserve="true"`](@/attributes/hx-preserve.md)"###,
    "hx-confirm" =>
r###"shows a confirm() dialog before issuing a request
description = """\
  The hx-confirm attribute in htmx provides a way to add confirmation dialogs before executing requests, allowing \
  you to protect users from accidental destructive actions. This documentation explains how to implement confirmation \
  prompts and customize their behavior through event handling."""

The `hx-confirm` attribute allows you to confirm an action before issuing a request.  This can be useful
in cases where the action is destructive and you want to ensure that the user really wants to do it.

Here is an example:

```html
<button hx-delete="/account" hx-confirm="Are you sure you wish to delete your account?">
  Delete My Account
</button>
```

## Event details

The event triggered by `hx-confirm` contains additional properties in its `detail`:

* triggeringEvent: the event that triggered the original request
* issueRequest(skipConfirmation=false): a callback which can be used to confirm the AJAX request
* question: the value of the `hx-confirm` attribute on the HTML element

## Notes

* `hx-confirm` is inherited and can be placed on a parent element
* `hx-confirm` uses the browser's `window.confirm` by default. You can customize this behavior as shown [in this example](@/examples/confirm.md).
* a boolean `skipConfirmation` can be passed to the `issueRequest` callback; if true (defaults to false), the `window.confirm` will not be called and the AJAX request is issued directly"###,
    "hx-delete" =>
r###"issues a DELETE to the specified URL
description = """\
  The hx-delete attribute in htmx will cause an element to issue a DELETE request to the specified URL and swap the \
  returned HTML into the DOM using a swap strategy."""

The `hx-delete` attribute will cause an element to issue a `DELETE` to the specified URL and swap
the HTML into the DOM using a swap strategy:

```html
<button hx-delete="/account" hx-target="body">
  Delete Your Account
</button>
```

This example will cause the `button` to issue a `DELETE` to `/account` and swap the returned HTML into
 the `innerHTML` of the `body`.

## Notes

* `hx-delete` is not inherited
* You can control the target of the swap using the [hx-target](@/attributes/hx-target.md) attribute
* You can control the swap strategy by using the [hx-swap](@/attributes/hx-swap.md) attribute
* You can control what event triggers the request with the [hx-trigger](@/attributes/hx-trigger.md) attribute
* You can control the data submitted with the request in various ways, documented here: [Parameters](@/docs.md#parameters)
* To remove the element following a successful `DELETE`, return a `200` status code with an empty body; if the server responds with a `204`, no swap takes place, documented here: [Requests & Responses](@/docs.md#requests)"###,
    "hx-disable" =>
r###"disables htmx processing for the given node and any children nodes
description = "The hx-disable attribute in htmx will disable htmx processing for a given element and all its children."

The `hx-disable` attribute will disable htmx processing for a given element and all its children.  This can be 
useful as a backup for HTML escaping, when you include user generated content in your site, and you want to 
prevent malicious scripting attacks.

The value of the tag is ignored, and it cannot be reversed by any content beneath it.
 
## Notes

* `hx-disable` is inherited"###,
    "hx-disabled-elt" =>
r###"adds the disabled attribute to the specified elements while a request is in flight
description = """\
  The hx-disabled-elt attribute in htmx allows you to specify elements that will have the `disabled` attribute added \
  to them for the duration of the request."""

The `hx-disabled-elt` attribute allows you to specify elements that will have the `disabled` attribute
added to them for the duration of the request. The value of this attribute can be:

* A CSS query selector of the element to disable.
* `this` to disable the element itself
* `closest <CSS selector>` which will find the [closest](https://developer.mozilla.org/docs/Web/API/Element/closest)
  ancestor element or itself, that matches the given CSS selector
  (e.g. `closest fieldset` will disable the closest to the element `fieldset`).
* `find <CSS selector>` which will find the first child descendant element that matches the given CSS selector
* `next` which resolves to [element.nextElementSibling](https://developer.mozilla.org/docs/Web/API/Element/nextElementSibling)
* `next <CSS selector>` which will scan the DOM forward for the first element that matches the given CSS selector
  (e.g. `next button` will disable the closest following sibling `button` element)
* `previous` which resolves to [element.previousElementSibling](https://developer.mozilla.org/docs/Web/API/Element/previousElementSibling)
* `previous <CSS selector>` which will scan the DOM backwards for the first element that matches the given CSS selector.
  (e.g. `previous input` will disable the closest previous sibling `input` element)

Here is an example with a button that will disable itself during a request:

```html
<button hx-post="/example" hx-disabled-elt="this">
    Post It!
</button>
```

When a request is in flight, this will cause the button to be marked with [the `disabled` attribute](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/disabled), 
which will prevent further clicks from occurring.  

The `hx-disabled-elt` attribute also supports specifying multiple CSS selectors separated by commas to disable multiple elements during the request. Here is an example that disables buttons and text input fields of a particular form during the request:

```html
<form hx-post="/example" hx-disabled-elt="find input[type='text'], find button">
    <input type="text" placeholder="Type here...">
    <button type="submit">Send</button>
</form>
```

## Notes

* `hx-disabled-elt` is inherited and can be placed on a parent element

[hx-trigger]: https://htmx.org/attributes/hx-trigger/"###,
    "hx-disinherit" =>
r###"control and disable automatic attribute inheritance for child nodes
description = """\
  The hx-disinherit attribute in htmx lets you control how child elements inherit attributes from their parents. This \
  documentation explains how to selectively disable inheritance of specific htmx attributes or all attributes, \
  allowing for more granular control over your web application's behavior."""

The default behavior for htmx is to "inherit" many attributes automatically: that is, an attribute such as
[hx-target](@/attributes/hx-target.md) may be placed on a parent element, and all child elements will inherit
that target.

The `hx-disinherit` attribute allows you to control this automatic attribute inheritance. An example scenario is to 
allow you to place an `hx-boost` on the `body` element of a page, but overriding that behavior in a specific part
of the page to allow for more specific behaviors.

htmx evaluates attribute inheritance as follows:

* when `hx-disinherit` is set on a parent node
  * `hx-disinherit="*"` all attribute inheritance for this element will be disabled
  * `hx-disinherit="hx-select hx-get hx-target"` disable inheritance for only one or multiple specified attributes

```html
<div hx-boost="true" hx-select="#content" hx-target="#content" hx-disinherit="*">
  <a href="/page1">Go To Page 1</a> <!-- boosted with the attribute settings above -->
  <a href="/page2" hx-boost="unset">Go To Page 1</a> <!-- not boosted -->
  <button hx-get="/test" hx-target="this"></button> <!-- hx-select is not inherited -->
</div>
```

```html
<div hx-boost="true" hx-select="#content" hx-target="#content" hx-disinherit="hx-target">
  <!-- hx-select is automatically set to parent's value; hx-target is not inherited -->
  <button hx-get="/test"></button>
</div>
```

```html
<div hx-select="#content">
  <div hx-boost="true" hx-target="#content" hx-disinherit="hx-select">
    <!-- hx-target is automatically inherited from parent's value -->
    <!-- hx-select is not inherited, because the direct parent does
    disables inheritance, despite not specifying hx-select itself -->
    <button hx-get="/test"></button>
  </div>
</div>
```

## Notes

* Read more about [Attribute Inheritance](@/docs.md#inheritance)"###,
    "hx-encoding" =>
r###"changes the request encoding type
description = """\
  The hx-encoding attribute in htmx allows you to switch the request encoding from the usual \
  `application/x-www-form-urlencoded` encoding to `multipart/form-data`, usually to support file uploads in an AJAX \
  request."""

The `hx-encoding` attribute allows you to switch the request encoding from the usual `application/x-www-form-urlencoded`
encoding to `multipart/form-data`, usually to support file uploads in an ajax request.

The value of this attribute should be `multipart/form-data`.

The `hx-encoding` tag may be placed on parent elements.

## Notes

* `hx-encoding` is inherited and can be placed on a parent element"###,
    "hx-ext" =>
r###"extensions to use for this element
description = """\
  The hx-ext attribute in htmx enables one or more htmx extensions for an element and all its children. You can also \
  use this attribute to ignore an extension that is enabled by a parent element."""

The `hx-ext` attribute enables an htmx [extension](https://htmx.org/extensions) for an element and all its children.

The value can be a single extension name or a comma-separated list of extensions to apply.

The `hx-ext` tag may be placed on parent elements if you want a plugin to apply to an entire swath of the DOM,
and on the `body` tag for it to apply to all htmx requests.

## Notes

* `hx-ext` is both inherited and merged with parent elements, so you can specify extensions on any element in the DOM 
hierarchy and it will apply to all child elements. 

* You can ignore an extension that is defined by a parent node using `hx-ext="ignore:extensionName"` 


```html
<div hx-ext="example">
  "Example" extension is used in this part of the tree...
  <div hx-ext="ignore:example">
    ... but it will not be used in this part.
  </div>
</div>
```
```html
<body hx-ext="preload,morph">
  "preload" and "morph" extensions are used in this part of the tree...
</body>
```"###,
    "hx-headers" =>
r###"adds to the headers that will be submitted with the request
description = """\
  The hx-headers attribute in htmx allows you to add to the headers that will be submitted with an AJAX request."""

The `hx-headers` attribute allows you to add to the headers that will be submitted with an AJAX request.

By default, the value of this attribute is a list of name-expression values in [JSON (JavaScript Object Notation)](https://www.json.org/json-en.html)
format.

If you wish for `hx-headers` to *evaluate* the values given, you can prefix the values with `javascript:` or `js:`.

```html
  <div hx-get="/example" hx-headers='{"myHeader": "My Value"}'>Get Some HTML, Including A Custom Header in the Request</div>

  <div hx-get="/example" hx-headers='js:{myVal: calculateValue()}'>Get Some HTML, Including a Dynamic Custom Header from Javascript in the Request</div>
```

## Security Considerations

* By default, the value of `hx-headers` must be valid [JSON](https://developer.mozilla.org/en-US/docs/Glossary/JSON).
  It is **not** dynamically computed.  If you use the `javascript:` prefix, be aware that you are introducing
  security considerations, especially when dealing with user input such as query strings or user-generated content,
  which could introduce a [Cross-Site Scripting (XSS)](https://owasp.org/www-community/attacks/xss/) vulnerability.

* Whilst far from being a foolproof solution to [Cross-Site Request Forgery](https://owasp.org/www-community/attacks/csrf), the `hx-headers` attribute can support backend services to provide [CSRF prevention](https://cheatsheetseries.owasp.org/cheatsheets/Cross-Site_Request_Forgery_Prevention_Cheat_Sheet.html). For more information see the [CSRF Prevention](https://htmx.org/docs/#csrf-prevention) section.

## Notes

* `hx-headers` is inherited and can be placed on a parent element.
* A child declaration of a header overrides a parent declaration."###,
    "hx-history" =>
r###"prevent sensitive data being saved to the history cache
description = """\
  The hx-history attribute in htmx allows you to prevent sensitive page data from being stored in the browser's \
  localStorage cache during history navigation, ensuring that the page state is retrieved from the server instead when \
  navigating through history."""

Set the `hx-history` attribute to `false` on any element in the current document, or any html fragment loaded into the current document by htmx, to prevent sensitive data being saved to the localStorage cache when htmx takes a snapshot of the page state. 

History navigation will work as expected, but on restoration the URL will be requested from the server instead of the history cache.

Here is an example:

```html
<html>
<body>
<div hx-history="false">
 ...
</div>
</body>
</html>
```

## Notes

* `hx-history="false"` can be present *anywhere* in the document to embargo the current page state from the history cache (i.e. even outside the element specified for the history snapshot [hx-history-elt](@/attributes/hx-history-elt.md))."###,
    "hx-history-elt" =>
r###"the element to snapshot and restore during history navigation
description = """\
  The hx-history-elt attribute in htmx allows you to specify the element that will be used to snapshot and restore \
  page state during navigation. In most cases we do not recommend using this element."""

The `hx-history-elt` attribute allows you to specify the element that will be used to snapshot and
restore page state during navigation.  By default, the `body` tag is used.  This is typically
good enough for most setups, but you may want to narrow it down to a child element.  Just make
sure that the element is always visible in your application, or htmx will not be able to restore
history navigation properly.


Here is an example:

```html
<html>
<body>
<div id="content" hx-history-elt>
 ...
</div>
</body>
</html>
```

## Notes

* `hx-history-elt` is not inherited
* In most cases we don't recommend narrowing the history snapshot"###,
    "hx-include" =>
r###"include additional data in requests
description = "The hx-include attribute in htmx allows you to include additional element values in an AJAX request."

The `hx-include` attribute allows you to include additional element values in an AJAX request. The value of this
attribute can be:

* A CSS query selector of the elements to include.
* `this` which will include the descendants of the element.
* `closest <CSS selector>` which will find the [closest](https://developer.mozilla.org/docs/Web/API/Element/closest)
  ancestor element or itself, that matches the given CSS selector
  (e.g. `closest tr` will target the closest table row to the element).
* `find <CSS selector>` which will find the first child descendant element that matches the given CSS selector.
* `next <CSS selector>` which will scan the DOM forward for the first element that matches the given CSS selector.
  (e.g. `next .error` will target the closest following sibling element with `error` class)
* `previous <CSS selector>` which will scan the DOM backwards for the first element that matches the given CSS selector.
  (e.g. `previous .error` will target the closest previous sibling with `error` class)

Here is an example that includes a separate input value:

```html
<div>
    <button hx-post="/register" hx-include="[name='email']">
        Register!
    </button>
    Enter email: <input name="email" type="email"/>
</div>
```

This is a little contrived as you would typically enclose both of these elements in a `form` and submit
the value automatically, but it demonstrates the concept.

Note that if you include a non-input element, all input elements enclosed in that element will be included.

## Notes

* `hx-include` is inherited and can be placed on a parent element
* While `hx-include` is inherited, it is evaluated from the element triggering the request. It is easy to get confused
  when working with the extended selectors such as `find` and `closest`.
  ```html
  <div hx-include="find input">
      <button hx-post="/register">
          Register!
      </button>
      Enter email: <input name="email" type="email"/>
  </div>
  ```
  In the above example, when clicking on the button, the `find input` selector is resolved from the button itself, which
  does not return any element here, since the button doesn't have any `input` child, thus in this case, raises an error.
* A standard CSS selector resolves
  to [document.querySelectorAll](https://developer.mozilla.org/docs/Web/API/Document/querySelectorAll) and will include
  multiple elements, while the extended selectors such as `find` or `next` only return a single element at most to
  include"###,
    "hx-indicator" =>
r###"the element to put the htmx-request class on during the request
description = """\
  The hx-indicator attribute in htmx allows you to specify the element that will have the `htmx-request` class added \
  to it for the duration of the request. This can be used to show spinners or progress indicators while the request is \
  in flight."""

The `hx-indicator` attribute allows you to specify the element that will have the `htmx-request` class
added to it for the duration of the request. This can be used to show spinners or progress indicators
while the request is in flight.

The value of this attribute is a CSS query selector of the element or elements to apply the class to,
or the keyword [`closest`](https://developer.mozilla.org/docs/Web/API/Element/closest), followed by a CSS selector, 
which will find the closest ancestor element or itself, that matches the given CSS selector (e.g. `closest tr`);

Here is an example with a spinner adjacent to the button:

```html
<div>
    <button hx-post="/example" hx-indicator="#spinner">
        Post It!
    </button>
    <img  id="spinner" class="htmx-indicator" src="/img/bars.svg"/>
</div>
```

When a request is in flight, this will cause the `htmx-request` class to be added to the `#spinner`
image.  The image also has the `htmx-indicator` class on it, which defines an opacity transition
that will show the spinner:

```css
    .htmx-indicator{
        opacity:0;
        transition: opacity 500ms ease-in;
    }
    .htmx-request .htmx-indicator{
        opacity:1;
    }
    .htmx-request.htmx-indicator{
        opacity:1;
    }
```

If you would prefer a different effect for showing the spinner you could define and use your own indicator
CSS.  Here is an example that uses `display` rather than opacity (Note that we use `my-indicator` instead of `htmx-indicator`):

```css
    .my-indicator{
        display:none;
    }
    .htmx-request .my-indicator{
        display:inline;
    }
    .htmx-request.my-indicator{
        display:inline;
    }
```

Note that the target of the `hx-indicator` selector need not be the exact element that you
want to show: it can be any element in the parent hierarchy of the indicator.

Finally, note that the `htmx-request` class by default is added to the element causing
the request, so you can place an indicator inside of that element and not need to explicitly
call it out with the `hx-indicator` attribute:

```html
<button hx-post="/example">
    Post It!
   <img  class="htmx-indicator" src="/img/bars.svg"/>
</button>
```

## Demo

This simulates what a spinner might look like in that situation:

<button class="btn" classes="toggle htmx-request:3s">
    Post It!
   <img  class="htmx-indicator" src="/img/bars.svg"/>
</button>

## Notes

* `hx-indicator` is inherited and can be placed on a parent element
* In the absence of an explicit indicator, the `htmx-request` class will be added to the element triggering the
  request
* If you want to use your own CSS but still use `htmx-indicator` as class name, then you need to disable `includeIndicatorStyles`. See [Configuring htmx](@/docs.md#config). The easiest way is to add this to the `<head>` of your HTML:
```html
<meta name="htmx-config" content='{"includeIndicatorStyles": false}'>
```"###,
    "hx-inherit" =>
r###"control and enable automatic attribute inheritance for child nodes if it has been disabled by default
"###,
    "hx-params" =>
r###"filters the parameters that will be submitted with a request
description = """\
  The hx-params attribute in htmx allows you to filter the parameters that will be submitted with an AJAX request."""

The `hx-params` attribute allows you to filter the parameters that will be submitted with an AJAX request.  

The possible values of this attribute are:

* `*` - Include all parameters (default)
* `none` - Include no parameters
* `not <param-list>` - Include all except the comma separated list of parameter names
* `<param-list>` - Include all the comma separated list of parameter names

```html
  <div hx-get="/example" hx-params="*">Get Some HTML, Including Params</div>
```

This div will include all the parameters that a `POST` would, but they will be URL encoded
and included in the URL, as per usual with a `GET`.

## Notes

* `hx-params` is inherited and can be placed on a parent element"###,
    "hx-patch" =>
r###"issues a PATCH to the specified URL
description = """\
  The hx-patch attribute in htmx will cause an element to issue a PATCH request to the specified URL and swap the \
  returned HTML into the DOM using a swap strategy."""

The `hx-patch` attribute will cause an element to issue a `PATCH` to the specified URL and swap
the HTML into the DOM using a swap strategy:

```html
<button hx-patch="/account" hx-target="body">
  Patch Your Account
</button>
```

This example will cause the `button` to issue a `PATCH` to `/account` and swap the returned HTML into
 the `innerHTML` of the `body`.
 
## Notes

* `hx-patch` is not inherited
* You can control the target of the swap using the [hx-target](@/attributes/hx-target.md) attribute
* You can control the swap strategy by using the [hx-swap](@/attributes/hx-swap.md) attribute
* You can control what event triggers the request with the [hx-trigger](@/attributes/hx-trigger.md) attribute
* You can control the data submitted with the request in various ways, documented here: [Parameters](@/docs.md#parameters)"###,
    "hx-preserve" =>
r###"specifies elements to keep unchanged between requests
description = """\
  The hx-preserve attribute in htmx allows you to keep an element unchanged during HTML replacement. Elements with \
  hx-preserve set are preserved by `id` when htmx updates any ancestor element."""

The `hx-preserve` attribute allows you to keep an element unchanged during HTML replacement.
Elements with `hx-preserve` set are preserved by `id` when htmx updates any ancestor element.
You *must* set an unchanging `id` on elements for `hx-preserve` to work.
The response requires an element with the same `id`, but its type and other attributes are ignored.

## Notes

* `hx-preserve` is not inherited
* You can use `hx-preserve="true"` or use it as a boolean attribute with just `hx-preserve`
* Some elements cannot unfortunately be preserved properly, such as `<input type="text">` (focus and caret position are lost), iframes or certain types of videos. To tackle some of these cases we recommend the [morphdom extension](https://github.com/bigskysoftware/htmx-extensions/blob/main/src/morphdom-swap/README.md), which does a more elaborate DOM
reconciliation
* When using [History Support](@/docs.md#history) for actions like the back button `hx-preserve` elements will also have their state preserved
* Avoid using [hx-swap](@/attributes/hx-swap.md) set to `none` with requests that could contain a `hx-preserve` element to avoid losing it
* `hx-preserve` can cause elements to be removed from their current location and relocated to a new location when swapping in a partial/oob response
  ```html
  <div id="new_location">
    Just relocated the video here
    <div id="video" hx-preserve></div>
  </div>
  ```
* Can be used on the inside content of a [hx-swap-oob](@/attributes/hx-swap-oob.md) element
  ```html
  <div id="notify" hx-swap-oob="true">
    Notification updated but keep the same retain
    <div id="retain" hx-preserve></div>
  </div>
  ```"###,
    "hx-prompt" =>
r###"shows a prompt() before submitting a request
description = """\
  The hx-prompt attribute in htmx allows you to show a prompt before issuing a request. The value of the prompt will \
  be included in the request in the `HX-Prompt` header."""

The `hx-prompt` attribute allows you to show a prompt before issuing a request.  The value of
the prompt will be included in the request in the `HX-Prompt` header.

Here is an example:

```html
<button hx-delete="/account" hx-prompt="Enter your account name to confirm deletion">
  Delete My Account
</button>
```

## Notes

* `hx-prompt` is inherited and can be placed on a parent element"###,
    "hx-put" =>
r###"issues a PUT to the specified URL
description = """\
  The hx-put attribute in htmx will cause an element to issue a PUT request to the specified URL and swap the returned \
  HTML into the DOM using a swap strategy."""

The `hx-put` attribute will cause an element to issue a `PUT` to the specified URL and swap
the HTML into the DOM using a swap strategy:

```html
<button hx-put="/account" hx-target="body">
  Put Money In Your Account
</button>
```

This example will cause the `button` to issue a `PUT` to `/account` and swap the returned HTML into
 the `innerHTML` of the `body`.
 
## Notes

* `hx-put` is not inherited
* You can control the target of the swap using the [hx-target](@/attributes/hx-target.md) attribute
* You can control the swap strategy by using the [hx-swap](@/attributes/hx-swap.md) attribute
* You can control what event triggers the request with the [hx-trigger](@/attributes/hx-trigger.md) attribute
* You can control the data submitted with the request in various ways, documented here: [Parameters](@/docs.md#parameters)"###,
    "hx-replace-url" =>
r###"replace the URL in the browser location bar
description = """\
  The hx-replace-url attribute in htmx allows you to replace the current URL of the browser location history."""

The `hx-replace-url` attribute allows you to replace the current url of the browser [location history](https://developer.mozilla.org/en-US/docs/Web/API/History_API).

The possible values of this attribute are:

1. `true`, which replaces the fetched URL in the browser navigation bar.
2. `false`, which disables replacing the fetched URL if it would otherwise be replaced due to inheritance.
3. A URL to be replaced into the location bar.
   This may be relative or absolute, as per [`history.replaceState()`](https://developer.mozilla.org/en-US/docs/Web/API/History/replaceState).

Here is an example:

```html
<div hx-get="/account" hx-replace-url="true">
  Go to My Account
</div>
```

This will cause htmx to snapshot the current DOM to `localStorage` and replace the URL `/account' in the browser location bar.

Another example:

```html
<div hx-get="/account" hx-replace-url="/account/home">
  Go to My Account
</div>
```

This will replace the URL `/account/home' in the browser location bar.

## Notes

* `hx-replace-url` is inherited and can be placed on a parent element
* The [`HX-Replace-Url` response header](@/headers/hx-replace-url.md) has similar behavior and can override this attribute.
* The [`hx-history-elt` attribute](@/attributes/hx-history-elt.md) allows changing which element is saved in the history cache.
* The [`hx-push-url` attribute](@/attributes/hx-push-url.md) is a similar and more commonly used attribute, which creates a 
  new history entry rather than replacing the current one."###,
    "hx-request" =>
r###"configures various aspects of the request
description = """\
  The hx-request attribute in htmx allows you to configure the request timeout, whether the request will send \
  credentials, and whether the request will include headers."""

The `hx-request` attribute allows you to configure various aspects of the request via the following attributes:
 
* `timeout` - the timeout for the request, in milliseconds
* `credentials` - if the request will send credentials
* `noHeaders` - strips all headers from the request

These attributes are set using a JSON-like syntax:

```html
<div ... hx-request='{"timeout":100}'>
  ...
</div>
```

You may make the values dynamically evaluated by adding the `javascript:` or `js:` prefix:

```html
<div ... hx-request='js: timeout:getTimeoutSetting() '>
  ...
</div>
```

## Notes

* `hx-request` is merge-inherited and can be placed on a parent element"###,
    "hx-sync" =>
r###"control how requests made by different elements are synchronized
description = "The hx-sync attribute in htmx allows you to synchronize AJAX requests between multiple elements."

The `hx-sync` attribute allows you to synchronize AJAX requests between multiple elements.

The `hx-sync` attribute consists of a CSS selector to indicate the element to synchronize on, followed optionally
by a colon and then by an optional syncing strategy.  The available strategies are:

* `drop` - drop (ignore) this request if an existing request is in flight (the default)
* `abort` - drop (ignore) this request if an existing request is in flight, and, if that is not the case, 
            *abort* this request if another request occurs while it is still in flight
* `replace` - abort the current request, if any, and replace it with this request
* `queue` - place this request in the request queue associated with the given element

The `queue` modifier can take an additional argument indicating exactly how to queue:

* `queue first` - queue the first request to show up while a request is in flight
* `queue last` - queue the last request to show up while a request is in flight
* `queue all` - queue all requests that show up while a request is in flight

## Notes

* `hx-sync` is inherited and can be placed on a parent element

This example resolves a race condition between a form's submit request and an individual input's validation request. Normally, without using `hx-sync`, filling out the input and immediately submitting the form triggers two parallel requests to `/validate` and `/store`. Using `hx-sync="closest form:abort"` on the input will watch for requests on the form and abort the input's request if a form request is present or starts while the input request is in flight.

```html
<form hx-post="/store">
    <input id="title" name="title" type="text" 
        hx-post="/validate" 
        hx-trigger="change"
        hx-sync="closest form:abort">
    <button type="submit">Submit</button>
</form>
```

If you'd rather prioritize the validation request over the submit request, you can use the `drop` strategy. This example will prioritize the validation request over the submit request so that if a validation request is in flight, the form cannot be submitted.

```html
<form hx-post="/store">
    <input id="title" name="title" type="text" 
        hx-post="/validate" 
        hx-trigger="change"
        hx-sync="closest form:drop"
    >
    <button type="submit">Submit</button>
</form>
```

When dealing with forms that contain many inputs, you can prioritize the submit request over all input validation requests using the hx-sync `replace` strategy on the form tag. This will cancel any in-flight validation requests and issue only the `hx-post="/store"` request. If you'd rather abort the submit request and prioritize any existing validation requests you can use the `hx-sync="this:abort"` strategy on the form tag.

```html
<form hx-post="/store" hx-sync="this:replace">
    <input id="title" name="title" type="text" hx-post="/validate" hx-trigger="change" />
    <button type="submit">Submit</button>
</form>
```

When implementing active search functionality the hx-trigger attribute's `delay` modifier can be used to debounce the user's input and avoid making multiple requests while the user types. However, once a request is made, if the user begins typing again a new request will begin even if the previous one has not finished processing. This example will cancel any in-flight requests and use only the last request. In cases where the search input is contained within the target, then using `hx-sync` like this also helps reduce the chances that the input will be replaced while the user is still typing.

```html
<input type="search" 
    hx-get="/search" 
    hx-trigger="keyup changed delay:500ms, search" 
    hx-target="#search-results"
    hx-sync="this:replace">
```"###,
    "hx-validate" =>
r###"force elements to validate themselves before a request
description = """\
  The hx-validate attribute in htmx will cause an element to validate itself using the HTML5 Validation API before it \
  submits a request."""

The `hx-validate` attribute will cause an element to validate itself by way of the [HTML5 Validation API](@/docs.md#validation)
before it submits a request.

Only `<form>` elements validate data by default, but other elements do not. Adding `hx-validate="true"` to `<input>`, `<textarea>` or `<select>` enables validation before sending requests.

## Notes

* `hx-validate` is not inherited"###,
    "hx-vars" =>
r###"adds values dynamically to the parameters to submit with the request (deprecated, please use hx-vals)
description = """\
  The hx-vars attribute in htmx allows you to dynamically add to the parameters that will be submitted with an AJAX \
  request. This attribute has been deprecated. We recommend you use the hx-vals attribute that provides the same \
  functionality with safer defaults."""

**NOTE: `hx-vars` has been deprecated in favor of [`hx-vals`](@/attributes/hx-vals.md), which is safer by default.**

The `hx-vars` attribute allows you to dynamically add to the parameters that will be submitted with an AJAX request.  

The value of this attribute is a comma separated list of `name`:`<expression>` values, the same as the internal
syntax of javascript [Object Literals](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Grammar_and_types#Object_literals).

```html
  <div hx-get="/example" hx-vars="myVar:computeMyVar()">Get Some HTML, Including A Dynamic Value in the Request</div>
```

## Security Considerations

* The expressions in `hx-vars` are dynamically computed which allows you to add JavaScript code that will be executed. Be careful to **never** trust user input in your expressions as this may lead to a [Cross-Site Scripting (XSS)](https://owasp.org/www-community/attacks/xss/) vulnerability. If you are dealing with user input such as query strings or user-generated content, consider using [hx-vals](@/attributes/hx-vals.md) which is a safer alternative.

## Notes

* `hx-vars` is inherited and can be placed on a parent element.
* A child declaration of a variable overrides a parent declaration.
* Input values with the same name will be overridden by variable declarations."###,
};
