pub const CSS : &str =
r###"
/*! normalize.css v8.0.1 | MIT License | github.com/necolas/normalize.css */

/* Document
   ========================================================================== */

/**
 * 1. Correct the line height in all browsers.
 * 2. Prevent adjustments of font size after orientation changes in iOS.
 */

html {
  line-height: 1.15; /* 1 */
  -webkit-text-size-adjust: 100%; /* 2 */
}

/* Sections
   ========================================================================== */

/**
 * Remove the margin in all browsers.
 */

body {
  margin: 0;
}

/**
 * Render the `main` element consistently in IE.
 */

main {
  display: block;
}

/**
 * Correct the font size and margin on `h1` elements within `section` and
 * `article` contexts in Chrome, Firefox, and Safari.
 */

h1 {
  font-size: 2em;
  margin: 0.67em 0;
}

/* Grouping content
   ========================================================================== */

/**
 * 1. Add the correct box sizing in Firefox.
 * 2. Show the overflow in Edge and IE.
 */

hr {
  box-sizing: content-box; /* 1 */
  height: 0; /* 1 */
  overflow: visible; /* 2 */
}

/**
 * 1. Correct the inheritance and scaling of font size in all browsers.
 * 2. Correct the odd `em` font sizing in all browsers.
 */

pre {
  font-family: monospace, monospace; /* 1 */
  font-size: 1em; /* 2 */
}

/* Text-level semantics
   ========================================================================== */

/**
 * Remove the gray background on active links in IE 10.
 */

a {
  background-color: transparent;
}

/**
 * 1. Remove the bottom border in Chrome 57-
 * 2. Add the correct text decoration in Chrome, Edge, IE, Opera, and Safari.
 */

abbr[title] {
  border-bottom: none; /* 1 */
  text-decoration: underline; /* 2 */
  -webkit-text-decoration: underline dotted;
          text-decoration: underline dotted; /* 2 */
}

/**
 * Add the correct font weight in Chrome, Edge, and Safari.
 */

b,
strong {
  font-weight: bolder;
}

/**
 * 1. Correct the inheritance and scaling of font size in all browsers.
 * 2. Correct the odd `em` font sizing in all browsers.
 */

code,
kbd,
samp {
  font-family: monospace, monospace; /* 1 */
  font-size: 1em; /* 2 */
}

/**
 * Add the correct font size in all browsers.
 */

small {
  font-size: 80%;
}

/**
 * Prevent `sub` and `sup` elements from affecting the line height in
 * all browsers.
 */

sub,
sup {
  font-size: 75%;
  line-height: 0;
  position: relative;
  vertical-align: baseline;
}

sub {
  bottom: -0.25em;
}

sup {
  top: -0.5em;
}

/* Embedded content
   ========================================================================== */

/**
 * Remove the border on images inside links in IE 10.
 */

img {
  border-style: none;
}

/* Forms
   ========================================================================== */

/**
 * 1. Change the font styles in all browsers.
 * 2. Remove the margin in Firefox and Safari.
 */

button,
input,
optgroup,
select,
textarea {
  font-family: inherit; /* 1 */
  font-size: 100%; /* 1 */
  line-height: 1.15; /* 1 */
  margin: 0; /* 2 */
}

/**
 * Show the overflow in IE.
 * 1. Show the overflow in Edge.
 */

button,
input { /* 1 */
  overflow: visible;
}

/**
 * Remove the inheritance of text transform in Edge, Firefox, and IE.
 * 1. Remove the inheritance of text transform in Firefox.
 */

button,
select { /* 1 */
  text-transform: none;
}

/**
 * Correct the inability to style clickable types in iOS and Safari.
 */

button,
[type="button"],
[type="reset"],
[type="submit"] {
  -webkit-appearance: button;
}

/**
 * Remove the inner border and padding in Firefox.
 */

button::-moz-focus-inner,
[type="button"]::-moz-focus-inner,
[type="reset"]::-moz-focus-inner,
[type="submit"]::-moz-focus-inner {
  border-style: none;
  padding: 0;
}

/**
 * Restore the focus styles unset by the previous rule.
 */

button:-moz-focusring,
[type="button"]:-moz-focusring,
[type="reset"]:-moz-focusring,
[type="submit"]:-moz-focusring {
  outline: 1px dotted ButtonText;
}

/**
 * Correct the padding in Firefox.
 */

fieldset {
  padding: 0.35em 0.75em 0.625em;
}

/**
 * 1. Correct the text wrapping in Edge and IE.
 * 2. Correct the color inheritance from `fieldset` elements in IE.
 * 3. Remove the padding so developers are not caught out when they zero out
 *    `fieldset` elements in all browsers.
 */

legend {
  box-sizing: border-box; /* 1 */
  color: inherit; /* 2 */
  display: table; /* 1 */
  max-width: 100%; /* 1 */
  padding: 0; /* 3 */
  white-space: normal; /* 1 */
}

/**
 * Add the correct vertical alignment in Chrome, Firefox, and Opera.
 */

progress {
  vertical-align: baseline;
}

/**
 * Remove the default vertical scrollbar in IE 10+.
 */

textarea {
  overflow: auto;
}

/**
 * 1. Add the correct box sizing in IE 10.
 * 2. Remove the padding in IE 10.
 */

[type="checkbox"],
[type="radio"] {
  box-sizing: border-box; /* 1 */
  padding: 0; /* 2 */
}

/**
 * Correct the cursor style of increment and decrement buttons in Chrome.
 */

[type="number"]::-webkit-inner-spin-button,
[type="number"]::-webkit-outer-spin-button {
  height: auto;
}

/**
 * 1. Correct the odd appearance in Chrome and Safari.
 * 2. Correct the outline style in Safari.
 */

[type="search"] {
  -webkit-appearance: textfield; /* 1 */
  outline-offset: -2px; /* 2 */
}

/**
 * Remove the inner padding in Chrome and Safari on macOS.
 */

[type="search"]::-webkit-search-decoration {
  -webkit-appearance: none;
}

/**
 * 1. Correct the inability to style clickable types in iOS and Safari.
 * 2. Change font properties to `inherit` in Safari.
 */

::-webkit-file-upload-button {
  -webkit-appearance: button; /* 1 */
  font: inherit; /* 2 */
}

/* Interactive
   ========================================================================== */

/*
 * Add the correct display in Edge, IE 10+, and Firefox.
 */

details {
  display: block;
}

/*
 * Add the correct display in all browsers.
 */

summary {
  display: list-item;
}

/* Misc
   ========================================================================== */

/**
 * Add the correct display in IE 10+.
 */

template {
  display: none;
}

/**
 * Add the correct display in IE 10.
 */

[hidden] {
  display: none;
}

/**
 * Manually forked from SUIT CSS Base: https://github.com/suitcss/base
 * A thin layer on top of normalize.css that provides a starting point more
 * suitable for web applications.
 */

/**
 * 1. Prevent padding and border from affecting element width
 * https://goo.gl/pYtbK7
 * 2. Change the default font family in all browsers (opinionated)
 */

html {
  box-sizing: border-box; /* 1 */
  font-family: sans-serif; /* 2 */
}

*,
*::before,
*::after {
  box-sizing: inherit;
}

/**
 * Removes the default spacing and border for appropriate elements.
 */

blockquote,
dl,
dd,
h1,
h2,
h3,
h4,
h5,
h6,
hr,
figure,
p,
pre {
  margin: 0;
}

button {
  background: transparent;
  padding: 0;
}

/**
 * Work around a Firefox/IE bug where the transparent `button` background
 * results in a loss of the default `button` focus styles.
 */

button:focus {
  outline: 1px dotted;
  outline: 5px auto -webkit-focus-ring-color;
}

fieldset {
  margin: 0;
  padding: 0;
}

ol,
ul {
  list-style: none;
  margin: 0;
  padding: 0;
}

/**
 * Tailwind custom reset styles
 */

/**
 * 1. Use the system font stack as a sane default.
 * 2. Use Tailwind's default "normal" line-height so the user isn't forced
 * to override it to ensure consistency even when using the default theme.
 */

html {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji"; /* 1 */
  line-height: 1.5; /* 2 */
}

/**
 * Allow adding a border to an element by just adding a border-width.
 *
 * By default, the way the browser specifies that an element should have no
 * border is by setting it's border-style to `none` in the user-agent
 * stylesheet.
 *
 * In order to easily add borders to elements by just setting the `border-width`
 * property, we change the default border-style for all elements to `solid`, and
 * use border-width to hide them instead. This way our `border` utilities only
 * need to set the `border-width` property instead of the entire `border`
 * shorthand, making our border utilities much more straightforward to compose.
 *
 * https://github.com/tailwindcss/tailwindcss/pull/116
 */

*,
*::before,
*::after {
  border-width: 0;
  border-style: solid;
  border-color: #e2e8f0;
}

/*
 * Ensure horizontal rules are visible by default
 */

hr {
  border-top-width: 1px;
}

/**
 * Undo the `border-style: none` reset that Normalize applies to images so that
 * our `border-{width}` utilities have the expected effect.
 *
 * The Normalize reset is unnecessary for us since we default the border-width
 * to 0 on all elements.
 *
 * https://github.com/tailwindcss/tailwindcss/issues/362
 */

img {
  border-style: solid;
}

textarea {
  resize: vertical;
}

input::-webkit-input-placeholder, textarea::-webkit-input-placeholder {
  color: #a0aec0;
}

input::-moz-placeholder, textarea::-moz-placeholder {
  color: #a0aec0;
}

input:-ms-input-placeholder, textarea:-ms-input-placeholder {
  color: #a0aec0;
}

input::-ms-input-placeholder, textarea::-ms-input-placeholder {
  color: #a0aec0;
}

input::placeholder,
textarea::placeholder {
  color: #a0aec0;
}

button,
[role="button"] {
  cursor: pointer;
}

table {
  border-collapse: collapse;
}

h1,
h2,
h3,
h4,
h5,
h6 {
  font-size: inherit;
  font-weight: inherit;
}

/**
 * Reset links to optimize for opt-in styling instead of
 * opt-out.
 */

a {
  color: inherit;
  text-decoration: inherit;
}

/**
 * Reset form element properties that are easy to forget to
 * style explicitly so you don't inadvertently introduce
 * styles that deviate from your design system. These styles
 * supplement a partial reset that is already applied by
 * normalize.css.
 */

button,
input,
optgroup,
select,
textarea {
  padding: 0;
  line-height: inherit;
  color: inherit;
}

/**
 * Use the configured 'mono' font family for elements that
 * are expected to be rendered with a monospace font, falling
 * back to the system monospace stack if there is no configured
 * 'mono' font family.
 */

pre,
code,
kbd,
samp {
  font-family: Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
}

/**
 * Make replaced elements `display: block` by default as that's
 * the behavior you want almost all of the time. Inspired by
 * CSS Remedy, with `svg` added as well.
 *
 * https://github.com/mozdevs/cssremedy/issues/14
 */

img,
svg,
video,
canvas,
audio,
iframe,
embed,
object {
  display: block;
  vertical-align: middle;
}

/**
 * Constrain images and videos to the parent width and preserve
 * their instrinsic aspect ratio.
 *
 * https://github.com/mozdevs/cssremedy/issues/14
 */

img,
video {
  max-width: 100%;
  height: auto;
}

.container {
  width: 100%;
}

@media (min-width: 640px) {
  .container {
    max-width: 640px;
  }
}

@media (min-width: 768px) {
  .container {
    max-width: 768px;
  }
}

@media (min-width: 1024px) {
  .container {
    max-width: 1024px;
  }
}

@media (min-width: 1280px) {
  .container {
    max-width: 1280px;
  }
}

.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border-width: 0;
}

.not-sr-only {
  position: static;
  width: auto;
  height: auto;
  padding: 0;
  margin: 0;
  overflow: visible;
  clip: auto;
  white-space: normal;
}

.focus\:sr-only:focus {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border-width: 0;
}

.focus\:not-sr-only:focus {
  position: static;
  width: auto;
  height: auto;
  padding: 0;
  margin: 0;
  overflow: visible;
  clip: auto;
  white-space: normal;
}

.appearance-none {
  -webkit-appearance: none;
     -moz-appearance: none;
          appearance: none;
}

.bg-fixed {
  background-attachment: fixed;
}

.bg-local {
  background-attachment: local;
}

.bg-scroll {
  background-attachment: scroll;
}

.bg-transparent {
  background-color: transparent;
}

.bg-black {
  background-color: #000;
}

.bg-white {
  background-color: #fff;
}

.bg-gray-100 {
  background-color: #f7fGafc;
}

.bg-gray-200 {
  background-color: #edf2f7;
}

.bg-gray-300 {
  background-color: #e2e8f0;
}

.bg-gray-400 {
  background-color: #cbd5e0;
}

.bg-gray-500 {
  background-color: #a0aec0;
}

.bg-gray-600 {
  background-color: #718096;
}

.bg-gray-700 {
  background-color: #4a5568;
}

.bg-gray-800 {
  background-color: #2d3748;
}

.bg-gray-900 {
  background-color: #1a202c;
}

.bg-red-100 {
  background-color: #fff5f5;
}

.bg-red-200 {
  background-color: #fed7d7;
}

.bg-red-300 {
  background-color: #feb2b2;
}

.bg-red-400 {
  background-color: #fc8181;
}

.bg-red-500 {
  background-color: #f56565;
}

.bg-red-600 {
  background-color: #e53e3e;
}

.bg-red-700 {
  background-color: #c53030;
}

.bg-red-800 {
  background-color: #9b2c2c;
}

.bg-red-900 {
  background-color: #742a2a;
}

.bg-orange-100 {
  background-color: #fffaf0;
}

.bg-orange-200 {
  background-color: #feebc8;
}

.bg-orange-300 {
  background-color: #fbd38d;
}

.bg-orange-400 {
  background-color: #f6ad55;
}

.bg-orange-500 {
  background-color: #ed8936;
}

.bg-orange-600 {
  background-color: #dd6b20;
}

.bg-orange-700 {
  background-color: #c05621;
}

.bg-orange-800 {
  background-color: #9c4221;
}

.bg-orange-900 {
  background-color: #7b341e;
}

.bg-yellow-100 {
  background-color: #fffff0;
}

.bg-yellow-200 {
  background-color: #fefcbf;
}

.bg-yellow-300 {
  background-color: #faf089;
}

.bg-yellow-400 {
  background-color: #f6e05e;
}

.bg-yellow-500 {
  background-color: #ecc94b;
}

.bg-yellow-600 {
  background-color: #d69e2e;
}

.bg-yellow-700 {
  background-color: #b7791f;
}

.bg-yellow-800 {
  background-color: #975a16;
}

.bg-yellow-900 {
  background-color: #744210;
}

.bg-green-100 {
  background-color: #f0fff4;
}

.bg-green-200 {
  background-color: #c6f6d5;
}

.bg-green-300 {
  background-color: #9ae6b4;
}

.bg-green-400 {
  background-color: #68d391;
}

.bg-green-500 {
  background-color: #48bb78;
}

.bg-green-600 {
  background-color: #38a169;
}

.bg-green-700 {
  background-color: #2f855a;
}

.bg-green-800 {
  background-color: #276749;
}

.bg-green-900 {
  background-color: #22543d;
}

.bg-teal-100 {
  background-color: #e6fffa;
}

.bg-teal-200 {
  background-color: #b2f5ea;
}

.bg-teal-300 {
  background-color: #81e6d9;
}

.bg-teal-400 {
  background-color: #4fd1c5;
}

.bg-teal-500 {
  background-color: #38b2ac;
}

.bg-teal-600 {
  background-color: #319795;
}

.bg-teal-700 {
  background-color: #2c7a7b;
}

.bg-teal-800 {
  background-color: #285e61;
}

.bg-teal-900 {
  background-color: #234e52;
}

.bg-blue-100 {
  background-color: #ebf8ff;
}

.bg-blue-200 {
  background-color: #bee3f8;
}

.bg-blue-300 {
  background-color: #90cdf4;
}

.bg-blue-400 {
  background-color: #63b3ed;
}

.bg-blue-500 {
  background-color: #4299e1;
}

.bg-blue-600 {
  background-color: #3182ce;
}

.bg-blue-700 {
  background-color: #2b6cb0;
}

.bg-blue-800 {
  background-color: #2c5282;
}

.bg-blue-900 {
  background-color: #2a4365;
}

.bg-indigo-100 {
  background-color: #ebf4ff;
}

.bg-indigo-200 {
  background-color: #c3dafe;
}

.bg-indigo-300 {
  background-color: #a3bffa;
}

.bg-indigo-400 {
  background-color: #7f9cf5;
}

.bg-indigo-500 {
  background-color: #667eea;
}

.bg-indigo-600 {
  background-color: #5a67d8;
}

.bg-indigo-700 {
  background-color: #4c51bf;
}

.bg-indigo-800 {
  background-color: #434190;
}

.bg-indigo-900 {
  background-color: #3c366b;
}

.bg-purple-100 {
  background-color: #faf5ff;
}

.bg-purple-200 {
  background-color: #e9d8fd;
}

.bg-purple-300 {
  background-color: #d6bcfa;
}

.bg-purple-400 {
  background-color: #b794f4;
}

.bg-purple-500 {
  background-color: #9f7aea;
}

.bg-purple-600 {
  background-color: #805ad5;
}

.bg-purple-700 {
  background-color: #6b46c1;
}

.bg-purple-800 {
  background-color: #553c9a;
}

.bg-purple-900 {
  background-color: #44337a;
}

.bg-pink-100 {
  background-color: #fff5f7;
}

.bg-pink-200 {
  background-color: #fed7e2;
}

.bg-pink-300 {
  background-color: #fbb6ce;
}

.bg-pink-400 {
  background-color: #f687b3;
}

.bg-pink-500 {
  background-color: #ed64a6;
}

.bg-pink-600 {
  background-color: #d53f8c;
}

.bg-pink-700 {
  background-color: #b83280;
}

.bg-pink-800 {
  background-color: #97266d;
}

.bg-pink-900 {
  background-color: #702459;
}

.hover\:bg-transparent:hover {
  background-color: transparent;
}

.hover\:bg-black:hover {
  background-color: #000;
}

.hover\:bg-white:hover {
  background-color: #fff;
}

.hover\:bg-gray-100:hover {
  background-color: #f7fafc;
}

.hover\:bg-gray-200:hover {
  background-color: #edf2f7;
}

.hover\:bg-gray-300:hover {
  background-color: #e2e8f0;
}

.hover\:bg-gray-400:hover {
  background-color: #cbd5e0;
}

.hover\:bg-gray-500:hover {
  background-color: #a0aec0;
}

.hover\:bg-gray-600:hover {
  background-color: #718096;
}

.hover\:bg-gray-700:hover {
  background-color: #4a5568;
}

.hover\:bg-gray-800:hover {
  background-color: #2d3748;
}

.hover\:bg-gray-900:hover {
  background-color: #1a202c;
}

.hover\:bg-red-100:hover {
  background-color: #fff5f5;
}

.hover\:bg-red-200:hover {
  background-color: #fed7d7;
}

.hover\:bg-red-300:hover {
  background-color: #feb2b2;
}

.hover\:bg-red-400:hover {
  background-color: #fc8181;
}

.hover\:bg-red-500:hover {
  background-color: #f56565;
}

.hover\:bg-red-600:hover {
  background-color: #e53e3e;
}

.hover\:bg-red-700:hover {
  background-color: #c53030;
}

.hover\:bg-red-800:hover {
  background-color: #9b2c2c;
}

.hover\:bg-red-900:hover {
  background-color: #742a2a;
}

.hover\:bg-orange-100:hover {
  background-color: #fffaf0;
}

.hover\:bg-orange-200:hover {
  background-color: #feebc8;
}

.hover\:bg-orange-300:hover {
  background-color: #fbd38d;
}

.hover\:bg-orange-400:hover {
  background-color: #f6ad55;
}

.hover\:bg-orange-500:hover {
  background-color: #ed8936;
}

.hover\:bg-orange-600:hover {
  background-color: #dd6b20;
}

.hover\:bg-orange-700:hover {
  background-color: #c05621;
}

.hover\:bg-orange-800:hover {
  background-color: #9c4221;
}

.hover\:bg-orange-900:hover {
  background-color: #7b341e;
}

.hover\:bg-yellow-100:hover {
  background-color: #fffff0;
}

.hover\:bg-yellow-200:hover {
  background-color: #fefcbf;
}

.hover\:bg-yellow-300:hover {
  background-color: #faf089;
}

.hover\:bg-yellow-400:hover {
  background-color: #f6e05e;
}

.hover\:bg-yellow-500:hover {
  background-color: #ecc94b;
}

.hover\:bg-yellow-600:hover {
  background-color: #d69e2e;
}

.hover\:bg-yellow-700:hover {
  background-color: #b7791f;
}

.hover\:bg-yellow-800:hover {
  background-color: #975a16;
}

.hover\:bg-yellow-900:hover {
  background-color: #744210;
}

.hover\:bg-green-100:hover {
  background-color: #f0fff4;
}

.hover\:bg-green-200:hover {
  background-color: #c6f6d5;
}

.hover\:bg-green-300:hover {
  background-color: #9ae6b4;
}

.hover\:bg-green-400:hover {
  background-color: #68d391;
}

.hover\:bg-green-500:hover {
  background-color: #48bb78;
}

.hover\:bg-green-600:hover {
  background-color: #38a169;
}

.hover\:bg-green-700:hover {
  background-color: #2f855a;
}

.hover\:bg-green-800:hover {
  background-color: #276749;
}

.hover\:bg-green-900:hover {
  background-color: #22543d;
}

.hover\:bg-teal-100:hover {
  background-color: #e6fffa;
}

.hover\:bg-teal-200:hover {
  background-color: #b2f5ea;
}

.hover\:bg-teal-300:hover {
  background-color: #81e6d9;
}

.hover\:bg-teal-400:hover {
  background-color: #4fd1c5;
}

.hover\:bg-teal-500:hover {
  background-color: #38b2ac;
}

.hover\:bg-teal-600:hover {
  background-color: #319795;
}

.hover\:bg-teal-700:hover {
  background-color: #2c7a7b;
}

.hover\:bg-teal-800:hover {
  background-color: #285e61;
}

.hover\:bg-teal-900:hover {
  background-color: #234e52;
}

.hover\:bg-blue-100:hover {
  background-color: #ebf8ff;
}

.hover\:bg-blue-200:hover {
  background-color: #bee3f8;
}

.hover\:bg-blue-300:hover {
  background-color: #90cdf4;
}

.hover\:bg-blue-400:hover {
  background-color: #63b3ed;
}

.hover\:bg-blue-500:hover {
  background-color: #4299e1;
}

.hover\:bg-blue-600:hover {
  background-color: #3182ce;
}

.hover\:bg-blue-700:hover {
  background-color: #2b6cb0;
}

.hover\:bg-blue-800:hover {
  background-color: #2c5282;
}

.hover\:bg-blue-900:hover {
  background-color: #2a4365;
}

.hover\:bg-indigo-100:hover {
  background-color: #ebf4ff;
}

.hover\:bg-indigo-200:hover {
  background-color: #c3dafe;
}

.hover\:bg-indigo-300:hover {
  background-color: #a3bffa;
}

.hover\:bg-indigo-400:hover {
  background-color: #7f9cf5;
}

.hover\:bg-indigo-500:hover {
  background-color: #667eea;
}

.hover\:bg-indigo-600:hover {
  background-color: #5a67d8;
}

.hover\:bg-indigo-700:hover {
  background-color: #4c51bf;
}

.hover\:bg-indigo-800:hover {
  background-color: #434190;
}

.hover\:bg-indigo-900:hover {
  background-color: #3c366b;
}

.hover\:bg-purple-100:hover {
  background-color: #faf5ff;
}

.hover\:bg-purple-200:hover {
  background-color: #e9d8fd;
}

.hover\:bg-purple-300:hover {
  background-color: #d6bcfa;
}

.hover\:bg-purple-400:hover {
  background-color: #b794f4;
}

.hover\:bg-purple-500:hover {
  background-color: #9f7aea;
}

.hover\:bg-purple-600:hover {
  background-color: #805ad5;
}

.hover\:bg-purple-700:hover {
  background-color: #6b46c1;
}

.hover\:bg-purple-800:hover {
  background-color: #553c9a;
}

.hover\:bg-purple-900:hover {
  background-color: #44337a;
}

.hover\:bg-pink-100:hover {
  background-color: #fff5f7;
}

.hover\:bg-pink-200:hover {
  background-color: #fed7e2;
}

.hover\:bg-pink-300:hover {
  background-color: #fbb6ce;
}

.hover\:bg-pink-400:hover {
  background-color: #f687b3;
}

.hover\:bg-pink-500:hover {
  background-color: #ed64a6;
}

.hover\:bg-pink-600:hover {
  background-color: #d53f8c;
}

.hover\:bg-pink-700:hover {
  background-color: #b83280;
}

.hover\:bg-pink-800:hover {
  background-color: #97266d;
}

.hover\:bg-pink-900:hover {
  background-color: #702459;
}

.focus\:bg-transparent:focus {
  background-color: transparent;
}

.focus\:bg-black:focus {
  background-color: #000;
}

.focus\:bg-white:focus {
  background-color: #fff;
}

.focus\:bg-gray-100:focus {
  background-color: #f7fafc;
}

.focus\:bg-gray-200:focus {
  background-color: #edf2f7;
}

.focus\:bg-gray-300:focus {
  background-color: #e2e8f0;
}

.focus\:bg-gray-400:focus {
  background-color: #cbd5e0;
}

.focus\:bg-gray-500:focus {
  background-color: #a0aec0;
}

.focus\:bg-gray-600:focus {
  background-color: #718096;
}

.focus\:bg-gray-700:focus {
  background-color: #4a5568;
}

.focus\:bg-gray-800:focus {
  background-color: #2d3748;
}

.focus\:bg-gray-900:focus {
  background-color: #1a202c;
}

.focus\:bg-red-100:focus {
  background-color: #fff5f5;
}

.focus\:bg-red-200:focus {
  background-color: #fed7d7;
}

.focus\:bg-red-300:focus {
  background-color: #feb2b2;
}

.focus\:bg-red-400:focus {
  background-color: #fc8181;
}

.focus\:bg-red-500:focus {
  background-color: #f56565;
}

.focus\:bg-red-600:focus {
  background-color: #e53e3e;
}

.focus\:bg-red-700:focus {
  background-color: #c53030;
}

.focus\:bg-red-800:focus {
  background-color: #9b2c2c;
}

.focus\:bg-red-900:focus {
  background-color: #742a2a;
}

.focus\:bg-orange-100:focus {
  background-color: #fffaf0;
}

.focus\:bg-orange-200:focus {
  background-color: #feebc8;
}

.focus\:bg-orange-300:focus {
  background-color: #fbd38d;
}

.focus\:bg-orange-400:focus {
  background-color: #f6ad55;
}

.focus\:bg-orange-500:focus {
  background-color: #ed8936;
}

.focus\:bg-orange-600:focus {
  background-color: #dd6b20;
}

.focus\:bg-orange-700:focus {
  background-color: #c05621;
}

.focus\:bg-orange-800:focus {
  background-color: #9c4221;
}

.focus\:bg-orange-900:focus {
  background-color: #7b341e;
}

.focus\:bg-yellow-100:focus {
  background-color: #fffff0;
}

.focus\:bg-yellow-200:focus {
  background-color: #fefcbf;
}

.focus\:bg-yellow-300:focus {
  background-color: #faf089;
}

.focus\:bg-yellow-400:focus {
  background-color: #f6e05e;
}

.focus\:bg-yellow-500:focus {
  background-color: #ecc94b;
}

.focus\:bg-yellow-600:focus {
  background-color: #d69e2e;
}

.focus\:bg-yellow-700:focus {
  background-color: #b7791f;
}

.focus\:bg-yellow-800:focus {
  background-color: #975a16;
}

.focus\:bg-yellow-900:focus {
  background-color: #744210;
}

.focus\:bg-green-100:focus {
  background-color: #f0fff4;
}

.focus\:bg-green-200:focus {
  background-color: #c6f6d5;
}

.focus\:bg-green-300:focus {
  background-color: #9ae6b4;
}

.focus\:bg-green-400:focus {
  background-color: #68d391;
}

.focus\:bg-green-500:focus {
  background-color: #48bb78;
}

.focus\:bg-green-600:focus {
  background-color: #38a169;
}

.focus\:bg-green-700:focus {
  background-color: #2f855a;
}

.focus\:bg-green-800:focus {
  background-color: #276749;
}

.focus\:bg-green-900:focus {
  background-color: #22543d;
}

.focus\:bg-teal-100:focus {
  background-color: #e6fffa;
}

.focus\:bg-teal-200:focus {
  background-color: #b2f5ea;
}

.focus\:bg-teal-300:focus {
  background-color: #81e6d9;
}

.focus\:bg-teal-400:focus {
  background-color: #4fd1c5;
}

.focus\:bg-teal-500:focus {
  background-color: #38b2ac;
}

.focus\:bg-teal-600:focus {
  background-color: #319795;
}

.focus\:bg-teal-700:focus {
  background-color: #2c7a7b;
}

.focus\:bg-teal-800:focus {
  background-color: #285e61;
}

.focus\:bg-teal-900:focus {
  background-color: #234e52;
}

.focus\:bg-blue-100:focus {
  background-color: #ebf8ff;
}

.focus\:bg-blue-200:focus {
  background-color: #bee3f8;
}

.focus\:bg-blue-300:focus {
  background-color: #90cdf4;
}

.focus\:bg-blue-400:focus {
  background-color: #63b3ed;
}

.focus\:bg-blue-500:focus {
  background-color: #4299e1;
}

.focus\:bg-blue-600:focus {
  background-color: #3182ce;
}

.focus\:bg-blue-700:focus {
  background-color: #2b6cb0;
}

.focus\:bg-blue-800:focus {
  background-color: #2c5282;
}

.focus\:bg-blue-900:focus {
  background-color: #2a4365;
}

.focus\:bg-indigo-100:focus {
  background-color: #ebf4ff;
}

.focus\:bg-indigo-200:focus {
  background-color: #c3dafe;
}

.focus\:bg-indigo-300:focus {
  background-color: #a3bffa;
}

.focus\:bg-indigo-400:focus {
  background-color: #7f9cf5;
}

.focus\:bg-indigo-500:focus {
  background-color: #667eea;
}

.focus\:bg-indigo-600:focus {
  background-color: #5a67d8;
}

.focus\:bg-indigo-700:focus {
  background-color: #4c51bf;
}

.focus\:bg-indigo-800:focus {
  background-color: #434190;
}

.focus\:bg-indigo-900:focus {
  background-color: #3c366b;
}

.focus\:bg-purple-100:focus {
  background-color: #faf5ff;
}

.focus\:bg-purple-200:focus {
  background-color: #e9d8fd;
}

.focus\:bg-purple-300:focus {
  background-color: #d6bcfa;
}

.focus\:bg-purple-400:focus {
  background-color: #b794f4;
}

.focus\:bg-purple-500:focus {
  background-color: #9f7aea;
}

.focus\:bg-purple-600:focus {
  background-color: #805ad5;
}

.focus\:bg-purple-700:focus {
  background-color: #6b46c1;
}

.focus\:bg-purple-800:focus {
  background-color: #553c9a;
}

.focus\:bg-purple-900:focus {
  background-color: #44337a;
}

.focus\:bg-pink-100:focus {
  background-color: #fff5f7;
}

.focus\:bg-pink-200:focus {
  background-color: #fed7e2;
}

.focus\:bg-pink-300:focus {
  background-color: #fbb6ce;
}

.focus\:bg-pink-400:focus {
  background-color: #f687b3;
}

.focus\:bg-pink-500:focus {
  background-color: #ed64a6;
}

.focus\:bg-pink-600:focus {
  background-color: #d53f8c;
}

.focus\:bg-pink-700:focus {
  background-color: #b83280;
}

.focus\:bg-pink-800:focus {
  background-color: #97266d;
}

.focus\:bg-pink-900:focus {
  background-color: #702459;
}

.bg-bottom {
  background-position: bottom;
}

.bg-center {
  background-position: center;
}

.bg-left {
  background-position: left;
}

.bg-left-bottom {
  background-position: left bottom;
}

.bg-left-top {
  background-position: left top;
}

.bg-right {
  background-position: right;
}

.bg-right-bottom {
  background-position: right bottom;
}

.bg-right-top {
  background-position: right top;
}

.bg-top {
  background-position: top;
}

.bg-repeat {
  background-repeat: repeat;
}

.bg-no-repeat {
  background-repeat: no-repeat;
}

.bg-repeat-x {
  background-repeat: repeat-x;
}

.bg-repeat-y {
  background-repeat: repeat-y;
}

.bg-repeat-round {
  background-repeat: round;
}

.bg-repeat-space {
  background-repeat: space;
}

.bg-auto {
  background-size: auto;
}

.bg-cover {
  background-size: cover;
}

.bg-contain {
  background-size: contain;
}

.border-collapse {
  border-collapse: collapse;
}

.border-separate {
  border-collapse: separate;
}

.border-transparent {
  border-color: transparent;
}

.border-black {
  border-color: #000;
}

.border-white {
  border-color: #fff;
}

.border-gray-100 {
  border-color: #f7fafc;
}

.border-gray-200 {
  border-color: #edf2f7;
}

.border-gray-300 {
  border-color: #e2e8f0;
}

.border-gray-400 {
  border-color: #cbd5e0;
}

.border-gray-500 {
  border-color: #a0aec0;
}

.border-gray-600 {
  border-color: #718096;
}

.border-gray-700 {
  border-color: #4a5568;
}

.border-gray-800 {
  border-color: #2d3748;
}

.border-gray-900 {
  border-color: #1a202c;
}

.border-red-100 {
  border-color: #fff5f5;
}

.border-red-200 {
  border-color: #fed7d7;
}

.border-red-300 {
  border-color: #feb2b2;
}

.border-red-400 {
  border-color: #fc8181;
}

.border-red-500 {
  border-color: #f56565;
}

.border-red-600 {
  border-color: #e53e3e;
}

.border-red-700 {
  border-color: #c53030;
}

.border-red-800 {
  border-color: #9b2c2c;
}

.border-red-900 {
  border-color: #742a2a;
}

.border-orange-100 {
  border-color: #fffaf0;
}

.border-orange-200 {
  border-color: #feebc8;
}

.border-orange-300 {
  border-color: #fbd38d;
}

.border-orange-400 {
  border-color: #f6ad55;
}

.border-orange-500 {
  border-color: #ed8936;
}

.border-orange-600 {
  border-color: #dd6b20;
}

.border-orange-700 {
  border-color: #c05621;
}

.border-orange-800 {
  border-color: #9c4221;
}

.border-orange-900 {
  border-color: #7b341e;
}

.border-yellow-100 {
  border-color: #fffff0;
}

.border-yellow-200 {
  border-color: #fefcbf;
}

.border-yellow-300 {
  border-color: #faf089;
}

.border-yellow-400 {
  border-color: #f6e05e;
}

.border-yellow-500 {
  border-color: #ecc94b;
}

.border-yellow-600 {
  border-color: #d69e2e;
}

.border-yellow-700 {
  border-color: #b7791f;
}

.border-yellow-800 {
  border-color: #975a16;
}

.border-yellow-900 {
  border-color: #744210;
}

.border-green-100 {
  border-color: #f0fff4;
}

.border-green-200 {
  border-color: #c6f6d5;
}

.border-green-300 {
  border-color: #9ae6b4;
}

.border-green-400 {
  border-color: #68d391;
}

.border-green-500 {
  border-color: #48bb78;
}

.border-green-600 {
  border-color: #38a169;
}

.border-green-700 {
  border-color: #2f855a;
}

.border-green-800 {
  border-color: #276749;
}

.border-green-900 {
  border-color: #22543d;
}

.border-teal-100 {
  border-color: #e6fffa;
}

.border-teal-200 {
  border-color: #b2f5ea;
}

.border-teal-300 {
  border-color: #81e6d9;
}

.border-teal-400 {
  border-color: #4fd1c5;
}

.border-teal-500 {
  border-color: #38b2ac;
}

.border-teal-600 {
  border-color: #319795;
}

.border-teal-700 {
  border-color: #2c7a7b;
}

.border-teal-800 {
  border-color: #285e61;
}

.border-teal-900 {
  border-color: #234e52;
}

.border-blue-100 {
  border-color: #ebf8ff;
}

.border-blue-200 {
  border-color: #bee3f8;
}

.border-blue-300 {
  border-color: #90cdf4;
}

.border-blue-400 {
  border-color: #63b3ed;
}

.border-blue-500 {
  border-color: #4299e1;
}

.border-blue-600 {
  border-color: #3182ce;
}

.border-blue-700 {
  border-color: #2b6cb0;
}

.border-blue-800 {
  border-color: #2c5282;
}

.border-blue-900 {
  border-color: #2a4365;
}

.border-indigo-100 {
  border-color: #ebf4ff;
}

.border-indigo-200 {
  border-color: #c3dafe;
}

.border-indigo-300 {
  border-color: #a3bffa;
}

.border-indigo-400 {
  border-color: #7f9cf5;
}

.border-indigo-500 {
  border-color: #667eea;
}

.border-indigo-600 {
  border-color: #5a67d8;
}

.border-indigo-700 {
  border-color: #4c51bf;
}

.border-indigo-800 {
  border-color: #434190;
}

.border-indigo-900 {
  border-color: #3c366b;
}

.border-purple-100 {
  border-color: #faf5ff;
}

.border-purple-200 {
  border-color: #e9d8fd;
}

.border-purple-300 {
  border-color: #d6bcfa;
}

.border-purple-400 {
  border-color: #b794f4;
}

.border-purple-500 {
  border-color: #9f7aea;
}

.border-purple-600 {
  border-color: #805ad5;
}

.border-purple-700 {
  border-color: #6b46c1;
}

.border-purple-800 {
  border-color: #553c9a;
}

.border-purple-900 {
  border-color: #44337a;
}

.border-pink-100 {
  border-color: #fff5f7;
}

.border-pink-200 {
  border-color: #fed7e2;
}

.border-pink-300 {
  border-color: #fbb6ce;
}

.border-pink-400 {
  border-color: #f687b3;
}

.border-pink-500 {
  border-color: #ed64a6;
}

.border-pink-600 {
  border-color: #d53f8c;
}

.border-pink-700 {
  border-color: #b83280;
}

.border-pink-800 {
  border-color: #97266d;
}

.border-pink-900 {
  border-color: #702459;
}

.hover\:border-transparent:hover {
  border-color: transparent;
}

.hover\:border-black:hover {
  border-color: #000;
}

.hover\:border-white:hover {
  border-color: #fff;
}

.hover\:border-gray-100:hover {
  border-color: #f7fafc;
}

.hover\:border-gray-200:hover {
  border-color: #edf2f7;
}

.hover\:border-gray-300:hover {
  border-color: #e2e8f0;
}

.hover\:border-gray-400:hover {
  border-color: #cbd5e0;
}

.hover\:border-gray-500:hover {
  border-color: #a0aec0;
}

.hover\:border-gray-600:hover {
  border-color: #718096;
}

.hover\:border-gray-700:hover {
  border-color: #4a5568;
}

.hover\:border-gray-800:hover {
  border-color: #2d3748;
}

.hover\:border-gray-900:hover {
  border-color: #1a202c;
}

.hover\:border-red-100:hover {
  border-color: #fff5f5;
}

.hover\:border-red-200:hover {
  border-color: #fed7d7;
}

.hover\:border-red-300:hover {
  border-color: #feb2b2;
}

.hover\:border-red-400:hover {
  border-color: #fc8181;
}

.hover\:border-red-500:hover {
  border-color: #f56565;
}

.hover\:border-red-600:hover {
  border-color: #e53e3e;
}

.hover\:border-red-700:hover {
  border-color: #c53030;
}

.hover\:border-red-800:hover {
  border-color: #9b2c2c;
}

.hover\:border-red-900:hover {
  border-color: #742a2a;
}

.hover\:border-orange-100:hover {
  border-color: #fffaf0;
}

.hover\:border-orange-200:hover {
  border-color: #feebc8;
}

.hover\:border-orange-300:hover {
  border-color: #fbd38d;
}

.hover\:border-orange-400:hover {
  border-color: #f6ad55;
}

.hover\:border-orange-500:hover {
  border-color: #ed8936;
}

.hover\:border-orange-600:hover {
  border-color: #dd6b20;
}

.hover\:border-orange-700:hover {
  border-color: #c05621;
}

.hover\:border-orange-800:hover {
  border-color: #9c4221;
}

.hover\:border-orange-900:hover {
  border-color: #7b341e;
}

.hover\:border-yellow-100:hover {
  border-color: #fffff0;
}

.hover\:border-yellow-200:hover {
  border-color: #fefcbf;
}

.hover\:border-yellow-300:hover {
  border-color: #faf089;
}

.hover\:border-yellow-400:hover {
  border-color: #f6e05e;
}

.hover\:border-yellow-500:hover {
  border-color: #ecc94b;
}

.hover\:border-yellow-600:hover {
  border-color: #d69e2e;
}

.hover\:border-yellow-700:hover {
  border-color: #b7791f;
}

.hover\:border-yellow-800:hover {
  border-color: #975a16;
}

.hover\:border-yellow-900:hover {
  border-color: #744210;
}

.hover\:border-green-100:hover {
  border-color: #f0fff4;
}

.hover\:border-green-200:hover {
  border-color: #c6f6d5;
}

.hover\:border-green-300:hover {
  border-color: #9ae6b4;
}

.hover\:border-green-400:hover {
  border-color: #68d391;
}

.hover\:border-green-500:hover {
  border-color: #48bb78;
}

.hover\:border-green-600:hover {
  border-color: #38a169;
}

.hover\:border-green-700:hover {
  border-color: #2f855a;
}

.hover\:border-green-800:hover {
  border-color: #276749;
}

.hover\:border-green-900:hover {
  border-color: #22543d;
}

.hover\:border-teal-100:hover {
  border-color: #e6fffa;
}

.hover\:border-teal-200:hover {
  border-color: #b2f5ea;
}

.hover\:border-teal-300:hover {
  border-color: #81e6d9;
}

.hover\:border-teal-400:hover {
  border-color: #4fd1c5;
}

.hover\:border-teal-500:hover {
  border-color: #38b2ac;
}

.hover\:border-teal-600:hover {
  border-color: #319795;
}

.hover\:border-teal-700:hover {
  border-color: #2c7a7b;
}

.hover\:border-teal-800:hover {
  border-color: #285e61;
}

.hover\:border-teal-900:hover {
  border-color: #234e52;
}

.hover\:border-blue-100:hover {
  border-color: #ebf8ff;
}

.hover\:border-blue-200:hover {
  border-color: #bee3f8;
}

.hover\:border-blue-300:hover {
  border-color: #90cdf4;
}

.hover\:border-blue-400:hover {
  border-color: #63b3ed;
}

.hover\:border-blue-500:hover {
  border-color: #4299e1;
}

.hover\:border-blue-600:hover {
  border-color: #3182ce;
}

.hover\:border-blue-700:hover {
  border-color: #2b6cb0;
}

.hover\:border-blue-800:hover {
  border-color: #2c5282;
}

.hover\:border-blue-900:hover {
  border-color: #2a4365;
}

.hover\:border-indigo-100:hover {
  border-color: #ebf4ff;
}

.hover\:border-indigo-200:hover {
  border-color: #c3dafe;
}

.hover\:border-indigo-300:hover {
  border-color: #a3bffa;
}

.hover\:border-indigo-400:hover {
  border-color: #7f9cf5;
}

.hover\:border-indigo-500:hover {
  border-color: #667eea;
}

.hover\:border-indigo-600:hover {
  border-color: #5a67d8;
}

.hover\:border-indigo-700:hover {
  border-color: #4c51bf;
}

.hover\:border-indigo-800:hover {
  border-color: #434190;
}

.hover\:border-indigo-900:hover {
  border-color: #3c366b;
}

.hover\:border-purple-100:hover {
  border-color: #faf5ff;
}

.hover\:border-purple-200:hover {
  border-color: #e9d8fd;
}

.hover\:border-purple-300:hover {
  border-color: #d6bcfa;
}

.hover\:border-purple-400:hover {
  border-color: #b794f4;
}

.hover\:border-purple-500:hover {
  border-color: #9f7aea;
}

.hover\:border-purple-600:hover {
  border-color: #805ad5;
}

.hover\:border-purple-700:hover {
  border-color: #6b46c1;
}

.hover\:border-purple-800:hover {
  border-color: #553c9a;
}

.hover\:border-purple-900:hover {
  border-color: #44337a;
}

.hover\:border-pink-100:hover {
  border-color: #fff5f7;
}

.hover\:border-pink-200:hover {
  border-color: #fed7e2;
}

.hover\:border-pink-300:hover {
  border-color: #fbb6ce;
}

.hover\:border-pink-400:hover {
  border-color: #f687b3;
}

.hover\:border-pink-500:hover {
  border-color: #ed64a6;
}

.hover\:border-pink-600:hover {
  border-color: #d53f8c;
}

.hover\:border-pink-700:hover {
  border-color: #b83280;
}

.hover\:border-pink-800:hover {
  border-color: #97266d;
}

.hover\:border-pink-900:hover {
  border-color: #702459;
}

.focus\:border-transparent:focus {
  border-color: transparent;
}

.focus\:border-black:focus {
  border-color: #000;
}

.focus\:border-white:focus {
  border-color: #fff;
}

.focus\:border-gray-100:focus {
  border-color: #f7fafc;
}

.focus\:border-gray-200:focus {
  border-color: #edf2f7;
}

.focus\:border-gray-300:focus {
  border-color: #e2e8f0;
}

.focus\:border-gray-400:focus {
  border-color: #cbd5e0;
}

.focus\:border-gray-500:focus {
  border-color: #a0aec0;
}

.focus\:border-gray-600:focus {
  border-color: #718096;
}

.focus\:border-gray-700:focus {
  border-color: #4a5568;
}

.focus\:border-gray-800:focus {
  border-color: #2d3748;
}

.focus\:border-gray-900:focus {
  border-color: #1a202c;
}

.focus\:border-red-100:focus {
  border-color: #fff5f5;
}

.focus\:border-red-200:focus {
  border-color: #fed7d7;
}

.focus\:border-red-300:focus {
  border-color: #feb2b2;
}

.focus\:border-red-400:focus {
  border-color: #fc8181;
}

.focus\:border-red-500:focus {
  border-color: #f56565;
}

.focus\:border-red-600:focus {
  border-color: #e53e3e;
}

.focus\:border-red-700:focus {
  border-color: #c53030;
}

.focus\:border-red-800:focus {
  border-color: #9b2c2c;
}

.focus\:border-red-900:focus {
  border-color: #742a2a;
}

.focus\:border-orange-100:focus {
  border-color: #fffaf0;
}

.focus\:border-orange-200:focus {
  border-color: #feebc8;
}

.focus\:border-orange-300:focus {
  border-color: #fbd38d;
}

.focus\:border-orange-400:focus {
  border-color: #f6ad55;
}

.focus\:border-orange-500:focus {
  border-color: #ed8936;
}

.focus\:border-orange-600:focus {
  border-color: #dd6b20;
}

.focus\:border-orange-700:focus {
  border-color: #c05621;
}

.focus\:border-orange-800:focus {
  border-color: #9c4221;
}

.focus\:border-orange-900:focus {
  border-color: #7b341e;
}

.focus\:border-yellow-100:focus {
  border-color: #fffff0;
}

.focus\:border-yellow-200:focus {
  border-color: #fefcbf;
}

.focus\:border-yellow-300:focus {
  border-color: #faf089;
}

.focus\:border-yellow-400:focus {
  border-color: #f6e05e;
}

.focus\:border-yellow-500:focus {
  border-color: #ecc94b;
}

.focus\:border-yellow-600:focus {
  border-color: #d69e2e;
}

.focus\:border-yellow-700:focus {
  border-color: #b7791f;
}

.focus\:border-yellow-800:focus {
  border-color: #975a16;
}

.focus\:border-yellow-900:focus {
  border-color: #744210;
}

.focus\:border-green-100:focus {
  border-color: #f0fff4;
}

.focus\:border-green-200:focus {
  border-color: #c6f6d5;
}

.focus\:border-green-300:focus {
  border-color: #9ae6b4;
}

.focus\:border-green-400:focus {
  border-color: #68d391;
}

.focus\:border-green-500:focus {
  border-color: #48bb78;
}

.focus\:border-green-600:focus {
  border-color: #38a169;
}

.focus\:border-green-700:focus {
  border-color: #2f855a;
}

.focus\:border-green-800:focus {
  border-color: #276749;
}

.focus\:border-green-900:focus {
  border-color: #22543d;
}

.focus\:border-teal-100:focus {
  border-color: #e6fffa;
}

.focus\:border-teal-200:focus {
  border-color: #b2f5ea;
}

.focus\:border-teal-300:focus {
  border-color: #81e6d9;
}

.focus\:border-teal-400:focus {
  border-color: #4fd1c5;
}

.focus\:border-teal-500:focus {
  border-color: #38b2ac;
}

.focus\:border-teal-600:focus {
  border-color: #319795;
}

.focus\:border-teal-700:focus {
  border-color: #2c7a7b;
}

.focus\:border-teal-800:focus {
  border-color: #285e61;
}

.focus\:border-teal-900:focus {
  border-color: #234e52;
}

.focus\:border-blue-100:focus {
  border-color: #ebf8ff;
}

.focus\:border-blue-200:focus {
  border-color: #bee3f8;
}

.focus\:border-blue-300:focus {
  border-color: #90cdf4;
}

.focus\:border-blue-400:focus {
  border-color: #63b3ed;
}

.focus\:border-blue-500:focus {
  border-color: #4299e1;
}

.focus\:border-blue-600:focus {
  border-color: #3182ce;
}

.focus\:border-blue-700:focus {
  border-color: #2b6cb0;
}

.focus\:border-blue-800:focus {
  border-color: #2c5282;
}

.focus\:border-blue-900:focus {
  border-color: #2a4365;
}

.focus\:border-indigo-100:focus {
  border-color: #ebf4ff;
}

.focus\:border-indigo-200:focus {
  border-color: #c3dafe;
}

.focus\:border-indigo-300:focus {
  border-color: #a3bffa;
}

.focus\:border-indigo-400:focus {
  border-color: #7f9cf5;
}

.focus\:border-indigo-500:focus {
  border-color: #667eea;
}

.focus\:border-indigo-600:focus {
  border-color: #5a67d8;
}

.focus\:border-indigo-700:focus {
  border-color: #4c51bf;
}

.focus\:border-indigo-800:focus {
  border-color: #434190;
}

.focus\:border-indigo-900:focus {
  border-color: #3c366b;
}

.focus\:border-purple-100:focus {
  border-color: #faf5ff;
}

.focus\:border-purple-200:focus {
  border-color: #e9d8fd;
}

.focus\:border-purple-300:focus {
  border-color: #d6bcfa;
}

.focus\:border-purple-400:focus {
  border-color: #b794f4;
}

.focus\:border-purple-500:focus {
  border-color: #9f7aea;
}

.focus\:border-purple-600:focus {
  border-color: #805ad5;
}

.focus\:border-purple-700:focus {
  border-color: #6b46c1;
}

.focus\:border-purple-800:focus {
  border-color: #553c9a;
}

.focus\:border-purple-900:focus {
  border-color: #44337a;
}

.focus\:border-pink-100:focus {
  border-color: #fff5f7;
}

.focus\:border-pink-200:focus {
  border-color: #fed7e2;
}

.focus\:border-pink-300:focus {
  border-color: #fbb6ce;
}

.focus\:border-pink-400:focus {
  border-color: #f687b3;
}

.focus\:border-pink-500:focus {
  border-color: #ed64a6;
}

.focus\:border-pink-600:focus {
  border-color: #d53f8c;
}

.focus\:border-pink-700:focus {
  border-color: #b83280;
}

.focus\:border-pink-800:focus {
  border-color: #97266d;
}

.focus\:border-pink-900:focus {
  border-color: #702459;
}

.rounded-none {
  border-radius: 0;
}

.rounded-sm {
  border-radius: 0.125rem;
}

.rounded {
  border-radius: 0.25rem;
}

.rounded-lg {
  border-radius: 0.5rem;
}

.rounded-full {
  border-radius: 9999px;
}

.rounded-t-none {
  border-top-left-radius: 0;
  border-top-right-radius: 0;
}

.rounded-r-none {
  border-top-right-radius: 0;
  border-bottom-right-radius: 0;
}

.rounded-b-none {
  border-bottom-right-radius: 0;
  border-bottom-left-radius: 0;
}

.rounded-l-none {
  border-top-left-radius: 0;
  border-bottom-left-radius: 0;
}

.rounded-t-sm {
  border-top-left-radius: 0.125rem;
  border-top-right-radius: 0.125rem;
}

.rounded-r-sm {
  border-top-right-radius: 0.125rem;
  border-bottom-right-radius: 0.125rem;
}

.rounded-b-sm {
  border-bottom-right-radius: 0.125rem;
  border-bottom-left-radius: 0.125rem;
}

.rounded-l-sm {
  border-top-left-radius: 0.125rem;
  border-bottom-left-radius: 0.125rem;
}

.rounded-t {
  border-top-left-radius: 0.25rem;
  border-top-right-radius: 0.25rem;
}

.rounded-r {
  border-top-right-radius: 0.25rem;
  border-bottom-right-radius: 0.25rem;
}

.rounded-b {
  border-bottom-right-radius: 0.25rem;
  border-bottom-left-radius: 0.25rem;
}

.rounded-l {
  border-top-left-radius: 0.25rem;
  border-bottom-left-radius: 0.25rem;
}

.rounded-t-lg {
  border-top-left-radius: 0.5rem;
  border-top-right-radius: 0.5rem;
}

.rounded-r-lg {
  border-top-right-radius: 0.5rem;
  border-bottom-right-radius: 0.5rem;
}

.rounded-b-lg {
  border-bottom-right-radius: 0.5rem;
  border-bottom-left-radius: 0.5rem;
}

.rounded-l-lg {
  border-top-left-radius: 0.5rem;
  border-bottom-left-radius: 0.5rem;
}

.rounded-t-full {
  border-top-left-radius: 9999px;
  border-top-right-radius: 9999px;
}

.rounded-r-full {
  border-top-right-radius: 9999px;
  border-bottom-right-radius: 9999px;
}

.rounded-b-full {
  border-bottom-right-radius: 9999px;
  border-bottom-left-radius: 9999px;
}

.rounded-l-full {
  border-top-left-radius: 9999px;
  border-bottom-left-radius: 9999px;
}

.rounded-tl-none {
  border-top-left-radius: 0;
}

.rounded-tr-none {
  border-top-right-radius: 0;
}

.rounded-br-none {
  border-bottom-right-radius: 0;
}

.rounded-bl-none {
  border-bottom-left-radius: 0;
}

.rounded-tl-sm {
  border-top-left-radius: 0.125rem;
}

.rounded-tr-sm {
  border-top-right-radius: 0.125rem;
}

.rounded-br-sm {
  border-bottom-right-radius: 0.125rem;
}

.rounded-bl-sm {
  border-bottom-left-radius: 0.125rem;
}

.rounded-tl {
  border-top-left-radius: 0.25rem;
}

.rounded-tr {
  border-top-right-radius: 0.25rem;
}

.rounded-br {
  border-bottom-right-radius: 0.25rem;
}

.rounded-bl {
  border-bottom-left-radius: 0.25rem;
}

.rounded-tl-lg {
  border-top-left-radius: 0.5rem;
}

.rounded-tr-lg {
  border-top-right-radius: 0.5rem;
}

.rounded-br-lg {
  border-bottom-right-radius: 0.5rem;
}

.rounded-bl-lg {
  border-bottom-left-radius: 0.5rem;
}

.rounded-tl-full {
  border-top-left-radius: 9999px;
}

.rounded-tr-full {
  border-top-right-radius: 9999px;
}

.rounded-br-full {
  border-bottom-right-radius: 9999px;
}

.rounded-bl-full {
  border-bottom-left-radius: 9999px;
}

.border-solid {
  border-style: solid;
}

.border-dashed {
  border-style: dashed;
}

.border-dotted {
  border-style: dotted;
}

.border-double {
  border-style: double;
}

.border-none {
  border-style: none;
}

.border-0 {
  border-width: 0;
}

.border-2 {
  border-width: 2px;
}

.border-4 {
  border-width: 4px;
}

.border-8 {
  border-width: 8px;
}

.border {
  border-width: 1px;
}

.border-t-0 {
  border-top-width: 0;
}

.border-r-0 {
  border-right-width: 0;
}

.border-b-0 {
  border-bottom-width: 0;
}

.border-l-0 {
  border-left-width: 0;
}

.border-t-2 {
  border-top-width: 2px;
}

.border-r-2 {
  border-right-width: 2px;
}

.border-b-2 {
  border-bottom-width: 2px;
}

.border-l-2 {
  border-left-width: 2px;
}

.border-t-4 {
  border-top-width: 4px;
}

.border-r-4 {
  border-right-width: 4px;
}

.border-b-4 {
  border-bottom-width: 4px;
}

.border-l-4 {
  border-left-width: 4px;
}

.border-t-8 {
  border-top-width: 8px;
}

.border-r-8 {
  border-right-width: 8px;
}

.border-b-8 {
  border-bottom-width: 8px;
}

.border-l-8 {
  border-left-width: 8px;
}

.border-t {
  border-top-width: 1px;
}

.border-r {
  border-right-width: 1px;
}

.border-b {
  border-bottom-width: 1px;
}

.border-l {
  border-left-width: 1px;
}

.cursor-auto {
  cursor: auto;
}

.cursor-default {
  cursor: default;
}

.cursor-pointer {
  cursor: pointer;
}

.cursor-wait {
  cursor: wait;
}

.cursor-text {
  cursor: text;
}

.cursor-move {
  cursor: move;
}

.cursor-not-allowed {
  cursor: not-allowed;
}

.block {
  display: block;
}

.inline-block {
  display: inline-block;
}

.inline {
  display: inline;
}

.flex {
  display: -webkit-box;
  display: flex;
}

.inline-flex {
  display: -webkit-inline-box;
  display: inline-flex;
}

.table {
  display: table;
}

.table-row {
  display: table-row;
}

.table-cell {
  display: table-cell;
}

.hidden {
  display: none;
}

.flex-row {
  -webkit-box-orient: horizontal;
  -webkit-box-direction: normal;
          flex-direction: row;
}

.flex-row-reverse {
  -webkit-box-orient: horizontal;
  -webkit-box-direction: reverse;
          flex-direction: row-reverse;
}

.flex-col {
  -webkit-box-orient: vertical;
  -webkit-box-direction: normal;
          flex-direction: column;
}

.flex-col-reverse {
  -webkit-box-orient: vertical;
  -webkit-box-direction: reverse;
          flex-direction: column-reverse;
}

.flex-wrap {
  flex-wrap: wrap;
}

.flex-wrap-reverse {
  flex-wrap: wrap-reverse;
}

.flex-no-wrap {
  flex-wrap: nowrap;
}

.items-start {
  -webkit-box-align: start;
          align-items: flex-start;
}

.items-end {
  -webkit-box-align: end;
          align-items: flex-end;
}

.items-center {
  -webkit-box-align: center;
          align-items: center;
}

.items-baseline {
  -webkit-box-align: baseline;
          align-items: baseline;
}

.items-stretch {
  -webkit-box-align: stretch;
          align-items: stretch;
}

.self-auto {
  align-self: auto;
}

.self-start {
  align-self: flex-start;
}

.self-end {
  align-self: flex-end;
}

.self-center {
  align-self: center;
}

.self-stretch {
  align-self: stretch;
}

.justify-start {
  -webkit-box-pack: start;
          justify-content: flex-start;
}

.justify-end {
  -webkit-box-pack: end;
          justify-content: flex-end;
}

.justify-center {
  -webkit-box-pack: center;
          justify-content: center;
}

.justify-between {
  -webkit-box-pack: justify;
          justify-content: space-between;
}

.justify-around {
  justify-content: space-around;
}

.content-center {
  align-content: center;
}

.content-start {
  align-content: flex-start;
}

.content-end {
  align-content: flex-end;
}

.content-between {
  align-content: space-between;
}

.content-around {
  align-content: space-around;
}

.flex-1 {
  -webkit-box-flex: 1;
          flex: 1 1 0%;
}

.flex-auto {
  -webkit-box-flex: 1;
          flex: 1 1 auto;
}

.flex-initial {
  -webkit-box-flex: 0;
          flex: 0 1 auto;
}

.flex-none {
  -webkit-box-flex: 0;
          flex: none;
}

.flex-grow-0 {
  -webkit-box-flex: 0;
          flex-grow: 0;
}

.flex-grow {
  -webkit-box-flex: 1;
          flex-grow: 1;
}

.flex-shrink-0 {
  flex-shrink: 0;
}

.flex-shrink {
  flex-shrink: 1;
}

.order-1 {
  -webkit-box-ordinal-group: 2;
          order: 1;
}

.order-2 {
  -webkit-box-ordinal-group: 3;
          order: 2;
}

.order-3 {
  -webkit-box-ordinal-group: 4;
          order: 3;
}

.order-4 {
  -webkit-box-ordinal-group: 5;
          order: 4;
}

.order-5 {
  -webkit-box-ordinal-group: 6;
          order: 5;
}

.order-6 {
  -webkit-box-ordinal-group: 7;
          order: 6;
}

.order-7 {
  -webkit-box-ordinal-group: 8;
          order: 7;
}

.order-8 {
  -webkit-box-ordinal-group: 9;
          order: 8;
}

.order-9 {
  -webkit-box-ordinal-group: 10;
          order: 9;
}

.order-10 {
  -webkit-box-ordinal-group: 11;
          order: 10;
}

.order-11 {
  -webkit-box-ordinal-group: 12;
          order: 11;
}

.order-12 {
  -webkit-box-ordinal-group: 13;
          order: 12;
}

.order-first {
  -webkit-box-ordinal-group: -9998;
          order: -9999;
}

.order-last {
  -webkit-box-ordinal-group: 10000;
          order: 9999;
}

.order-none {
  -webkit-box-ordinal-group: 1;
          order: 0;
}

.float-right {
  float: right;
}

.float-left {
  float: left;
}

.float-none {
  float: none;
}

.clearfix:after {
  content: "";
  display: table;
  clear: both;
}

.font-sans {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji";
}

.font-serif {
  font-family: Georgia, Cambria, "Times New Roman", Times, serif;
}

.font-mono {
  font-family: Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
}

.font-hairline {
  font-weight: 100;
}

.font-thin {
  font-weight: 200;
}

.font-light {
  font-weight: 300;
}

.font-normal {
  font-weight: 400;
}

.font-medium {
  font-weight: 500;
}

.font-semibold {
  font-weight: 600;
}

.font-bold {
  font-weight: 700;
}

.font-extrabold {
  font-weight: 800;
}

.font-black {
  font-weight: 900;
}

.hover\:font-hairline:hover {
  font-weight: 100;
}

.hover\:font-thin:hover {
  font-weight: 200;
}

.hover\:font-light:hover {
  font-weight: 300;
}

.hover\:font-normal:hover {
  font-weight: 400;
}

.hover\:font-medium:hover {
  font-weight: 500;
}

.hover\:font-semibold:hover {
  font-weight: 600;
}

.hover\:font-bold:hover {
  font-weight: 700;
}

.hover\:font-extrabold:hover {
  font-weight: 800;
}

.hover\:font-black:hover {
  font-weight: 900;
}

.focus\:font-hairline:focus {
  font-weight: 100;
}

.focus\:font-thin:focus {
  font-weight: 200;
}

.focus\:font-light:focus {
  font-weight: 300;
}

.focus\:font-normal:focus {
  font-weight: 400;
}

.focus\:font-medium:focus {
  font-weight: 500;
}

.focus\:font-semibold:focus {
  font-weight: 600;
}

.focus\:font-bold:focus {
  font-weight: 700;
}

.focus\:font-extrabold:focus {
  font-weight: 800;
}

.focus\:font-black:focus {
  font-weight: 900;
}

.h-0 {
  height: 0;
}

.h-1 {
  height: 0.25rem;
}

.h-2 {
  height: 0.5rem;
}

.h-3 {
  height: 0.75rem;
}

.h-4 {
  height: 1rem;
}

.h-5 {
  height: 1.25rem;
}

.h-6 {
  height: 1.5rem;
}

.h-8 {
  height: 2rem;
}

.h-10 {
  height: 2.5rem;
}

.h-12 {
  height: 3rem;
}

.h-16 {
  height: 4rem;
}

.h-20 {
  height: 5rem;
}

.h-24 {
  height: 6rem;
}

.h-32 {
  height: 8rem;
}

.h-40 {
  height: 10rem;
}

.h-48 {
  height: 12rem;
}

.h-56 {
  height: 14rem;
}

.h-64 {
  height: 16rem;
}

.h-auto {
  height: auto;
}

.h-px {
  height: 1px;
}

.h-full {
  height: 100%;
}

.h-screen {
  height: 100vh;
}

.leading-none {
  line-height: 1;
}

.leading-tight {
  line-height: 1.25;
}

.leading-snug {
  line-height: 1.375;
}

.leading-normal {
  line-height: 1.5;
}

.leading-relaxed {
  line-height: 1.625;
}

.leading-loose {
  line-height: 2;
}

.list-inside {
  list-style-position: inside;
}

.list-outside {
  list-style-position: outside;
}

.list-none {
  list-style-type: none;
}

.list-disc {
  list-style-type: disc;
}

.list-decimal {
  list-style-type: decimal;
}

.m-0 {
  margin: 0;
}

.m-1 {
  margin: 0.25rem;
}

.m-2 {
  margin: 0.5rem;
}

.m-3 {
  margin: 0.75rem;
}

.m-4 {
  margin: 1rem;
}

.m-5 {
  margin: 1.25rem;
}

.m-6 {
  margin: 1.5rem;
}

.m-8 {
  margin: 2rem;
}

.m-10 {
  margin: 2.5rem;
}

.m-12 {
  margin: 3rem;
}

.m-16 {
  margin: 4rem;
}

.m-20 {
  margin: 5rem;
}

.m-24 {
  margin: 6rem;
}

.m-32 {
  margin: 8rem;
}

.m-40 {
  margin: 10rem;
}

.m-48 {
  margin: 12rem;
}

.m-56 {
  margin: 14rem;
}

.m-64 {
  margin: 16rem;
}

.m-auto {
  margin: auto;
}

.m-px {
  margin: 1px;
}

.-m-1 {
  margin: -0.25rem;
}

.-m-2 {
  margin: -0.5rem;
}

.-m-3 {
  margin: -0.75rem;
}

.-m-4 {
  margin: -1rem;
}

.-m-5 {
  margin: -1.25rem;
}

.-m-6 {
  margin: -1.5rem;
}

.-m-8 {
  margin: -2rem;
}

.-m-10 {
  margin: -2.5rem;
}

.-m-12 {
  margin: -3rem;
}

.-m-16 {
  margin: -4rem;
}

.-m-20 {
  margin: -5rem;
}

.-m-24 {
  margin: -6rem;
}

.-m-32 {
  margin: -8rem;
}

.-m-40 {
  margin: -10rem;
}

.-m-48 {
  margin: -12rem;
}

.-m-56 {
  margin: -14rem;
}

.-m-64 {
  margin: -16rem;
}

.-m-px {
  margin: -1px;
}

.my-0 {
  margin-top: 0;
  margin-bottom: 0;
}

.mx-0 {
  margin-left: 0;
  margin-right: 0;
}

.my-1 {
  margin-top: 0.25rem;
  margin-bottom: 0.25rem;
}

.mx-1 {
  margin-left: 0.25rem;
  margin-right: 0.25rem;
}

.my-2 {
  margin-top: 0.5rem;
  margin-bottom: 0.5rem;
}

.mx-2 {
  margin-left: 0.5rem;
  margin-right: 0.5rem;
}

.my-3 {
  margin-top: 0.75rem;
  margin-bottom: 0.75rem;
}

.mx-3 {
  margin-left: 0.75rem;
  margin-right: 0.75rem;
}

.my-4 {
  margin-top: 1rem;
  margin-bottom: 1rem;
}

.mx-4 {
  margin-left: 1rem;
  margin-right: 1rem;
}

.my-5 {
  margin-top: 1.25rem;
  margin-bottom: 1.25rem;
}

.mx-5 {
  margin-left: 1.25rem;
  margin-right: 1.25rem;
}

.my-6 {
  margin-top: 1.5rem;
  margin-bottom: 1.5rem;
}

.mx-6 {
  margin-left: 1.5rem;
  margin-right: 1.5rem;
}

.my-8 {
  margin-top: 2rem;
  margin-bottom: 2rem;
}

.mx-8 {
  margin-left: 2rem;
  margin-right: 2rem;
}

.my-10 {
  margin-top: 2.5rem;
  margin-bottom: 2.5rem;
}

.mx-10 {
  margin-left: 2.5rem;
  margin-right: 2.5rem;
}

.my-12 {
  margin-top: 3rem;
  margin-bottom: 3rem;
}

.mx-12 {
  margin-left: 3rem;
  margin-right: 3rem;
}

.my-16 {
  margin-top: 4rem;
  margin-bottom: 4rem;
}

.mx-16 {
  margin-left: 4rem;
  margin-right: 4rem;
}

.my-20 {
  margin-top: 5rem;
  margin-bottom: 5rem;
}

.mx-20 {
  margin-left: 5rem;
  margin-right: 5rem;
}

.my-24 {
  margin-top: 6rem;
  margin-bottom: 6rem;
}

.mx-24 {
  margin-left: 6rem;
  margin-right: 6rem;
}

.my-32 {
  margin-top: 8rem;
  margin-bottom: 8rem;
}

.mx-32 {
  margin-left: 8rem;
  margin-right: 8rem;
}

.my-40 {
  margin-top: 10rem;
  margin-bottom: 10rem;
}

.mx-40 {
  margin-left: 10rem;
  margin-right: 10rem;
}

.my-48 {
  margin-top: 12rem;
  margin-bottom: 12rem;
}

.mx-48 {
  margin-left: 12rem;
  margin-right: 12rem;
}

.my-56 {
  margin-top: 14rem;
  margin-bottom: 14rem;
}

.mx-56 {
  margin-left: 14rem;
  margin-right: 14rem;
}

.my-64 {
  margin-top: 16rem;
  margin-bottom: 16rem;
}

.mx-64 {
  margin-left: 16rem;
  margin-right: 16rem;
}

.my-auto {
  margin-top: auto;
  margin-bottom: auto;
}

.mx-auto {
  margin-left: auto;
  margin-right: auto;
}

.my-px {
  margin-top: 1px;
  margin-bottom: 1px;
}

.mx-px {
  margin-left: 1px;
  margin-right: 1px;
}

.-my-1 {
  margin-top: -0.25rem;
  margin-bottom: -0.25rem;
}

.-mx-1 {
  margin-left: -0.25rem;
  margin-right: -0.25rem;
}

.-my-2 {
  margin-top: -0.5rem;
  margin-bottom: -0.5rem;
}

.-mx-2 {
  margin-left: -0.5rem;
  margin-right: -0.5rem;
}

.-my-3 {
  margin-top: -0.75rem;
  margin-bottom: -0.75rem;
}

.-mx-3 {
  margin-left: -0.75rem;
  margin-right: -0.75rem;
}

.-my-4 {
  margin-top: -1rem;
  margin-bottom: -1rem;
}

.-mx-4 {
  margin-left: -1rem;
  margin-right: -1rem;
}

.-my-5 {
  margin-top: -1.25rem;
  margin-bottom: -1.25rem;
}

.-mx-5 {
  margin-left: -1.25rem;
  margin-right: -1.25rem;
}

.-my-6 {
  margin-top: -1.5rem;
  margin-bottom: -1.5rem;
}

.-mx-6 {
  margin-left: -1.5rem;
  margin-right: -1.5rem;
}

.-my-8 {
  margin-top: -2rem;
  margin-bottom: -2rem;
}

.-mx-8 {
  margin-left: -2rem;
  margin-right: -2rem;
}

.-my-10 {
  margin-top: -2.5rem;
  margin-bottom: -2.5rem;
}

.-mx-10 {
  margin-left: -2.5rem;
  margin-right: -2.5rem;
}

.-my-12 {
  margin-top: -3rem;
  margin-bottom: -3rem;
}

.-mx-12 {
  margin-left: -3rem;
  margin-right: -3rem;
}

.-my-16 {
  margin-top: -4rem;
  margin-bottom: -4rem;
}

.-mx-16 {
  margin-left: -4rem;
  margin-right: -4rem;
}

.-my-20 {
  margin-top: -5rem;
  margin-bottom: -5rem;
}

.-mx-20 {
  margin-left: -5rem;
  margin-right: -5rem;
}

.-my-24 {
  margin-top: -6rem;
  margin-bottom: -6rem;
}

.-mx-24 {
  margin-left: -6rem;
  margin-right: -6rem;
}

.-my-32 {
  margin-top: -8rem;
  margin-bottom: -8rem;
}

.-mx-32 {
  margin-left: -8rem;
  margin-right: -8rem;
}

.-my-40 {
  margin-top: -10rem;
  margin-bottom: -10rem;
}

.-mx-40 {
  margin-left: -10rem;
  margin-right: -10rem;
}

.-my-48 {
  margin-top: -12rem;
  margin-bottom: -12rem;
}

.-mx-48 {
  margin-left: -12rem;
  margin-right: -12rem;
}

.-my-56 {
  margin-top: -14rem;
  margin-bottom: -14rem;
}

.-mx-56 {
  margin-left: -14rem;
  margin-right: -14rem;
}

.-my-64 {
  margin-top: -16rem;
  margin-bottom: -16rem;
}

.-mx-64 {
  margin-left: -16rem;
  margin-right: -16rem;
}

.-my-px {
  margin-top: -1px;
  margin-bottom: -1px;
}

.-mx-px {
  margin-left: -1px;
  margin-right: -1px;
}

.mt-0 {
  margin-top: 0;
}

.mr-0 {
  margin-right: 0;
}

.mb-0 {
  margin-bottom: 0;
}

.ml-0 {
  margin-left: 0;
}

.mt-1 {
  margin-top: 0.25rem;
}

.mr-1 {
  margin-right: 0.25rem;
}

.mb-1 {
  margin-bottom: 0.25rem;
}

.ml-1 {
  margin-left: 0.25rem;
}

.mt-2 {
  margin-top: 0.5rem;
}

.mr-2 {
  margin-right: 0.5rem;
}

.mb-2 {
  margin-bottom: 0.5rem;
}

.ml-2 {
  margin-left: 0.5rem;
}

.mt-3 {
  margin-top: 0.75rem;
}

.mr-3 {
  margin-right: 0.75rem;
}

.mb-3 {
  margin-bottom: 0.75rem;
}

.ml-3 {
  margin-left: 0.75rem;
}

.mt-4 {
  margin-top: 1rem;
}

.mr-4 {
  margin-right: 1rem;
}

.mb-4 {
  margin-bottom: 1rem;
}

.ml-4 {
  margin-left: 1rem;
}

.mt-5 {
  margin-top: 1.25rem;
}

.mr-5 {
  margin-right: 1.25rem;
}

.mb-5 {
  margin-bottom: 1.25rem;
}

.ml-5 {
  margin-left: 1.25rem;
}

.mt-6 {
  margin-top: 1.5rem;
}

.mr-6 {
  margin-right: 1.5rem;
}

.mb-6 {
  margin-bottom: 1.5rem;
}

.ml-6 {
  margin-left: 1.5rem;
}

.mt-8 {
  margin-top: 2rem;
}

.mr-8 {
  margin-right: 2rem;
}

.mb-8 {
  margin-bottom: 2rem;
}

.ml-8 {
  margin-left: 2rem;
}

.mt-10 {
  margin-top: 2.5rem;
}

.mr-10 {
  margin-right: 2.5rem;
}

.mb-10 {
  margin-bottom: 2.5rem;
}

.ml-10 {
  margin-left: 2.5rem;
}

.mt-12 {
  margin-top: 3rem;
}

.mr-12 {
  margin-right: 3rem;
}

.mb-12 {
  margin-bottom: 3rem;
}

.ml-12 {
  margin-left: 3rem;
}

.mt-16 {
  margin-top: 4rem;
}

.mr-16 {
  margin-right: 4rem;
}

.mb-16 {
  margin-bottom: 4rem;
}

.ml-16 {
  margin-left: 4rem;
}

.mt-20 {
  margin-top: 5rem;
}

.mr-20 {
  margin-right: 5rem;
}

.mb-20 {
  margin-bottom: 5rem;
}

.ml-20 {
  margin-left: 5rem;
}

.mt-24 {
  margin-top: 6rem;
}

.mr-24 {
  margin-right: 6rem;
}

.mb-24 {
  margin-bottom: 6rem;
}

.ml-24 {
  margin-left: 6rem;
}

.mt-32 {
  margin-top: 8rem;
}

.mr-32 {
  margin-right: 8rem;
}

.mb-32 {
  margin-bottom: 8rem;
}

.ml-32 {
  margin-left: 8rem;
}

.mt-40 {
  margin-top: 10rem;
}

.mr-40 {
  margin-right: 10rem;
}

.mb-40 {
  margin-bottom: 10rem;
}

.ml-40 {
  margin-left: 10rem;
}

.mt-48 {
  margin-top: 12rem;
}

.mr-48 {
  margin-right: 12rem;
}

.mb-48 {
  margin-bottom: 12rem;
}

.ml-48 {
  margin-left: 12rem;
}

.mt-56 {
  margin-top: 14rem;
}

.mr-56 {
  margin-right: 14rem;
}

.mb-56 {
  margin-bottom: 14rem;
}

.ml-56 {
  margin-left: 14rem;
}

.mt-64 {
  margin-top: 16rem;
}

.mr-64 {
  margin-right: 16rem;
}

.mb-64 {
  margin-bottom: 16rem;
}

.ml-64 {
  margin-left: 16rem;
}

.mt-auto {
  margin-top: auto;
}

.mr-auto {
  margin-right: auto;
}

.mb-auto {
  margin-bottom: auto;
}

.ml-auto {
  margin-left: auto;
}

.mt-px {
  margin-top: 1px;
}

.mr-px {
  margin-right: 1px;
}

.mb-px {
  margin-bottom: 1px;
}

.ml-px {
  margin-left: 1px;
}

.-mt-1 {
  margin-top: -0.25rem;
}

.-mr-1 {
  margin-right: -0.25rem;
}

.-mb-1 {
  margin-bottom: -0.25rem;
}

.-ml-1 {
  margin-left: -0.25rem;
}

.-mt-2 {
  margin-top: -0.5rem;
}

.-mr-2 {
  margin-right: -0.5rem;
}

.-mb-2 {
  margin-bottom: -0.5rem;
}

.-ml-2 {
  margin-left: -0.5rem;
}

.-mt-3 {
  margin-top: -0.75rem;
}

.-mr-3 {
  margin-right: -0.75rem;
}

.-mb-3 {
  margin-bottom: -0.75rem;
}

.-ml-3 {
  margin-left: -0.75rem;
}

.-mt-4 {
  margin-top: -1rem;
}

.-mr-4 {
  margin-right: -1rem;
}

.-mb-4 {
  margin-bottom: -1rem;
}

.-ml-4 {
  margin-left: -1rem;
}

.-mt-5 {
  margin-top: -1.25rem;
}

.-mr-5 {
  margin-right: -1.25rem;
}

.-mb-5 {
  margin-bottom: -1.25rem;
}

.-ml-5 {
  margin-left: -1.25rem;
}

.-mt-6 {
  margin-top: -1.5rem;
}

.-mr-6 {
  margin-right: -1.5rem;
}

.-mb-6 {
  margin-bottom: -1.5rem;
}

.-ml-6 {
  margin-left: -1.5rem;
}

.-mt-8 {
  margin-top: -2rem;
}

.-mr-8 {
  margin-right: -2rem;
}

.-mb-8 {
  margin-bottom: -2rem;
}

.-ml-8 {
  margin-left: -2rem;
}

.-mt-10 {
  margin-top: -2.5rem;
}

.-mr-10 {
  margin-right: -2.5rem;
}

.-mb-10 {
  margin-bottom: -2.5rem;
}

.-ml-10 {
  margin-left: -2.5rem;
}

.-mt-12 {
  margin-top: -3rem;
}

.-mr-12 {
  margin-right: -3rem;
}

.-mb-12 {
  margin-bottom: -3rem;
}

.-ml-12 {
  margin-left: -3rem;
}

.-mt-16 {
  margin-top: -4rem;
}

.-mr-16 {
  margin-right: -4rem;
}

.-mb-16 {
  margin-bottom: -4rem;
}

.-ml-16 {
  margin-left: -4rem;
}

.-mt-20 {
  margin-top: -5rem;
}

.-mr-20 {
  margin-right: -5rem;
}

.-mb-20 {
  margin-bottom: -5rem;
}

.-ml-20 {
  margin-left: -5rem;
}

.-mt-24 {
  margin-top: -6rem;
}

.-mr-24 {
  margin-right: -6rem;
}

.-mb-24 {
  margin-bottom: -6rem;
}

.-ml-24 {
  margin-left: -6rem;
}

.-mt-32 {
  margin-top: -8rem;
}

.-mr-32 {
  margin-right: -8rem;
}

.-mb-32 {
  margin-bottom: -8rem;
}

.-ml-32 {
  margin-left: -8rem;
}

.-mt-40 {
  margin-top: -10rem;
}

.-mr-40 {
  margin-right: -10rem;
}

.-mb-40 {
  margin-bottom: -10rem;
}

.-ml-40 {
  margin-left: -10rem;
}

.-mt-48 {
  margin-top: -12rem;
}

.-mr-48 {
  margin-right: -12rem;
}

.-mb-48 {
  margin-bottom: -12rem;
}

.-ml-48 {
  margin-left: -12rem;
}

.-mt-56 {
  margin-top: -14rem;
}

.-mr-56 {
  margin-right: -14rem;
}

.-mb-56 {
  margin-bottom: -14rem;
}

.-ml-56 {
  margin-left: -14rem;
}

.-mt-64 {
  margin-top: -16rem;
}

.-mr-64 {
  margin-right: -16rem;
}

.-mb-64 {
  margin-bottom: -16rem;
}

.-ml-64 {
  margin-left: -16rem;
}

.-mt-px {
  margin-top: -1px;
}

.-mr-px {
  margin-right: -1px;
}

.-mb-px {
  margin-bottom: -1px;
}

.-ml-px {
  margin-left: -1px;
}

.max-h-full {
  max-height: 100%;
}

.max-h-screen {
  max-height: 100vh;
}

.max-w-xs {
  max-width: 20rem;
}

.max-w-sm {
  max-width: 24rem;
}

.max-w-md {
  max-width: 28rem;
}

.max-w-lg {
  max-width: 32rem;
}

.max-w-xl {
  max-width: 36rem;
}

.max-w-2xl {
  max-width: 42rem;
}

.max-w-3xl {
  max-width: 48rem;
}

.max-w-4xl {
  max-width: 56rem;
}

.max-w-5xl {
  max-width: 64rem;
}

.max-w-6xl {
  max-width: 72rem;
}

.max-w-full {
  max-width: 100%;
}

.min-h-0 {
  min-height: 0;
}

.min-h-full {
  min-height: 100%;
}

.min-h-screen {
  min-height: 100vh;
}

.min-w-0 {
  min-width: 0;
}

.min-w-full {
  min-width: 100%;
}

.object-contain {
  -o-object-fit: contain;
     object-fit: contain;
}

.object-cover {
  -o-object-fit: cover;
     object-fit: cover;
}

.object-fill {
  -o-object-fit: fill;
     object-fit: fill;
}

.object-none {
  -o-object-fit: none;
     object-fit: none;
}

.object-scale-down {
  -o-object-fit: scale-down;
     object-fit: scale-down;
}

.object-bottom {
  -o-object-position: bottom;
     object-position: bottom;
}

.object-center {
  -o-object-position: center;
     object-position: center;
}

.object-left {
  -o-object-position: left;
     object-position: left;
}

.object-left-bottom {
  -o-object-position: left bottom;
     object-position: left bottom;
}

.object-left-top {
  -o-object-position: left top;
     object-position: left top;
}

.object-right {
  -o-object-position: right;
     object-position: right;
}

.object-right-bottom {
  -o-object-position: right bottom;
     object-position: right bottom;
}

.object-right-top {
  -o-object-position: right top;
     object-position: right top;
}

.object-top {
  -o-object-position: top;
     object-position: top;
}

.opacity-0 {
  opacity: 0;
}

.opacity-25 {
  opacity: 0.25;
}

.opacity-50 {
  opacity: 0.5;
}

.opacity-75 {
  opacity: 0.75;
}

.opacity-100 {
  opacity: 1;
}

.hover\:opacity-0:hover {
  opacity: 0;
}

.hover\:opacity-25:hover {
  opacity: 0.25;
}

.hover\:opacity-50:hover {
  opacity: 0.5;
}

.hover\:opacity-75:hover {
  opacity: 0.75;
}

.hover\:opacity-100:hover {
  opacity: 1;
}

.focus\:opacity-0:focus {
  opacity: 0;
}

.focus\:opacity-25:focus {
  opacity: 0.25;
}

.focus\:opacity-50:focus {
  opacity: 0.5;
}

.focus\:opacity-75:focus {
  opacity: 0.75;
}

.focus\:opacity-100:focus {
  opacity: 1;
}

.outline-none {
  outline: 0;
}

.focus\:outline-none:focus {
  outline: 0;
}

.overflow-auto {
  overflow: auto;
}

.overflow-hidden {
  overflow: hidden;
}

.overflow-visible {
  overflow: visible;
}

.overflow-scroll {
  overflow: scroll;
}

.overflow-x-auto {
  overflow-x: auto;
}

.overflow-y-auto {
  overflow-y: auto;
}

.overflow-x-hidden {
  overflow-x: hidden;
}

.overflow-y-hidden {
  overflow-y: hidden;
}

.overflow-x-visible {
  overflow-x: visible;
}

.overflow-y-visible {
  overflow-y: visible;
}

.overflow-x-scroll {
  overflow-x: scroll;
}

.overflow-y-scroll {
  overflow-y: scroll;
}

.scrolling-touch {
  -webkit-overflow-scrolling: touch;
}

.scrolling-auto {
  -webkit-overflow-scrolling: auto;
}

.p-0 {
  padding: 0;
}

.p-1 {
  padding: 0.25rem;
}

.p-2 {
  padding: 0.5rem;
}

.p-3 {
  padding: 0.75rem;
}

.p-4 {
  padding: 1rem;
}

.p-5 {
  padding: 1.25rem;
}

.p-6 {
  padding: 1.5rem;
}

.p-8 {
  padding: 2rem;
}

.p-10 {
  padding: 2.5rem;
}

.p-12 {
  padding: 3rem;
}

.p-16 {
  padding: 4rem;
}

.p-20 {
  padding: 5rem;
}

.p-24 {
  padding: 6rem;
}

.p-32 {
  padding: 8rem;
}

.p-40 {
  padding: 10rem;
}

.p-48 {
  padding: 12rem;
}

.p-56 {
  padding: 14rem;
}

.p-64 {
  padding: 16rem;
}

.p-px {
  padding: 1px;
}

.py-0 {
  padding-top: 0;
  padding-bottom: 0;
}

.px-0 {
  padding-left: 0;
  padding-right: 0;
}

.py-1 {
  padding-top: 0.25rem;
  padding-bottom: 0.25rem;
}

.px-1 {
  padding-left: 0.25rem;
  padding-right: 0.25rem;
}

.py-2 {
  padding-top: 0.5rem;
  padding-bottom: 0.5rem;
}

.px-2 {
  padding-left: 0.5rem;
  padding-right: 0.5rem;
}

.py-3 {
  padding-top: 0.75rem;
  padding-bottom: 0.75rem;
}

.px-3 {
  padding-left: 0.75rem;
  padding-right: 0.75rem;
}

.py-4 {
  padding-top: 1rem;
  padding-bottom: 1rem;
}

.px-4 {
  padding-left: 1rem;
  padding-right: 1rem;
}

.py-5 {
  padding-top: 1.25rem;
  padding-bottom: 1.25rem;
}

.px-5 {
  padding-left: 1.25rem;
  padding-right: 1.25rem;
}

.py-6 {
  padding-top: 1.5rem;
  padding-bottom: 1.5rem;
}

.px-6 {
  padding-left: 1.5rem;
  padding-right: 1.5rem;
}

.py-8 {
  padding-top: 2rem;
  padding-bottom: 2rem;
}

.px-8 {
  padding-left: 2rem;
  padding-right: 2rem;
}

.py-10 {
  padding-top: 2.5rem;
  padding-bottom: 2.5rem;
}

.px-10 {
  padding-left: 2.5rem;
  padding-right: 2.5rem;
}

.py-12 {
  padding-top: 3rem;
  padding-bottom: 3rem;
}

.px-12 {
  padding-left: 3rem;
  padding-right: 3rem;
}

.py-16 {
  padding-top: 4rem;
  padding-bottom: 4rem;
}

.px-16 {
  padding-left: 4rem;
  padding-right: 4rem;
}

.py-20 {
  padding-top: 5rem;
  padding-bottom: 5rem;
}

.px-20 {
  padding-left: 5rem;
  padding-right: 5rem;
}

.py-24 {
  padding-top: 6rem;
  padding-bottom: 6rem;
}

.px-24 {
  padding-left: 6rem;
  padding-right: 6rem;
}

.py-32 {
  padding-top: 8rem;
  padding-bottom: 8rem;
}

.px-32 {
  padding-left: 8rem;
  padding-right: 8rem;
}

.py-40 {
  padding-top: 10rem;
  padding-bottom: 10rem;
}

.px-40 {
  padding-left: 10rem;
  padding-right: 10rem;
}

.py-48 {
  padding-top: 12rem;
  padding-bottom: 12rem;
}

.px-48 {
  padding-left: 12rem;
  padding-right: 12rem;
}

.py-56 {
  padding-top: 14rem;
  padding-bottom: 14rem;
}

.px-56 {
  padding-left: 14rem;
  padding-right: 14rem;
}

.py-64 {
  padding-top: 16rem;
  padding-bottom: 16rem;
}

.px-64 {
  padding-left: 16rem;
  padding-right: 16rem;
}

.py-px {
  padding-top: 1px;
  padding-bottom: 1px;
}

.px-px {
  padding-left: 1px;
  padding-right: 1px;
}

.pt-0 {
  padding-top: 0;
}

.pr-0 {
  padding-right: 0;
}

.pb-0 {
  padding-bottom: 0;
}

.pl-0 {
  padding-left: 0;
}

.pt-1 {
  padding-top: 0.25rem;
}

.pr-1 {
  padding-right: 0.25rem;
}

.pb-1 {
  padding-bottom: 0.25rem;
}

.pl-1 {
  padding-left: 0.25rem;
}

.pt-2 {
  padding-top: 0.5rem;
}

.pr-2 {
  padding-right: 0.5rem;
}

.pb-2 {
  padding-bottom: 0.5rem;
}

.pl-2 {
  padding-left: 0.5rem;
}

.pt-3 {
  padding-top: 0.75rem;
}

.pr-3 {
  padding-right: 0.75rem;
}

.pb-3 {
  padding-bottom: 0.75rem;
}

.pl-3 {
  padding-left: 0.75rem;
}

.pt-4 {
  padding-top: 1rem;
}

.pr-4 {
  padding-right: 1rem;
}

.pb-4 {
  padding-bottom: 1rem;
}

.pl-4 {
  padding-left: 1rem;
}

.pt-5 {
  padding-top: 1.25rem;
}

.pr-5 {
  padding-right: 1.25rem;
}

.pb-5 {
  padding-bottom: 1.25rem;
}

.pl-5 {
  padding-left: 1.25rem;
}

.pt-6 {
  padding-top: 1.5rem;
}

.pr-6 {
  padding-right: 1.5rem;
}

.pb-6 {
  padding-bottom: 1.5rem;
}

.pl-6 {
  padding-left: 1.5rem;
}

.pt-8 {
  padding-top: 2rem;
}

.pr-8 {
  padding-right: 2rem;
}

.pb-8 {
  padding-bottom: 2rem;
}

.pl-8 {
  padding-left: 2rem;
}

.pt-10 {
  padding-top: 2.5rem;
}

.pr-10 {
  padding-right: 2.5rem;
}

.pb-10 {
  padding-bottom: 2.5rem;
}

.pl-10 {
  padding-left: 2.5rem;
}

.pt-12 {
  padding-top: 3rem;
}

.pr-12 {
  padding-right: 3rem;
}

.pb-12 {
  padding-bottom: 3rem;
}

.pl-12 {
  padding-left: 3rem;
}

.pt-16 {
  padding-top: 4rem;
}

.pr-16 {
  padding-right: 4rem;
}

.pb-16 {
  padding-bottom: 4rem;
}

.pl-16 {
  padding-left: 4rem;
}

.pt-20 {
  padding-top: 5rem;
}

.pr-20 {
  padding-right: 5rem;
}

.pb-20 {
  padding-bottom: 5rem;
}

.pl-20 {
  padding-left: 5rem;
}

.pt-24 {
  padding-top: 6rem;
}

.pr-24 {
  padding-right: 6rem;
}

.pb-24 {
  padding-bottom: 6rem;
}

.pl-24 {
  padding-left: 6rem;
}

.pt-32 {
  padding-top: 8rem;
}

.pr-32 {
  padding-right: 8rem;
}

.pb-32 {
  padding-bottom: 8rem;
}

.pl-32 {
  padding-left: 8rem;
}

.pt-40 {
  padding-top: 10rem;
}

.pr-40 {
  padding-right: 10rem;
}

.pb-40 {
  padding-bottom: 10rem;
}

.pl-40 {
  padding-left: 10rem;
}

.pt-48 {
  padding-top: 12rem;
}

.pr-48 {
  padding-right: 12rem;
}

.pb-48 {
  padding-bottom: 12rem;
}

.pl-48 {
  padding-left: 12rem;
}

.pt-56 {
  padding-top: 14rem;
}

.pr-56 {
  padding-right: 14rem;
}

.pb-56 {
  padding-bottom: 14rem;
}

.pl-56 {
  padding-left: 14rem;
}

.pt-64 {
  padding-top: 16rem;
}

.pr-64 {
  padding-right: 16rem;
}

.pb-64 {
  padding-bottom: 16rem;
}

.pl-64 {
  padding-left: 16rem;
}

.pt-px {
  padding-top: 1px;
}

.pr-px {
  padding-right: 1px;
}

.pb-px {
  padding-bottom: 1px;
}

.pl-px {
  padding-left: 1px;
}

.placeholder-transparent::-webkit-input-placeholder {
  color: transparent;
}

.placeholder-transparent::-moz-placeholder {
  color: transparent;
}

.placeholder-transparent:-ms-input-placeholder {
  color: transparent;
}

.placeholder-transparent::-ms-input-placeholder {
  color: transparent;
}

.placeholder-transparent::placeholder {
  color: transparent;
}

.placeholder-black::-webkit-input-placeholder {
  color: #000;
}

.placeholder-black::-moz-placeholder {
  color: #000;
}

.placeholder-black:-ms-input-placeholder {
  color: #000;
}

.placeholder-black::-ms-input-placeholder {
  color: #000;
}

.placeholder-black::placeholder {
  color: #000;
}

.placeholder-white::-webkit-input-placeholder {
  color: #fff;
}

.placeholder-white::-moz-placeholder {
  color: #fff;
}

.placeholder-white:-ms-input-placeholder {
  color: #fff;
}

.placeholder-white::-ms-input-placeholder {
  color: #fff;
}

.placeholder-white::placeholder {
  color: #fff;
}

.placeholder-gray-100::-webkit-input-placeholder {
  color: #f7fafc;
}

.placeholder-gray-100::-moz-placeholder {
  color: #f7fafc;
}

.placeholder-gray-100:-ms-input-placeholder {
  color: #f7fafc;
}

.placeholder-gray-100::-ms-input-placeholder {
  color: #f7fafc;
}

.placeholder-gray-100::placeholder {
  color: #f7fafc;
}

.placeholder-gray-200::-webkit-input-placeholder {
  color: #edf2f7;
}

.placeholder-gray-200::-moz-placeholder {
  color: #edf2f7;
}

.placeholder-gray-200:-ms-input-placeholder {
  color: #edf2f7;
}

.placeholder-gray-200::-ms-input-placeholder {
  color: #edf2f7;
}

.placeholder-gray-200::placeholder {
  color: #edf2f7;
}

.placeholder-gray-300::-webkit-input-placeholder {
  color: #e2e8f0;
}

.placeholder-gray-300::-moz-placeholder {
  color: #e2e8f0;
}

.placeholder-gray-300:-ms-input-placeholder {
  color: #e2e8f0;
}

.placeholder-gray-300::-ms-input-placeholder {
  color: #e2e8f0;
}

.placeholder-gray-300::placeholder {
  color: #e2e8f0;
}

.placeholder-gray-400::-webkit-input-placeholder {
  color: #cbd5e0;
}

.placeholder-gray-400::-moz-placeholder {
  color: #cbd5e0;
}

.placeholder-gray-400:-ms-input-placeholder {
  color: #cbd5e0;
}

.placeholder-gray-400::-ms-input-placeholder {
  color: #cbd5e0;
}

.placeholder-gray-400::placeholder {
  color: #cbd5e0;
}

.placeholder-gray-500::-webkit-input-placeholder {
  color: #a0aec0;
}

.placeholder-gray-500::-moz-placeholder {
  color: #a0aec0;
}

.placeholder-gray-500:-ms-input-placeholder {
  color: #a0aec0;
}

.placeholder-gray-500::-ms-input-placeholder {
  color: #a0aec0;
}

.placeholder-gray-500::placeholder {
  color: #a0aec0;
}

.placeholder-gray-600::-webkit-input-placeholder {
  color: #718096;
}

.placeholder-gray-600::-moz-placeholder {
  color: #718096;
}

.placeholder-gray-600:-ms-input-placeholder {
  color: #718096;
}

.placeholder-gray-600::-ms-input-placeholder {
  color: #718096;
}

.placeholder-gray-600::placeholder {
  color: #718096;
}

.placeholder-gray-700::-webkit-input-placeholder {
  color: #4a5568;
}

.placeholder-gray-700::-moz-placeholder {
  color: #4a5568;
}

.placeholder-gray-700:-ms-input-placeholder {
  color: #4a5568;
}

.placeholder-gray-700::-ms-input-placeholder {
  color: #4a5568;
}

.placeholder-gray-700::placeholder {
  color: #4a5568;
}

.placeholder-gray-800::-webkit-input-placeholder {
  color: #2d3748;
}

.placeholder-gray-800::-moz-placeholder {
  color: #2d3748;
}

.placeholder-gray-800:-ms-input-placeholder {
  color: #2d3748;
}

.placeholder-gray-800::-ms-input-placeholder {
  color: #2d3748;
}

.placeholder-gray-800::placeholder {
  color: #2d3748;
}

.placeholder-gray-900::-webkit-input-placeholder {
  color: #1a202c;
}

.placeholder-gray-900::-moz-placeholder {
  color: #1a202c;
}

.placeholder-gray-900:-ms-input-placeholder {
  color: #1a202c;
}

.placeholder-gray-900::-ms-input-placeholder {
  color: #1a202c;
}

.placeholder-gray-900::placeholder {
  color: #1a202c;
}

.placeholder-red-100::-webkit-input-placeholder {
  color: #fff5f5;
}

.placeholder-red-100::-moz-placeholder {
  color: #fff5f5;
}

.placeholder-red-100:-ms-input-placeholder {
  color: #fff5f5;
}

.placeholder-red-100::-ms-input-placeholder {
  color: #fff5f5;
}

.placeholder-red-100::placeholder {
  color: #fff5f5;
}

.placeholder-red-200::-webkit-input-placeholder {
  color: #fed7d7;
}

.placeholder-red-200::-moz-placeholder {
  color: #fed7d7;
}

.placeholder-red-200:-ms-input-placeholder {
  color: #fed7d7;
}

.placeholder-red-200::-ms-input-placeholder {
  color: #fed7d7;
}

.placeholder-red-200::placeholder {
  color: #fed7d7;
}

.placeholder-red-300::-webkit-input-placeholder {
  color: #feb2b2;
}

.placeholder-red-300::-moz-placeholder {
  color: #feb2b2;
}

.placeholder-red-300:-ms-input-placeholder {
  color: #feb2b2;
}

.placeholder-red-300::-ms-input-placeholder {
  color: #feb2b2;
}

.placeholder-red-300::placeholder {
  color: #feb2b2;
}

.placeholder-red-400::-webkit-input-placeholder {
  color: #fc8181;
}

.placeholder-red-400::-moz-placeholder {
  color: #fc8181;
}

.placeholder-red-400:-ms-input-placeholder {
  color: #fc8181;
}

.placeholder-red-400::-ms-input-placeholder {
  color: #fc8181;
}

.placeholder-red-400::placeholder {
  color: #fc8181;
}

.placeholder-red-500::-webkit-input-placeholder {
  color: #f56565;
}

.placeholder-red-500::-moz-placeholder {
  color: #f56565;
}

.placeholder-red-500:-ms-input-placeholder {
  color: #f56565;
}

.placeholder-red-500::-ms-input-placeholder {
  color: #f56565;
}

.placeholder-red-500::placeholder {
  color: #f56565;
}

.placeholder-red-600::-webkit-input-placeholder {
  color: #e53e3e;
}

.placeholder-red-600::-moz-placeholder {
  color: #e53e3e;
}

.placeholder-red-600:-ms-input-placeholder {
  color: #e53e3e;
}

.placeholder-red-600::-ms-input-placeholder {
  color: #e53e3e;
}

.placeholder-red-600::placeholder {
  color: #e53e3e;
}

.placeholder-red-700::-webkit-input-placeholder {
  color: #c53030;
}

.placeholder-red-700::-moz-placeholder {
  color: #c53030;
}

.placeholder-red-700:-ms-input-placeholder {
  color: #c53030;
}

.placeholder-red-700::-ms-input-placeholder {
  color: #c53030;
}

.placeholder-red-700::placeholder {
  color: #c53030;
}

.placeholder-red-800::-webkit-input-placeholder {
  color: #9b2c2c;
}

.placeholder-red-800::-moz-placeholder {
  color: #9b2c2c;
}

.placeholder-red-800:-ms-input-placeholder {
  color: #9b2c2c;
}

.placeholder-red-800::-ms-input-placeholder {
  color: #9b2c2c;
}

.placeholder-red-800::placeholder {
  color: #9b2c2c;
}

.placeholder-red-900::-webkit-input-placeholder {
  color: #742a2a;
}

.placeholder-red-900::-moz-placeholder {
  color: #742a2a;
}

.placeholder-red-900:-ms-input-placeholder {
  color: #742a2a;
}

.placeholder-red-900::-ms-input-placeholder {
  color: #742a2a;
}

.placeholder-red-900::placeholder {
  color: #742a2a;
}

.placeholder-orange-100::-webkit-input-placeholder {
  color: #fffaf0;
}

.placeholder-orange-100::-moz-placeholder {
  color: #fffaf0;
}

.placeholder-orange-100:-ms-input-placeholder {
  color: #fffaf0;
}

.placeholder-orange-100::-ms-input-placeholder {
  color: #fffaf0;
}

.placeholder-orange-100::placeholder {
  color: #fffaf0;
}

.placeholder-orange-200::-webkit-input-placeholder {
  color: #feebc8;
}

.placeholder-orange-200::-moz-placeholder {
  color: #feebc8;
}

.placeholder-orange-200:-ms-input-placeholder {
  color: #feebc8;
}

.placeholder-orange-200::-ms-input-placeholder {
  color: #feebc8;
}

.placeholder-orange-200::placeholder {
  color: #feebc8;
}

.placeholder-orange-300::-webkit-input-placeholder {
  color: #fbd38d;
}

.placeholder-orange-300::-moz-placeholder {
  color: #fbd38d;
}

.placeholder-orange-300:-ms-input-placeholder {
  color: #fbd38d;
}

.placeholder-orange-300::-ms-input-placeholder {
  color: #fbd38d;
}

.placeholder-orange-300::placeholder {
  color: #fbd38d;
}

.placeholder-orange-400::-webkit-input-placeholder {
  color: #f6ad55;
}

.placeholder-orange-400::-moz-placeholder {
  color: #f6ad55;
}

.placeholder-orange-400:-ms-input-placeholder {
  color: #f6ad55;
}

.placeholder-orange-400::-ms-input-placeholder {
  color: #f6ad55;
}

.placeholder-orange-400::placeholder {
  color: #f6ad55;
}

.placeholder-orange-500::-webkit-input-placeholder {
  color: #ed8936;
}

.placeholder-orange-500::-moz-placeholder {
  color: #ed8936;
}

.placeholder-orange-500:-ms-input-placeholder {
  color: #ed8936;
}

.placeholder-orange-500::-ms-input-placeholder {
  color: #ed8936;
}

.placeholder-orange-500::placeholder {
  color: #ed8936;
}

.placeholder-orange-600::-webkit-input-placeholder {
  color: #dd6b20;
}

.placeholder-orange-600::-moz-placeholder {
  color: #dd6b20;
}

.placeholder-orange-600:-ms-input-placeholder {
  color: #dd6b20;
}

.placeholder-orange-600::-ms-input-placeholder {
  color: #dd6b20;
}

.placeholder-orange-600::placeholder {
  color: #dd6b20;
}

.placeholder-orange-700::-webkit-input-placeholder {
  color: #c05621;
}

.placeholder-orange-700::-moz-placeholder {
  color: #c05621;
}

.placeholder-orange-700:-ms-input-placeholder {
  color: #c05621;
}

.placeholder-orange-700::-ms-input-placeholder {
  color: #c05621;
}

.placeholder-orange-700::placeholder {
  color: #c05621;
}

.placeholder-orange-800::-webkit-input-placeholder {
  color: #9c4221;
}

.placeholder-orange-800::-moz-placeholder {
  color: #9c4221;
}

.placeholder-orange-800:-ms-input-placeholder {
  color: #9c4221;
}

.placeholder-orange-800::-ms-input-placeholder {
  color: #9c4221;
}

.placeholder-orange-800::placeholder {
  color: #9c4221;
}

.placeholder-orange-900::-webkit-input-placeholder {
  color: #7b341e;
}

.placeholder-orange-900::-moz-placeholder {
  color: #7b341e;
}

.placeholder-orange-900:-ms-input-placeholder {
  color: #7b341e;
}

.placeholder-orange-900::-ms-input-placeholder {
  color: #7b341e;
}

.placeholder-orange-900::placeholder {
  color: #7b341e;
}

.placeholder-yellow-100::-webkit-input-placeholder {
  color: #fffff0;
}

.placeholder-yellow-100::-moz-placeholder {
  color: #fffff0;
}

.placeholder-yellow-100:-ms-input-placeholder {
  color: #fffff0;
}

.placeholder-yellow-100::-ms-input-placeholder {
  color: #fffff0;
}

.placeholder-yellow-100::placeholder {
  color: #fffff0;
}

.placeholder-yellow-200::-webkit-input-placeholder {
  color: #fefcbf;
}

.placeholder-yellow-200::-moz-placeholder {
  color: #fefcbf;
}

.placeholder-yellow-200:-ms-input-placeholder {
  color: #fefcbf;
}

.placeholder-yellow-200::-ms-input-placeholder {
  color: #fefcbf;
}

.placeholder-yellow-200::placeholder {
  color: #fefcbf;
}

.placeholder-yellow-300::-webkit-input-placeholder {
  color: #faf089;
}

.placeholder-yellow-300::-moz-placeholder {
  color: #faf089;
}

.placeholder-yellow-300:-ms-input-placeholder {
  color: #faf089;
}

.placeholder-yellow-300::-ms-input-placeholder {
  color: #faf089;
}

.placeholder-yellow-300::placeholder {
  color: #faf089;
}

.placeholder-yellow-400::-webkit-input-placeholder {
  color: #f6e05e;
}

.placeholder-yellow-400::-moz-placeholder {
  color: #f6e05e;
}

.placeholder-yellow-400:-ms-input-placeholder {
  color: #f6e05e;
}

.placeholder-yellow-400::-ms-input-placeholder {
  color: #f6e05e;
}

.placeholder-yellow-400::placeholder {
  color: #f6e05e;
}

.placeholder-yellow-500::-webkit-input-placeholder {
  color: #ecc94b;
}

.placeholder-yellow-500::-moz-placeholder {
  color: #ecc94b;
}

.placeholder-yellow-500:-ms-input-placeholder {
  color: #ecc94b;
}

.placeholder-yellow-500::-ms-input-placeholder {
  color: #ecc94b;
}

.placeholder-yellow-500::placeholder {
  color: #ecc94b;
}

.placeholder-yellow-600::-webkit-input-placeholder {
  color: #d69e2e;
}

.placeholder-yellow-600::-moz-placeholder {
  color: #d69e2e;
}

.placeholder-yellow-600:-ms-input-placeholder {
  color: #d69e2e;
}

.placeholder-yellow-600::-ms-input-placeholder {
  color: #d69e2e;
}

.placeholder-yellow-600::placeholder {
  color: #d69e2e;
}

.placeholder-yellow-700::-webkit-input-placeholder {
  color: #b7791f;
}

.placeholder-yellow-700::-moz-placeholder {
  color: #b7791f;
}

.placeholder-yellow-700:-ms-input-placeholder {
  color: #b7791f;
}

.placeholder-yellow-700::-ms-input-placeholder {
  color: #b7791f;
}

.placeholder-yellow-700::placeholder {
  color: #b7791f;
}

.placeholder-yellow-800::-webkit-input-placeholder {
  color: #975a16;
}

.placeholder-yellow-800::-moz-placeholder {
  color: #975a16;
}

.placeholder-yellow-800:-ms-input-placeholder {
  color: #975a16;
}

.placeholder-yellow-800::-ms-input-placeholder {
  color: #975a16;
}

.placeholder-yellow-800::placeholder {
  color: #975a16;
}

.placeholder-yellow-900::-webkit-input-placeholder {
  color: #744210;
}

.placeholder-yellow-900::-moz-placeholder {
  color: #744210;
}

.placeholder-yellow-900:-ms-input-placeholder {
  color: #744210;
}

.placeholder-yellow-900::-ms-input-placeholder {
  color: #744210;
}

.placeholder-yellow-900::placeholder {
  color: #744210;
}

.placeholder-green-100::-webkit-input-placeholder {
  color: #f0fff4;
}

.placeholder-green-100::-moz-placeholder {
  color: #f0fff4;
}

.placeholder-green-100:-ms-input-placeholder {
  color: #f0fff4;
}

.placeholder-green-100::-ms-input-placeholder {
  color: #f0fff4;
}

.placeholder-green-100::placeholder {
  color: #f0fff4;
}

.placeholder-green-200::-webkit-input-placeholder {
  color: #c6f6d5;
}

.placeholder-green-200::-moz-placeholder {
  color: #c6f6d5;
}

.placeholder-green-200:-ms-input-placeholder {
  color: #c6f6d5;
}

.placeholder-green-200::-ms-input-placeholder {
  color: #c6f6d5;
}

.placeholder-green-200::placeholder {
  color: #c6f6d5;
}

.placeholder-green-300::-webkit-input-placeholder {
  color: #9ae6b4;
}

.placeholder-green-300::-moz-placeholder {
  color: #9ae6b4;
}

.placeholder-green-300:-ms-input-placeholder {
  color: #9ae6b4;
}

.placeholder-green-300::-ms-input-placeholder {
  color: #9ae6b4;
}

.placeholder-green-300::placeholder {
  color: #9ae6b4;
}

.placeholder-green-400::-webkit-input-placeholder {
  color: #68d391;
}

.placeholder-green-400::-moz-placeholder {
  color: #68d391;
}

.placeholder-green-400:-ms-input-placeholder {
  color: #68d391;
}

.placeholder-green-400::-ms-input-placeholder {
  color: #68d391;
}

.placeholder-green-400::placeholder {
  color: #68d391;
}

.placeholder-green-500::-webkit-input-placeholder {
  color: #48bb78;
}

.placeholder-green-500::-moz-placeholder {
  color: #48bb78;
}

.placeholder-green-500:-ms-input-placeholder {
  color: #48bb78;
}

.placeholder-green-500::-ms-input-placeholder {
  color: #48bb78;
}

.placeholder-green-500::placeholder {
  color: #48bb78;
}

.placeholder-green-600::-webkit-input-placeholder {
  color: #38a169;
}

.placeholder-green-600::-moz-placeholder {
  color: #38a169;
}

.placeholder-green-600:-ms-input-placeholder {
  color: #38a169;
}

.placeholder-green-600::-ms-input-placeholder {
  color: #38a169;
}

.placeholder-green-600::placeholder {
  color: #38a169;
}

.placeholder-green-700::-webkit-input-placeholder {
  color: #2f855a;
}

.placeholder-green-700::-moz-placeholder {
  color: #2f855a;
}

.placeholder-green-700:-ms-input-placeholder {
  color: #2f855a;
}

.placeholder-green-700::-ms-input-placeholder {
  color: #2f855a;
}

.placeholder-green-700::placeholder {
  color: #2f855a;
}

.placeholder-green-800::-webkit-input-placeholder {
  color: #276749;
}

.placeholder-green-800::-moz-placeholder {
  color: #276749;
}

.placeholder-green-800:-ms-input-placeholder {
  color: #276749;
}

.placeholder-green-800::-ms-input-placeholder {
  color: #276749;
}

.placeholder-green-800::placeholder {
  color: #276749;
}

.placeholder-green-900::-webkit-input-placeholder {
  color: #22543d;
}

.placeholder-green-900::-moz-placeholder {
  color: #22543d;
}

.placeholder-green-900:-ms-input-placeholder {
  color: #22543d;
}

.placeholder-green-900::-ms-input-placeholder {
  color: #22543d;
}

.placeholder-green-900::placeholder {
  color: #22543d;
}

.placeholder-teal-100::-webkit-input-placeholder {
  color: #e6fffa;
}

.placeholder-teal-100::-moz-placeholder {
  color: #e6fffa;
}

.placeholder-teal-100:-ms-input-placeholder {
  color: #e6fffa;
}

.placeholder-teal-100::-ms-input-placeholder {
  color: #e6fffa;
}

.placeholder-teal-100::placeholder {
  color: #e6fffa;
}

.placeholder-teal-200::-webkit-input-placeholder {
  color: #b2f5ea;
}

.placeholder-teal-200::-moz-placeholder {
  color: #b2f5ea;
}

.placeholder-teal-200:-ms-input-placeholder {
  color: #b2f5ea;
}

.placeholder-teal-200::-ms-input-placeholder {
  color: #b2f5ea;
}

.placeholder-teal-200::placeholder {
  color: #b2f5ea;
}

.placeholder-teal-300::-webkit-input-placeholder {
  color: #81e6d9;
}

.placeholder-teal-300::-moz-placeholder {
  color: #81e6d9;
}

.placeholder-teal-300:-ms-input-placeholder {
  color: #81e6d9;
}

.placeholder-teal-300::-ms-input-placeholder {
  color: #81e6d9;
}

.placeholder-teal-300::placeholder {
  color: #81e6d9;
}

.placeholder-teal-400::-webkit-input-placeholder {
  color: #4fd1c5;
}

.placeholder-teal-400::-moz-placeholder {
  color: #4fd1c5;
}

.placeholder-teal-400:-ms-input-placeholder {
  color: #4fd1c5;
}

.placeholder-teal-400::-ms-input-placeholder {
  color: #4fd1c5;
}

.placeholder-teal-400::placeholder {
  color: #4fd1c5;
}

.placeholder-teal-500::-webkit-input-placeholder {
  color: #38b2ac;
}

.placeholder-teal-500::-moz-placeholder {
  color: #38b2ac;
}

.placeholder-teal-500:-ms-input-placeholder {
  color: #38b2ac;
}

.placeholder-teal-500::-ms-input-placeholder {
  color: #38b2ac;
}

.placeholder-teal-500::placeholder {
  color: #38b2ac;
}

.placeholder-teal-600::-webkit-input-placeholder {
  color: #319795;
}

.placeholder-teal-600::-moz-placeholder {
  color: #319795;
}

.placeholder-teal-600:-ms-input-placeholder {
  color: #319795;
}

.placeholder-teal-600::-ms-input-placeholder {
  color: #319795;
}

.placeholder-teal-600::placeholder {
  color: #319795;
}

.placeholder-teal-700::-webkit-input-placeholder {
  color: #2c7a7b;
}

.placeholder-teal-700::-moz-placeholder {
  color: #2c7a7b;
}

.placeholder-teal-700:-ms-input-placeholder {
  color: #2c7a7b;
}

.placeholder-teal-700::-ms-input-placeholder {
  color: #2c7a7b;
}

.placeholder-teal-700::placeholder {
  color: #2c7a7b;
}

.placeholder-teal-800::-webkit-input-placeholder {
  color: #285e61;
}

.placeholder-teal-800::-moz-placeholder {
  color: #285e61;
}

.placeholder-teal-800:-ms-input-placeholder {
  color: #285e61;
}

.placeholder-teal-800::-ms-input-placeholder {
  color: #285e61;
}

.placeholder-teal-800::placeholder {
  color: #285e61;
}

.placeholder-teal-900::-webkit-input-placeholder {
  color: #234e52;
}

.placeholder-teal-900::-moz-placeholder {
  color: #234e52;
}

.placeholder-teal-900:-ms-input-placeholder {
  color: #234e52;
}

.placeholder-teal-900::-ms-input-placeholder {
  color: #234e52;
}

.placeholder-teal-900::placeholder {
  color: #234e52;
}

.placeholder-blue-100::-webkit-input-placeholder {
  color: #ebf8ff;
}

.placeholder-blue-100::-moz-placeholder {
  color: #ebf8ff;
}

.placeholder-blue-100:-ms-input-placeholder {
  color: #ebf8ff;
}

.placeholder-blue-100::-ms-input-placeholder {
  color: #ebf8ff;
}

.placeholder-blue-100::placeholder {
  color: #ebf8ff;
}

.placeholder-blue-200::-webkit-input-placeholder {
  color: #bee3f8;
}

.placeholder-blue-200::-moz-placeholder {
  color: #bee3f8;
}

.placeholder-blue-200:-ms-input-placeholder {
  color: #bee3f8;
}

.placeholder-blue-200::-ms-input-placeholder {
  color: #bee3f8;
}

.placeholder-blue-200::placeholder {
  color: #bee3f8;
}

.placeholder-blue-300::-webkit-input-placeholder {
  color: #90cdf4;
}

.placeholder-blue-300::-moz-placeholder {
  color: #90cdf4;
}

.placeholder-blue-300:-ms-input-placeholder {
  color: #90cdf4;
}

.placeholder-blue-300::-ms-input-placeholder {
  color: #90cdf4;
}

.placeholder-blue-300::placeholder {
  color: #90cdf4;
}

.placeholder-blue-400::-webkit-input-placeholder {
  color: #63b3ed;
}

.placeholder-blue-400::-moz-placeholder {
  color: #63b3ed;
}

.placeholder-blue-400:-ms-input-placeholder {
  color: #63b3ed;
}

.placeholder-blue-400::-ms-input-placeholder {
  color: #63b3ed;
}

.placeholder-blue-400::placeholder {
  color: #63b3ed;
}

.placeholder-blue-500::-webkit-input-placeholder {
  color: #4299e1;
}

.placeholder-blue-500::-moz-placeholder {
  color: #4299e1;
}

.placeholder-blue-500:-ms-input-placeholder {
  color: #4299e1;
}

.placeholder-blue-500::-ms-input-placeholder {
  color: #4299e1;
}

.placeholder-blue-500::placeholder {
  color: #4299e1;
}

.placeholder-blue-600::-webkit-input-placeholder {
  color: #3182ce;
}

.placeholder-blue-600::-moz-placeholder {
  color: #3182ce;
}

.placeholder-blue-600:-ms-input-placeholder {
  color: #3182ce;
}

.placeholder-blue-600::-ms-input-placeholder {
  color: #3182ce;
}

.placeholder-blue-600::placeholder {
  color: #3182ce;
}

.placeholder-blue-700::-webkit-input-placeholder {
  color: #2b6cb0;
}

.placeholder-blue-700::-moz-placeholder {
  color: #2b6cb0;
}

.placeholder-blue-700:-ms-input-placeholder {
  color: #2b6cb0;
}

.placeholder-blue-700::-ms-input-placeholder {
  color: #2b6cb0;
}

.placeholder-blue-700::placeholder {
  color: #2b6cb0;
}

.placeholder-blue-800::-webkit-input-placeholder {
  color: #2c5282;
}

.placeholder-blue-800::-moz-placeholder {
  color: #2c5282;
}

.placeholder-blue-800:-ms-input-placeholder {
  color: #2c5282;
}

.placeholder-blue-800::-ms-input-placeholder {
  color: #2c5282;
}

.placeholder-blue-800::placeholder {
  color: #2c5282;
}

.placeholder-blue-900::-webkit-input-placeholder {
  color: #2a4365;
}

.placeholder-blue-900::-moz-placeholder {
  color: #2a4365;
}

.placeholder-blue-900:-ms-input-placeholder {
  color: #2a4365;
}

.placeholder-blue-900::-ms-input-placeholder {
  color: #2a4365;
}

.placeholder-blue-900::placeholder {
  color: #2a4365;
}

.placeholder-indigo-100::-webkit-input-placeholder {
  color: #ebf4ff;
}

.placeholder-indigo-100::-moz-placeholder {
  color: #ebf4ff;
}

.placeholder-indigo-100:-ms-input-placeholder {
  color: #ebf4ff;
}

.placeholder-indigo-100::-ms-input-placeholder {
  color: #ebf4ff;
}

.placeholder-indigo-100::placeholder {
  color: #ebf4ff;
}

.placeholder-indigo-200::-webkit-input-placeholder {
  color: #c3dafe;
}

.placeholder-indigo-200::-moz-placeholder {
  color: #c3dafe;
}

.placeholder-indigo-200:-ms-input-placeholder {
  color: #c3dafe;
}

.placeholder-indigo-200::-ms-input-placeholder {
  color: #c3dafe;
}

.placeholder-indigo-200::placeholder {
  color: #c3dafe;
}

.placeholder-indigo-300::-webkit-input-placeholder {
  color: #a3bffa;
}

.placeholder-indigo-300::-moz-placeholder {
  color: #a3bffa;
}

.placeholder-indigo-300:-ms-input-placeholder {
  color: #a3bffa;
}

.placeholder-indigo-300::-ms-input-placeholder {
  color: #a3bffa;
}

.placeholder-indigo-300::placeholder {
  color: #a3bffa;
}

.placeholder-indigo-400::-webkit-input-placeholder {
  color: #7f9cf5;
}

.placeholder-indigo-400::-moz-placeholder {
  color: #7f9cf5;
}

.placeholder-indigo-400:-ms-input-placeholder {
  color: #7f9cf5;
}

.placeholder-indigo-400::-ms-input-placeholder {
  color: #7f9cf5;
}

.placeholder-indigo-400::placeholder {
  color: #7f9cf5;
}

.placeholder-indigo-500::-webkit-input-placeholder {
  color: #667eea;
}

.placeholder-indigo-500::-moz-placeholder {
  color: #667eea;
}

.placeholder-indigo-500:-ms-input-placeholder {
  color: #667eea;
}

.placeholder-indigo-500::-ms-input-placeholder {
  color: #667eea;
}

.placeholder-indigo-500::placeholder {
  color: #667eea;
}

.placeholder-indigo-600::-webkit-input-placeholder {
  color: #5a67d8;
}

.placeholder-indigo-600::-moz-placeholder {
  color: #5a67d8;
}

.placeholder-indigo-600:-ms-input-placeholder {
  color: #5a67d8;
}

.placeholder-indigo-600::-ms-input-placeholder {
  color: #5a67d8;
}

.placeholder-indigo-600::placeholder {
  color: #5a67d8;
}

.placeholder-indigo-700::-webkit-input-placeholder {
  color: #4c51bf;
}

.placeholder-indigo-700::-moz-placeholder {
  color: #4c51bf;
}

.placeholder-indigo-700:-ms-input-placeholder {
  color: #4c51bf;
}

.placeholder-indigo-700::-ms-input-placeholder {
  color: #4c51bf;
}

.placeholder-indigo-700::placeholder {
  color: #4c51bf;
}

.placeholder-indigo-800::-webkit-input-placeholder {
  color: #434190;
}

.placeholder-indigo-800::-moz-placeholder {
  color: #434190;
}

.placeholder-indigo-800:-ms-input-placeholder {
  color: #434190;
}

.placeholder-indigo-800::-ms-input-placeholder {
  color: #434190;
}

.placeholder-indigo-800::placeholder {
  color: #434190;
}

.placeholder-indigo-900::-webkit-input-placeholder {
  color: #3c366b;
}

.placeholder-indigo-900::-moz-placeholder {
  color: #3c366b;
}

.placeholder-indigo-900:-ms-input-placeholder {
  color: #3c366b;
}

.placeholder-indigo-900::-ms-input-placeholder {
  color: #3c366b;
}

.placeholder-indigo-900::placeholder {
  color: #3c366b;
}

.placeholder-purple-100::-webkit-input-placeholder {
  color: #faf5ff;
}

.placeholder-purple-100::-moz-placeholder {
  color: #faf5ff;
}

.placeholder-purple-100:-ms-input-placeholder {
  color: #faf5ff;
}

.placeholder-purple-100::-ms-input-placeholder {
  color: #faf5ff;
}

.placeholder-purple-100::placeholder {
  color: #faf5ff;
}

.placeholder-purple-200::-webkit-input-placeholder {
  color: #e9d8fd;
}

.placeholder-purple-200::-moz-placeholder {
  color: #e9d8fd;
}

.placeholder-purple-200:-ms-input-placeholder {
  color: #e9d8fd;
}

.placeholder-purple-200::-ms-input-placeholder {
  color: #e9d8fd;
}

.placeholder-purple-200::placeholder {
  color: #e9d8fd;
}

.placeholder-purple-300::-webkit-input-placeholder {
  color: #d6bcfa;
}

.placeholder-purple-300::-moz-placeholder {
  color: #d6bcfa;
}

.placeholder-purple-300:-ms-input-placeholder {
  color: #d6bcfa;
}

.placeholder-purple-300::-ms-input-placeholder {
  color: #d6bcfa;
}

.placeholder-purple-300::placeholder {
  color: #d6bcfa;
}

.placeholder-purple-400::-webkit-input-placeholder {
  color: #b794f4;
}

.placeholder-purple-400::-moz-placeholder {
  color: #b794f4;
}

.placeholder-purple-400:-ms-input-placeholder {
  color: #b794f4;
}

.placeholder-purple-400::-ms-input-placeholder {
  color: #b794f4;
}

.placeholder-purple-400::placeholder {
  color: #b794f4;
}

.placeholder-purple-500::-webkit-input-placeholder {
  color: #9f7aea;
}

.placeholder-purple-500::-moz-placeholder {
  color: #9f7aea;
}

.placeholder-purple-500:-ms-input-placeholder {
  color: #9f7aea;
}

.placeholder-purple-500::-ms-input-placeholder {
  color: #9f7aea;
}

.placeholder-purple-500::placeholder {
  color: #9f7aea;
}

.placeholder-purple-600::-webkit-input-placeholder {
  color: #805ad5;
}

.placeholder-purple-600::-moz-placeholder {
  color: #805ad5;
}

.placeholder-purple-600:-ms-input-placeholder {
  color: #805ad5;
}

.placeholder-purple-600::-ms-input-placeholder {
  color: #805ad5;
}

.placeholder-purple-600::placeholder {
  color: #805ad5;
}

.placeholder-purple-700::-webkit-input-placeholder {
  color: #6b46c1;
}

.placeholder-purple-700::-moz-placeholder {
  color: #6b46c1;
}

.placeholder-purple-700:-ms-input-placeholder {
  color: #6b46c1;
}

.placeholder-purple-700::-ms-input-placeholder {
  color: #6b46c1;
}

.placeholder-purple-700::placeholder {
  color: #6b46c1;
}

.placeholder-purple-800::-webkit-input-placeholder {
  color: #553c9a;
}

.placeholder-purple-800::-moz-placeholder {
  color: #553c9a;
}

.placeholder-purple-800:-ms-input-placeholder {
  color: #553c9a;
}

.placeholder-purple-800::-ms-input-placeholder {
  color: #553c9a;
}

.placeholder-purple-800::placeholder {
  color: #553c9a;
}

.placeholder-purple-900::-webkit-input-placeholder {
  color: #44337a;
}

.placeholder-purple-900::-moz-placeholder {
  color: #44337a;
}

.placeholder-purple-900:-ms-input-placeholder {
  color: #44337a;
}

.placeholder-purple-900::-ms-input-placeholder {
  color: #44337a;
}

.placeholder-purple-900::placeholder {
  color: #44337a;
}

.placeholder-pink-100::-webkit-input-placeholder {
  color: #fff5f7;
}

.placeholder-pink-100::-moz-placeholder {
  color: #fff5f7;
}

.placeholder-pink-100:-ms-input-placeholder {
  color: #fff5f7;
}

.placeholder-pink-100::-ms-input-placeholder {
  color: #fff5f7;
}

.placeholder-pink-100::placeholder {
  color: #fff5f7;
}

.placeholder-pink-200::-webkit-input-placeholder {
  color: #fed7e2;
}

.placeholder-pink-200::-moz-placeholder {
  color: #fed7e2;
}

.placeholder-pink-200:-ms-input-placeholder {
  color: #fed7e2;
}

.placeholder-pink-200::-ms-input-placeholder {
  color: #fed7e2;
}

.placeholder-pink-200::placeholder {
  color: #fed7e2;
}

.placeholder-pink-300::-webkit-input-placeholder {
  color: #fbb6ce;
}

.placeholder-pink-300::-moz-placeholder {
  color: #fbb6ce;
}

.placeholder-pink-300:-ms-input-placeholder {
  color: #fbb6ce;
}

.placeholder-pink-300::-ms-input-placeholder {
  color: #fbb6ce;
}

.placeholder-pink-300::placeholder {
  color: #fbb6ce;
}

.placeholder-pink-400::-webkit-input-placeholder {
  color: #f687b3;
}

.placeholder-pink-400::-moz-placeholder {
  color: #f687b3;
}

.placeholder-pink-400:-ms-input-placeholder {
  color: #f687b3;
}

.placeholder-pink-400::-ms-input-placeholder {
  color: #f687b3;
}

.placeholder-pink-400::placeholder {
  color: #f687b3;
}

.placeholder-pink-500::-webkit-input-placeholder {
  color: #ed64a6;
}

.placeholder-pink-500::-moz-placeholder {
  color: #ed64a6;
}

.placeholder-pink-500:-ms-input-placeholder {
  color: #ed64a6;
}

.placeholder-pink-500::-ms-input-placeholder {
  color: #ed64a6;
}

.placeholder-pink-500::placeholder {
  color: #ed64a6;
}

.placeholder-pink-600::-webkit-input-placeholder {
  color: #d53f8c;
}

.placeholder-pink-600::-moz-placeholder {
  color: #d53f8c;
}

.placeholder-pink-600:-ms-input-placeholder {
  color: #d53f8c;
}

.placeholder-pink-600::-ms-input-placeholder {
  color: #d53f8c;
}

.placeholder-pink-600::placeholder {
  color: #d53f8c;
}

.placeholder-pink-700::-webkit-input-placeholder {
  color: #b83280;
}

.placeholder-pink-700::-moz-placeholder {
  color: #b83280;
}

.placeholder-pink-700:-ms-input-placeholder {
  color: #b83280;
}

.placeholder-pink-700::-ms-input-placeholder {
  color: #b83280;
}

.placeholder-pink-700::placeholder {
  color: #b83280;
}

.placeholder-pink-800::-webkit-input-placeholder {
  color: #97266d;
}

.placeholder-pink-800::-moz-placeholder {
  color: #97266d;
}

.placeholder-pink-800:-ms-input-placeholder {
  color: #97266d;
}

.placeholder-pink-800::-ms-input-placeholder {
  color: #97266d;
}

.placeholder-pink-800::placeholder {
  color: #97266d;
}

.placeholder-pink-900::-webkit-input-placeholder {
  color: #702459;
}

.placeholder-pink-900::-moz-placeholder {
  color: #702459;
}

.placeholder-pink-900:-ms-input-placeholder {
  color: #702459;
}

.placeholder-pink-900::-ms-input-placeholder {
  color: #702459;
}

.placeholder-pink-900::placeholder {
  color: #702459;
}

.focus\:placeholder-transparent:focus::-webkit-input-placeholder {
  color: transparent;
}

.focus\:placeholder-transparent:focus::-moz-placeholder {
  color: transparent;
}

.focus\:placeholder-transparent:focus:-ms-input-placeholder {
  color: transparent;
}

.focus\:placeholder-transparent:focus::-ms-input-placeholder {
  color: transparent;
}

.focus\:placeholder-transparent:focus::placeholder {
  color: transparent;
}

.focus\:placeholder-black:focus::-webkit-input-placeholder {
  color: #000;
}

.focus\:placeholder-black:focus::-moz-placeholder {
  color: #000;
}

.focus\:placeholder-black:focus:-ms-input-placeholder {
  color: #000;
}

.focus\:placeholder-black:focus::-ms-input-placeholder {
  color: #000;
}

.focus\:placeholder-black:focus::placeholder {
  color: #000;
}

.focus\:placeholder-white:focus::-webkit-input-placeholder {
  color: #fff;
}

.focus\:placeholder-white:focus::-moz-placeholder {
  color: #fff;
}

.focus\:placeholder-white:focus:-ms-input-placeholder {
  color: #fff;
}

.focus\:placeholder-white:focus::-ms-input-placeholder {
  color: #fff;
}

.focus\:placeholder-white:focus::placeholder {
  color: #fff;
}

.focus\:placeholder-gray-100:focus::-webkit-input-placeholder {
  color: #f7fafc;
}

.focus\:placeholder-gray-100:focus::-moz-placeholder {
  color: #f7fafc;
}

.focus\:placeholder-gray-100:focus:-ms-input-placeholder {
  color: #f7fafc;
}

.focus\:placeholder-gray-100:focus::-ms-input-placeholder {
  color: #f7fafc;
}

.focus\:placeholder-gray-100:focus::placeholder {
  color: #f7fafc;
}

.focus\:placeholder-gray-200:focus::-webkit-input-placeholder {
  color: #edf2f7;
}

.focus\:placeholder-gray-200:focus::-moz-placeholder {
  color: #edf2f7;
}

.focus\:placeholder-gray-200:focus:-ms-input-placeholder {
  color: #edf2f7;
}

.focus\:placeholder-gray-200:focus::-ms-input-placeholder {
  color: #edf2f7;
}

.focus\:placeholder-gray-200:focus::placeholder {
  color: #edf2f7;
}

.focus\:placeholder-gray-300:focus::-webkit-input-placeholder {
  color: #e2e8f0;
}

.focus\:placeholder-gray-300:focus::-moz-placeholder {
  color: #e2e8f0;
}

.focus\:placeholder-gray-300:focus:-ms-input-placeholder {
  color: #e2e8f0;
}

.focus\:placeholder-gray-300:focus::-ms-input-placeholder {
  color: #e2e8f0;
}

.focus\:placeholder-gray-300:focus::placeholder {
  color: #e2e8f0;
}

.focus\:placeholder-gray-400:focus::-webkit-input-placeholder {
  color: #cbd5e0;
}

.focus\:placeholder-gray-400:focus::-moz-placeholder {
  color: #cbd5e0;
}

.focus\:placeholder-gray-400:focus:-ms-input-placeholder {
  color: #cbd5e0;
}

.focus\:placeholder-gray-400:focus::-ms-input-placeholder {
  color: #cbd5e0;
}

.focus\:placeholder-gray-400:focus::placeholder {
  color: #cbd5e0;
}

.focus\:placeholder-gray-500:focus::-webkit-input-placeholder {
  color: #a0aec0;
}

.focus\:placeholder-gray-500:focus::-moz-placeholder {
  color: #a0aec0;
}

.focus\:placeholder-gray-500:focus:-ms-input-placeholder {
  color: #a0aec0;
}

.focus\:placeholder-gray-500:focus::-ms-input-placeholder {
  color: #a0aec0;
}

.focus\:placeholder-gray-500:focus::placeholder {
  color: #a0aec0;
}

.focus\:placeholder-gray-600:focus::-webkit-input-placeholder {
  color: #718096;
}

.focus\:placeholder-gray-600:focus::-moz-placeholder {
  color: #718096;
}

.focus\:placeholder-gray-600:focus:-ms-input-placeholder {
  color: #718096;
}

.focus\:placeholder-gray-600:focus::-ms-input-placeholder {
  color: #718096;
}

.focus\:placeholder-gray-600:focus::placeholder {
  color: #718096;
}

.focus\:placeholder-gray-700:focus::-webkit-input-placeholder {
  color: #4a5568;
}

.focus\:placeholder-gray-700:focus::-moz-placeholder {
  color: #4a5568;
}

.focus\:placeholder-gray-700:focus:-ms-input-placeholder {
  color: #4a5568;
}

.focus\:placeholder-gray-700:focus::-ms-input-placeholder {
  color: #4a5568;
}

.focus\:placeholder-gray-700:focus::placeholder {
  color: #4a5568;
}

.focus\:placeholder-gray-800:focus::-webkit-input-placeholder {
  color: #2d3748;
}

.focus\:placeholder-gray-800:focus::-moz-placeholder {
  color: #2d3748;
}

.focus\:placeholder-gray-800:focus:-ms-input-placeholder {
  color: #2d3748;
}

.focus\:placeholder-gray-800:focus::-ms-input-placeholder {
  color: #2d3748;
}

.focus\:placeholder-gray-800:focus::placeholder {
  color: #2d3748;
}

.focus\:placeholder-gray-900:focus::-webkit-input-placeholder {
  color: #1a202c;
}

.focus\:placeholder-gray-900:focus::-moz-placeholder {
  color: #1a202c;
}

.focus\:placeholder-gray-900:focus:-ms-input-placeholder {
  color: #1a202c;
}

.focus\:placeholder-gray-900:focus::-ms-input-placeholder {
  color: #1a202c;
}

.focus\:placeholder-gray-900:focus::placeholder {
  color: #1a202c;
}

.focus\:placeholder-red-100:focus::-webkit-input-placeholder {
  color: #fff5f5;
}

.focus\:placeholder-red-100:focus::-moz-placeholder {
  color: #fff5f5;
}

.focus\:placeholder-red-100:focus:-ms-input-placeholder {
  color: #fff5f5;
}

.focus\:placeholder-red-100:focus::-ms-input-placeholder {
  color: #fff5f5;
}

.focus\:placeholder-red-100:focus::placeholder {
  color: #fff5f5;
}

.focus\:placeholder-red-200:focus::-webkit-input-placeholder {
  color: #fed7d7;
}

.focus\:placeholder-red-200:focus::-moz-placeholder {
  color: #fed7d7;
}

.focus\:placeholder-red-200:focus:-ms-input-placeholder {
  color: #fed7d7;
}

.focus\:placeholder-red-200:focus::-ms-input-placeholder {
  color: #fed7d7;
}

.focus\:placeholder-red-200:focus::placeholder {
  color: #fed7d7;
}

.focus\:placeholder-red-300:focus::-webkit-input-placeholder {
  color: #feb2b2;
}

.focus\:placeholder-red-300:focus::-moz-placeholder {
  color: #feb2b2;
}

.focus\:placeholder-red-300:focus:-ms-input-placeholder {
  color: #feb2b2;
}

.focus\:placeholder-red-300:focus::-ms-input-placeholder {
  color: #feb2b2;
}

.focus\:placeholder-red-300:focus::placeholder {
  color: #feb2b2;
}

.focus\:placeholder-red-400:focus::-webkit-input-placeholder {
  color: #fc8181;
}

.focus\:placeholder-red-400:focus::-moz-placeholder {
  color: #fc8181;
}

.focus\:placeholder-red-400:focus:-ms-input-placeholder {
  color: #fc8181;
}

.focus\:placeholder-red-400:focus::-ms-input-placeholder {
  color: #fc8181;
}

.focus\:placeholder-red-400:focus::placeholder {
  color: #fc8181;
}

.focus\:placeholder-red-500:focus::-webkit-input-placeholder {
  color: #f56565;
}

.focus\:placeholder-red-500:focus::-moz-placeholder {
  color: #f56565;
}

.focus\:placeholder-red-500:focus:-ms-input-placeholder {
  color: #f56565;
}

.focus\:placeholder-red-500:focus::-ms-input-placeholder {
  color: #f56565;
}

.focus\:placeholder-red-500:focus::placeholder {
  color: #f56565;
}

.focus\:placeholder-red-600:focus::-webkit-input-placeholder {
  color: #e53e3e;
}

.focus\:placeholder-red-600:focus::-moz-placeholder {
  color: #e53e3e;
}

.focus\:placeholder-red-600:focus:-ms-input-placeholder {
  color: #e53e3e;
}

.focus\:placeholder-red-600:focus::-ms-input-placeholder {
  color: #e53e3e;
}

.focus\:placeholder-red-600:focus::placeholder {
  color: #e53e3e;
}

.focus\:placeholder-red-700:focus::-webkit-input-placeholder {
  color: #c53030;
}

.focus\:placeholder-red-700:focus::-moz-placeholder {
  color: #c53030;
}

.focus\:placeholder-red-700:focus:-ms-input-placeholder {
  color: #c53030;
}

.focus\:placeholder-red-700:focus::-ms-input-placeholder {
  color: #c53030;
}

.focus\:placeholder-red-700:focus::placeholder {
  color: #c53030;
}

.focus\:placeholder-red-800:focus::-webkit-input-placeholder {
  color: #9b2c2c;
}

.focus\:placeholder-red-800:focus::-moz-placeholder {
  color: #9b2c2c;
}

.focus\:placeholder-red-800:focus:-ms-input-placeholder {
  color: #9b2c2c;
}

.focus\:placeholder-red-800:focus::-ms-input-placeholder {
  color: #9b2c2c;
}

.focus\:placeholder-red-800:focus::placeholder {
  color: #9b2c2c;
}

.focus\:placeholder-red-900:focus::-webkit-input-placeholder {
  color: #742a2a;
}

.focus\:placeholder-red-900:focus::-moz-placeholder {
  color: #742a2a;
}

.focus\:placeholder-red-900:focus:-ms-input-placeholder {
  color: #742a2a;
}

.focus\:placeholder-red-900:focus::-ms-input-placeholder {
  color: #742a2a;
}

.focus\:placeholder-red-900:focus::placeholder {
  color: #742a2a;
}

.focus\:placeholder-orange-100:focus::-webkit-input-placeholder {
  color: #fffaf0;
}

.focus\:placeholder-orange-100:focus::-moz-placeholder {
  color: #fffaf0;
}

.focus\:placeholder-orange-100:focus:-ms-input-placeholder {
  color: #fffaf0;
}

.focus\:placeholder-orange-100:focus::-ms-input-placeholder {
  color: #fffaf0;
}

.focus\:placeholder-orange-100:focus::placeholder {
  color: #fffaf0;
}

.focus\:placeholder-orange-200:focus::-webkit-input-placeholder {
  color: #feebc8;
}

.focus\:placeholder-orange-200:focus::-moz-placeholder {
  color: #feebc8;
}

.focus\:placeholder-orange-200:focus:-ms-input-placeholder {
  color: #feebc8;
}

.focus\:placeholder-orange-200:focus::-ms-input-placeholder {
  color: #feebc8;
}

.focus\:placeholder-orange-200:focus::placeholder {
  color: #feebc8;
}

.focus\:placeholder-orange-300:focus::-webkit-input-placeholder {
  color: #fbd38d;
}

.focus\:placeholder-orange-300:focus::-moz-placeholder {
  color: #fbd38d;
}

.focus\:placeholder-orange-300:focus:-ms-input-placeholder {
  color: #fbd38d;
}

.focus\:placeholder-orange-300:focus::-ms-input-placeholder {
  color: #fbd38d;
}

.focus\:placeholder-orange-300:focus::placeholder {
  color: #fbd38d;
}

.focus\:placeholder-orange-400:focus::-webkit-input-placeholder {
  color: #f6ad55;
}

.focus\:placeholder-orange-400:focus::-moz-placeholder {
  color: #f6ad55;
}

.focus\:placeholder-orange-400:focus:-ms-input-placeholder {
  color: #f6ad55;
}

.focus\:placeholder-orange-400:focus::-ms-input-placeholder {
  color: #f6ad55;
}

.focus\:placeholder-orange-400:focus::placeholder {
  color: #f6ad55;
}

.focus\:placeholder-orange-500:focus::-webkit-input-placeholder {
  color: #ed8936;
}

.focus\:placeholder-orange-500:focus::-moz-placeholder {
  color: #ed8936;
}

.focus\:placeholder-orange-500:focus:-ms-input-placeholder {
  color: #ed8936;
}

.focus\:placeholder-orange-500:focus::-ms-input-placeholder {
  color: #ed8936;
}

.focus\:placeholder-orange-500:focus::placeholder {
  color: #ed8936;
}

.focus\:placeholder-orange-600:focus::-webkit-input-placeholder {
  color: #dd6b20;
}

.focus\:placeholder-orange-600:focus::-moz-placeholder {
  color: #dd6b20;
}

.focus\:placeholder-orange-600:focus:-ms-input-placeholder {
  color: #dd6b20;
}

.focus\:placeholder-orange-600:focus::-ms-input-placeholder {
  color: #dd6b20;
}

.focus\:placeholder-orange-600:focus::placeholder {
  color: #dd6b20;
}

.focus\:placeholder-orange-700:focus::-webkit-input-placeholder {
  color: #c05621;
}

.focus\:placeholder-orange-700:focus::-moz-placeholder {
  color: #c05621;
}

.focus\:placeholder-orange-700:focus:-ms-input-placeholder {
  color: #c05621;
}

.focus\:placeholder-orange-700:focus::-ms-input-placeholder {
  color: #c05621;
}

.focus\:placeholder-orange-700:focus::placeholder {
  color: #c05621;
}

.focus\:placeholder-orange-800:focus::-webkit-input-placeholder {
  color: #9c4221;
}

.focus\:placeholder-orange-800:focus::-moz-placeholder {
  color: #9c4221;
}

.focus\:placeholder-orange-800:focus:-ms-input-placeholder {
  color: #9c4221;
}

.focus\:placeholder-orange-800:focus::-ms-input-placeholder {
  color: #9c4221;
}

.focus\:placeholder-orange-800:focus::placeholder {
  color: #9c4221;
}

.focus\:placeholder-orange-900:focus::-webkit-input-placeholder {
  color: #7b341e;
}

.focus\:placeholder-orange-900:focus::-moz-placeholder {
  color: #7b341e;
}

.focus\:placeholder-orange-900:focus:-ms-input-placeholder {
  color: #7b341e;
}

.focus\:placeholder-orange-900:focus::-ms-input-placeholder {
  color: #7b341e;
}

.focus\:placeholder-orange-900:focus::placeholder {
  color: #7b341e;
}

.focus\:placeholder-yellow-100:focus::-webkit-input-placeholder {
  color: #fffff0;
}

.focus\:placeholder-yellow-100:focus::-moz-placeholder {
  color: #fffff0;
}

.focus\:placeholder-yellow-100:focus:-ms-input-placeholder {
  color: #fffff0;
}

.focus\:placeholder-yellow-100:focus::-ms-input-placeholder {
  color: #fffff0;
}

.focus\:placeholder-yellow-100:focus::placeholder {
  color: #fffff0;
}

.focus\:placeholder-yellow-200:focus::-webkit-input-placeholder {
  color: #fefcbf;
}

.focus\:placeholder-yellow-200:focus::-moz-placeholder {
  color: #fefcbf;
}

.focus\:placeholder-yellow-200:focus:-ms-input-placeholder {
  color: #fefcbf;
}

.focus\:placeholder-yellow-200:focus::-ms-input-placeholder {
  color: #fefcbf;
}

.focus\:placeholder-yellow-200:focus::placeholder {
  color: #fefcbf;
}

.focus\:placeholder-yellow-300:focus::-webkit-input-placeholder {
  color: #faf089;
}

.focus\:placeholder-yellow-300:focus::-moz-placeholder {
  color: #faf089;
}

.focus\:placeholder-yellow-300:focus:-ms-input-placeholder {
  color: #faf089;
}

.focus\:placeholder-yellow-300:focus::-ms-input-placeholder {
  color: #faf089;
}

.focus\:placeholder-yellow-300:focus::placeholder {
  color: #faf089;
}

.focus\:placeholder-yellow-400:focus::-webkit-input-placeholder {
  color: #f6e05e;
}

.focus\:placeholder-yellow-400:focus::-moz-placeholder {
  color: #f6e05e;
}

.focus\:placeholder-yellow-400:focus:-ms-input-placeholder {
  color: #f6e05e;
}

.focus\:placeholder-yellow-400:focus::-ms-input-placeholder {
  color: #f6e05e;
}

.focus\:placeholder-yellow-400:focus::placeholder {
  color: #f6e05e;
}

.focus\:placeholder-yellow-500:focus::-webkit-input-placeholder {
  color: #ecc94b;
}

.focus\:placeholder-yellow-500:focus::-moz-placeholder {
  color: #ecc94b;
}

.focus\:placeholder-yellow-500:focus:-ms-input-placeholder {
  color: #ecc94b;
}

.focus\:placeholder-yellow-500:focus::-ms-input-placeholder {
  color: #ecc94b;
}

.focus\:placeholder-yellow-500:focus::placeholder {
  color: #ecc94b;
}

.focus\:placeholder-yellow-600:focus::-webkit-input-placeholder {
  color: #d69e2e;
}

.focus\:placeholder-yellow-600:focus::-moz-placeholder {
  color: #d69e2e;
}

.focus\:placeholder-yellow-600:focus:-ms-input-placeholder {
  color: #d69e2e;
}

.focus\:placeholder-yellow-600:focus::-ms-input-placeholder {
  color: #d69e2e;
}

.focus\:placeholder-yellow-600:focus::placeholder {
  color: #d69e2e;
}

.focus\:placeholder-yellow-700:focus::-webkit-input-placeholder {
  color: #b7791f;
}

.focus\:placeholder-yellow-700:focus::-moz-placeholder {
  color: #b7791f;
}

.focus\:placeholder-yellow-700:focus:-ms-input-placeholder {
  color: #b7791f;
}

.focus\:placeholder-yellow-700:focus::-ms-input-placeholder {
  color: #b7791f;
}

.focus\:placeholder-yellow-700:focus::placeholder {
  color: #b7791f;
}

.focus\:placeholder-yellow-800:focus::-webkit-input-placeholder {
  color: #975a16;
}

.focus\:placeholder-yellow-800:focus::-moz-placeholder {
  color: #975a16;
}

.focus\:placeholder-yellow-800:focus:-ms-input-placeholder {
  color: #975a16;
}

.focus\:placeholder-yellow-800:focus::-ms-input-placeholder {
  color: #975a16;
}

.focus\:placeholder-yellow-800:focus::placeholder {
  color: #975a16;
}

.focus\:placeholder-yellow-900:focus::-webkit-input-placeholder {
  color: #744210;
}

.focus\:placeholder-yellow-900:focus::-moz-placeholder {
  color: #744210;
}

.focus\:placeholder-yellow-900:focus:-ms-input-placeholder {
  color: #744210;
}

.focus\:placeholder-yellow-900:focus::-ms-input-placeholder {
  color: #744210;
}

.focus\:placeholder-yellow-900:focus::placeholder {
  color: #744210;
}

.focus\:placeholder-green-100:focus::-webkit-input-placeholder {
  color: #f0fff4;
}

.focus\:placeholder-green-100:focus::-moz-placeholder {
  color: #f0fff4;
}

.focus\:placeholder-green-100:focus:-ms-input-placeholder {
  color: #f0fff4;
}

.focus\:placeholder-green-100:focus::-ms-input-placeholder {
  color: #f0fff4;
}

.focus\:placeholder-green-100:focus::placeholder {
  color: #f0fff4;
}

.focus\:placeholder-green-200:focus::-webkit-input-placeholder {
  color: #c6f6d5;
}

.focus\:placeholder-green-200:focus::-moz-placeholder {
  color: #c6f6d5;
}

.focus\:placeholder-green-200:focus:-ms-input-placeholder {
  color: #c6f6d5;
}

.focus\:placeholder-green-200:focus::-ms-input-placeholder {
  color: #c6f6d5;
}

.focus\:placeholder-green-200:focus::placeholder {
  color: #c6f6d5;
}

.focus\:placeholder-green-300:focus::-webkit-input-placeholder {
  color: #9ae6b4;
}

.focus\:placeholder-green-300:focus::-moz-placeholder {
  color: #9ae6b4;
}

.focus\:placeholder-green-300:focus:-ms-input-placeholder {
  color: #9ae6b4;
}

.focus\:placeholder-green-300:focus::-ms-input-placeholder {
  color: #9ae6b4;
}

.focus\:placeholder-green-300:focus::placeholder {
  color: #9ae6b4;
}

.focus\:placeholder-green-400:focus::-webkit-input-placeholder {
  color: #68d391;
}

.focus\:placeholder-green-400:focus::-moz-placeholder {
  color: #68d391;
}

.focus\:placeholder-green-400:focus:-ms-input-placeholder {
  color: #68d391;
}

.focus\:placeholder-green-400:focus::-ms-input-placeholder {
  color: #68d391;
}

.focus\:placeholder-green-400:focus::placeholder {
  color: #68d391;
}

.focus\:placeholder-green-500:focus::-webkit-input-placeholder {
  color: #48bb78;
}

.focus\:placeholder-green-500:focus::-moz-placeholder {
  color: #48bb78;
}

.focus\:placeholder-green-500:focus:-ms-input-placeholder {
  color: #48bb78;
}

.focus\:placeholder-green-500:focus::-ms-input-placeholder {
  color: #48bb78;
}

.focus\:placeholder-green-500:focus::placeholder {
  color: #48bb78;
}

.focus\:placeholder-green-600:focus::-webkit-input-placeholder {
  color: #38a169;
}

.focus\:placeholder-green-600:focus::-moz-placeholder {
  color: #38a169;
}

.focus\:placeholder-green-600:focus:-ms-input-placeholder {
  color: #38a169;
}

.focus\:placeholder-green-600:focus::-ms-input-placeholder {
  color: #38a169;
}

.focus\:placeholder-green-600:focus::placeholder {
  color: #38a169;
}

.focus\:placeholder-green-700:focus::-webkit-input-placeholder {
  color: #2f855a;
}

.focus\:placeholder-green-700:focus::-moz-placeholder {
  color: #2f855a;
}

.focus\:placeholder-green-700:focus:-ms-input-placeholder {
  color: #2f855a;
}

.focus\:placeholder-green-700:focus::-ms-input-placeholder {
  color: #2f855a;
}

.focus\:placeholder-green-700:focus::placeholder {
  color: #2f855a;
}

.focus\:placeholder-green-800:focus::-webkit-input-placeholder {
  color: #276749;
}

.focus\:placeholder-green-800:focus::-moz-placeholder {
  color: #276749;
}

.focus\:placeholder-green-800:focus:-ms-input-placeholder {
  color: #276749;
}

.focus\:placeholder-green-800:focus::-ms-input-placeholder {
  color: #276749;
}

.focus\:placeholder-green-800:focus::placeholder {
  color: #276749;
}

.focus\:placeholder-green-900:focus::-webkit-input-placeholder {
  color: #22543d;
}

.focus\:placeholder-green-900:focus::-moz-placeholder {
  color: #22543d;
}

.focus\:placeholder-green-900:focus:-ms-input-placeholder {
  color: #22543d;
}

.focus\:placeholder-green-900:focus::-ms-input-placeholder {
  color: #22543d;
}

.focus\:placeholder-green-900:focus::placeholder {
  color: #22543d;
}

.focus\:placeholder-teal-100:focus::-webkit-input-placeholder {
  color: #e6fffa;
}

.focus\:placeholder-teal-100:focus::-moz-placeholder {
  color: #e6fffa;
}

.focus\:placeholder-teal-100:focus:-ms-input-placeholder {
  color: #e6fffa;
}

.focus\:placeholder-teal-100:focus::-ms-input-placeholder {
  color: #e6fffa;
}

.focus\:placeholder-teal-100:focus::placeholder {
  color: #e6fffa;
}

.focus\:placeholder-teal-200:focus::-webkit-input-placeholder {
  color: #b2f5ea;
}

.focus\:placeholder-teal-200:focus::-moz-placeholder {
  color: #b2f5ea;
}

.focus\:placeholder-teal-200:focus:-ms-input-placeholder {
  color: #b2f5ea;
}

.focus\:placeholder-teal-200:focus::-ms-input-placeholder {
  color: #b2f5ea;
}

.focus\:placeholder-teal-200:focus::placeholder {
  color: #b2f5ea;
}

.focus\:placeholder-teal-300:focus::-webkit-input-placeholder {
  color: #81e6d9;
}

.focus\:placeholder-teal-300:focus::-moz-placeholder {
  color: #81e6d9;
}

.focus\:placeholder-teal-300:focus:-ms-input-placeholder {
  color: #81e6d9;
}

.focus\:placeholder-teal-300:focus::-ms-input-placeholder {
  color: #81e6d9;
}

.focus\:placeholder-teal-300:focus::placeholder {
  color: #81e6d9;
}

.focus\:placeholder-teal-400:focus::-webkit-input-placeholder {
  color: #4fd1c5;
}

.focus\:placeholder-teal-400:focus::-moz-placeholder {
  color: #4fd1c5;
}

.focus\:placeholder-teal-400:focus:-ms-input-placeholder {
  color: #4fd1c5;
}

.focus\:placeholder-teal-400:focus::-ms-input-placeholder {
  color: #4fd1c5;
}

.focus\:placeholder-teal-400:focus::placeholder {
  color: #4fd1c5;
}

.focus\:placeholder-teal-500:focus::-webkit-input-placeholder {
  color: #38b2ac;
}

.focus\:placeholder-teal-500:focus::-moz-placeholder {
  color: #38b2ac;
}

.focus\:placeholder-teal-500:focus:-ms-input-placeholder {
  color: #38b2ac;
}

.focus\:placeholder-teal-500:focus::-ms-input-placeholder {
  color: #38b2ac;
}

.focus\:placeholder-teal-500:focus::placeholder {
  color: #38b2ac;
}

.focus\:placeholder-teal-600:focus::-webkit-input-placeholder {
  color: #319795;
}

.focus\:placeholder-teal-600:focus::-moz-placeholder {
  color: #319795;
}

.focus\:placeholder-teal-600:focus:-ms-input-placeholder {
  color: #319795;
}

.focus\:placeholder-teal-600:focus::-ms-input-placeholder {
  color: #319795;
}

.focus\:placeholder-teal-600:focus::placeholder {
  color: #319795;
}

.focus\:placeholder-teal-700:focus::-webkit-input-placeholder {
  color: #2c7a7b;
}

.focus\:placeholder-teal-700:focus::-moz-placeholder {
  color: #2c7a7b;
}

.focus\:placeholder-teal-700:focus:-ms-input-placeholder {
  color: #2c7a7b;
}

.focus\:placeholder-teal-700:focus::-ms-input-placeholder {
  color: #2c7a7b;
}

.focus\:placeholder-teal-700:focus::placeholder {
  color: #2c7a7b;
}

.focus\:placeholder-teal-800:focus::-webkit-input-placeholder {
  color: #285e61;
}

.focus\:placeholder-teal-800:focus::-moz-placeholder {
  color: #285e61;
}

.focus\:placeholder-teal-800:focus:-ms-input-placeholder {
  color: #285e61;
}

.focus\:placeholder-teal-800:focus::-ms-input-placeholder {
  color: #285e61;
}

.focus\:placeholder-teal-800:focus::placeholder {
  color: #285e61;
}

.focus\:placeholder-teal-900:focus::-webkit-input-placeholder {
  color: #234e52;
}

.focus\:placeholder-teal-900:focus::-moz-placeholder {
  color: #234e52;
}

.focus\:placeholder-teal-900:focus:-ms-input-placeholder {
  color: #234e52;
}

.focus\:placeholder-teal-900:focus::-ms-input-placeholder {
  color: #234e52;
}

.focus\:placeholder-teal-900:focus::placeholder {
  color: #234e52;
}

.focus\:placeholder-blue-100:focus::-webkit-input-placeholder {
  color: #ebf8ff;
}

.focus\:placeholder-blue-100:focus::-moz-placeholder {
  color: #ebf8ff;
}

.focus\:placeholder-blue-100:focus:-ms-input-placeholder {
  color: #ebf8ff;
}

.focus\:placeholder-blue-100:focus::-ms-input-placeholder {
  color: #ebf8ff;
}

.focus\:placeholder-blue-100:focus::placeholder {
  color: #ebf8ff;
}

.focus\:placeholder-blue-200:focus::-webkit-input-placeholder {
  color: #bee3f8;
}

.focus\:placeholder-blue-200:focus::-moz-placeholder {
  color: #bee3f8;
}

.focus\:placeholder-blue-200:focus:-ms-input-placeholder {
  color: #bee3f8;
}

.focus\:placeholder-blue-200:focus::-ms-input-placeholder {
  color: #bee3f8;
}

.focus\:placeholder-blue-200:focus::placeholder {
  color: #bee3f8;
}

.focus\:placeholder-blue-300:focus::-webkit-input-placeholder {
  color: #90cdf4;
}

.focus\:placeholder-blue-300:focus::-moz-placeholder {
  color: #90cdf4;
}

.focus\:placeholder-blue-300:focus:-ms-input-placeholder {
  color: #90cdf4;
}

.focus\:placeholder-blue-300:focus::-ms-input-placeholder {
  color: #90cdf4;
}

.focus\:placeholder-blue-300:focus::placeholder {
  color: #90cdf4;
}

.focus\:placeholder-blue-400:focus::-webkit-input-placeholder {
  color: #63b3ed;
}

.focus\:placeholder-blue-400:focus::-moz-placeholder {
  color: #63b3ed;
}

.focus\:placeholder-blue-400:focus:-ms-input-placeholder {
  color: #63b3ed;
}

.focus\:placeholder-blue-400:focus::-ms-input-placeholder {
  color: #63b3ed;
}

.focus\:placeholder-blue-400:focus::placeholder {
  color: #63b3ed;
}

.focus\:placeholder-blue-500:focus::-webkit-input-placeholder {
  color: #4299e1;
}

.focus\:placeholder-blue-500:focus::-moz-placeholder {
  color: #4299e1;
}

.focus\:placeholder-blue-500:focus:-ms-input-placeholder {
  color: #4299e1;
}

.focus\:placeholder-blue-500:focus::-ms-input-placeholder {
  color: #4299e1;
}

.focus\:placeholder-blue-500:focus::placeholder {
  color: #4299e1;
}

.focus\:placeholder-blue-600:focus::-webkit-input-placeholder {
  color: #3182ce;
}

.focus\:placeholder-blue-600:focus::-moz-placeholder {
  color: #3182ce;
}

.focus\:placeholder-blue-600:focus:-ms-input-placeholder {
  color: #3182ce;
}

.focus\:placeholder-blue-600:focus::-ms-input-placeholder {
  color: #3182ce;
}

.focus\:placeholder-blue-600:focus::placeholder {
  color: #3182ce;
}

.focus\:placeholder-blue-700:focus::-webkit-input-placeholder {
  color: #2b6cb0;
}

.focus\:placeholder-blue-700:focus::-moz-placeholder {
  color: #2b6cb0;
}

.focus\:placeholder-blue-700:focus:-ms-input-placeholder {
  color: #2b6cb0;
}

.focus\:placeholder-blue-700:focus::-ms-input-placeholder {
  color: #2b6cb0;
}

.focus\:placeholder-blue-700:focus::placeholder {
  color: #2b6cb0;
}

.focus\:placeholder-blue-800:focus::-webkit-input-placeholder {
  color: #2c5282;
}

.focus\:placeholder-blue-800:focus::-moz-placeholder {
  color: #2c5282;
}

.focus\:placeholder-blue-800:focus:-ms-input-placeholder {
  color: #2c5282;
}

.focus\:placeholder-blue-800:focus::-ms-input-placeholder {
  color: #2c5282;
}

.focus\:placeholder-blue-800:focus::placeholder {
  color: #2c5282;
}

.focus\:placeholder-blue-900:focus::-webkit-input-placeholder {
  color: #2a4365;
}

.focus\:placeholder-blue-900:focus::-moz-placeholder {
  color: #2a4365;
}

.focus\:placeholder-blue-900:focus:-ms-input-placeholder {
  color: #2a4365;
}

.focus\:placeholder-blue-900:focus::-ms-input-placeholder {
  color: #2a4365;
}

.focus\:placeholder-blue-900:focus::placeholder {
  color: #2a4365;
}

.focus\:placeholder-indigo-100:focus::-webkit-input-placeholder {
  color: #ebf4ff;
}

.focus\:placeholder-indigo-100:focus::-moz-placeholder {
  color: #ebf4ff;
}

.focus\:placeholder-indigo-100:focus:-ms-input-placeholder {
  color: #ebf4ff;
}

.focus\:placeholder-indigo-100:focus::-ms-input-placeholder {
  color: #ebf4ff;
}

.focus\:placeholder-indigo-100:focus::placeholder {
  color: #ebf4ff;
}

.focus\:placeholder-indigo-200:focus::-webkit-input-placeholder {
  color: #c3dafe;
}

.focus\:placeholder-indigo-200:focus::-moz-placeholder {
  color: #c3dafe;
}

.focus\:placeholder-indigo-200:focus:-ms-input-placeholder {
  color: #c3dafe;
}

.focus\:placeholder-indigo-200:focus::-ms-input-placeholder {
  color: #c3dafe;
}

.focus\:placeholder-indigo-200:focus::placeholder {
  color: #c3dafe;
}

.focus\:placeholder-indigo-300:focus::-webkit-input-placeholder {
  color: #a3bffa;
}

.focus\:placeholder-indigo-300:focus::-moz-placeholder {
  color: #a3bffa;
}

.focus\:placeholder-indigo-300:focus:-ms-input-placeholder {
  color: #a3bffa;
}

.focus\:placeholder-indigo-300:focus::-ms-input-placeholder {
  color: #a3bffa;
}

.focus\:placeholder-indigo-300:focus::placeholder {
  color: #a3bffa;
}

.focus\:placeholder-indigo-400:focus::-webkit-input-placeholder {
  color: #7f9cf5;
}

.focus\:placeholder-indigo-400:focus::-moz-placeholder {
  color: #7f9cf5;
}

.focus\:placeholder-indigo-400:focus:-ms-input-placeholder {
  color: #7f9cf5;
}

.focus\:placeholder-indigo-400:focus::-ms-input-placeholder {
  color: #7f9cf5;
}

.focus\:placeholder-indigo-400:focus::placeholder {
  color: #7f9cf5;
}

.focus\:placeholder-indigo-500:focus::-webkit-input-placeholder {
  color: #667eea;
}

.focus\:placeholder-indigo-500:focus::-moz-placeholder {
  color: #667eea;
}

.focus\:placeholder-indigo-500:focus:-ms-input-placeholder {
  color: #667eea;
}

.focus\:placeholder-indigo-500:focus::-ms-input-placeholder {
  color: #667eea;
}

.focus\:placeholder-indigo-500:focus::placeholder {
  color: #667eea;
}

.focus\:placeholder-indigo-600:focus::-webkit-input-placeholder {
  color: #5a67d8;
}

.focus\:placeholder-indigo-600:focus::-moz-placeholder {
  color: #5a67d8;
}

.focus\:placeholder-indigo-600:focus:-ms-input-placeholder {
  color: #5a67d8;
}

.focus\:placeholder-indigo-600:focus::-ms-input-placeholder {
  color: #5a67d8;
}

.focus\:placeholder-indigo-600:focus::placeholder {
  color: #5a67d8;
}

.focus\:placeholder-indigo-700:focus::-webkit-input-placeholder {
  color: #4c51bf;
}

.focus\:placeholder-indigo-700:focus::-moz-placeholder {
  color: #4c51bf;
}

.focus\:placeholder-indigo-700:focus:-ms-input-placeholder {
  color: #4c51bf;
}

.focus\:placeholder-indigo-700:focus::-ms-input-placeholder {
  color: #4c51bf;
}

.focus\:placeholder-indigo-700:focus::placeholder {
  color: #4c51bf;
}

.focus\:placeholder-indigo-800:focus::-webkit-input-placeholder {
  color: #434190;
}

.focus\:placeholder-indigo-800:focus::-moz-placeholder {
  color: #434190;
}

.focus\:placeholder-indigo-800:focus:-ms-input-placeholder {
  color: #434190;
}

.focus\:placeholder-indigo-800:focus::-ms-input-placeholder {
  color: #434190;
}

.focus\:placeholder-indigo-800:focus::placeholder {
  color: #434190;
}

.focus\:placeholder-indigo-900:focus::-webkit-input-placeholder {
  color: #3c366b;
}

.focus\:placeholder-indigo-900:focus::-moz-placeholder {
  color: #3c366b;
}

.focus\:placeholder-indigo-900:focus:-ms-input-placeholder {
  color: #3c366b;
}

.focus\:placeholder-indigo-900:focus::-ms-input-placeholder {
  color: #3c366b;
}

.focus\:placeholder-indigo-900:focus::placeholder {
  color: #3c366b;
}

.focus\:placeholder-purple-100:focus::-webkit-input-placeholder {
  color: #faf5ff;
}

.focus\:placeholder-purple-100:focus::-moz-placeholder {
  color: #faf5ff;
}

.focus\:placeholder-purple-100:focus:-ms-input-placeholder {
  color: #faf5ff;
}

.focus\:placeholder-purple-100:focus::-ms-input-placeholder {
  color: #faf5ff;
}

.focus\:placeholder-purple-100:focus::placeholder {
  color: #faf5ff;
}

.focus\:placeholder-purple-200:focus::-webkit-input-placeholder {
  color: #e9d8fd;
}

.focus\:placeholder-purple-200:focus::-moz-placeholder {
  color: #e9d8fd;
}

.focus\:placeholder-purple-200:focus:-ms-input-placeholder {
  color: #e9d8fd;
}

.focus\:placeholder-purple-200:focus::-ms-input-placeholder {
  color: #e9d8fd;
}

.focus\:placeholder-purple-200:focus::placeholder {
  color: #e9d8fd;
}

.focus\:placeholder-purple-300:focus::-webkit-input-placeholder {
  color: #d6bcfa;
}

.focus\:placeholder-purple-300:focus::-moz-placeholder {
  color: #d6bcfa;
}

.focus\:placeholder-purple-300:focus:-ms-input-placeholder {
  color: #d6bcfa;
}

.focus\:placeholder-purple-300:focus::-ms-input-placeholder {
  color: #d6bcfa;
}

.focus\:placeholder-purple-300:focus::placeholder {
  color: #d6bcfa;
}

.focus\:placeholder-purple-400:focus::-webkit-input-placeholder {
  color: #b794f4;
}

.focus\:placeholder-purple-400:focus::-moz-placeholder {
  color: #b794f4;
}

.focus\:placeholder-purple-400:focus:-ms-input-placeholder {
  color: #b794f4;
}

.focus\:placeholder-purple-400:focus::-ms-input-placeholder {
  color: #b794f4;
}

.focus\:placeholder-purple-400:focus::placeholder {
  color: #b794f4;
}

.focus\:placeholder-purple-500:focus::-webkit-input-placeholder {
  color: #9f7aea;
}

.focus\:placeholder-purple-500:focus::-moz-placeholder {
  color: #9f7aea;
}

.focus\:placeholder-purple-500:focus:-ms-input-placeholder {
  color: #9f7aea;
}

.focus\:placeholder-purple-500:focus::-ms-input-placeholder {
  color: #9f7aea;
}

.focus\:placeholder-purple-500:focus::placeholder {
  color: #9f7aea;
}

.focus\:placeholder-purple-600:focus::-webkit-input-placeholder {
  color: #805ad5;
}

.focus\:placeholder-purple-600:focus::-moz-placeholder {
  color: #805ad5;
}

.focus\:placeholder-purple-600:focus:-ms-input-placeholder {
  color: #805ad5;
}

.focus\:placeholder-purple-600:focus::-ms-input-placeholder {
  color: #805ad5;
}

.focus\:placeholder-purple-600:focus::placeholder {
  color: #805ad5;
}

.focus\:placeholder-purple-700:focus::-webkit-input-placeholder {
  color: #6b46c1;
}

.focus\:placeholder-purple-700:focus::-moz-placeholder {
  color: #6b46c1;
}

.focus\:placeholder-purple-700:focus:-ms-input-placeholder {
  color: #6b46c1;
}

.focus\:placeholder-purple-700:focus::-ms-input-placeholder {
  color: #6b46c1;
}

.focus\:placeholder-purple-700:focus::placeholder {
  color: #6b46c1;
}

.focus\:placeholder-purple-800:focus::-webkit-input-placeholder {
  color: #553c9a;
}

.focus\:placeholder-purple-800:focus::-moz-placeholder {
  color: #553c9a;
}

.focus\:placeholder-purple-800:focus:-ms-input-placeholder {
  color: #553c9a;
}

.focus\:placeholder-purple-800:focus::-ms-input-placeholder {
  color: #553c9a;
}

.focus\:placeholder-purple-800:focus::placeholder {
  color: #553c9a;
}

.focus\:placeholder-purple-900:focus::-webkit-input-placeholder {
  color: #44337a;
}

.focus\:placeholder-purple-900:focus::-moz-placeholder {
  color: #44337a;
}

.focus\:placeholder-purple-900:focus:-ms-input-placeholder {
  color: #44337a;
}

.focus\:placeholder-purple-900:focus::-ms-input-placeholder {
  color: #44337a;
}

.focus\:placeholder-purple-900:focus::placeholder {
  color: #44337a;
}

.focus\:placeholder-pink-100:focus::-webkit-input-placeholder {
  color: #fff5f7;
}

.focus\:placeholder-pink-100:focus::-moz-placeholder {
  color: #fff5f7;
}

.focus\:placeholder-pink-100:focus:-ms-input-placeholder {
  color: #fff5f7;
}

.focus\:placeholder-pink-100:focus::-ms-input-placeholder {
  color: #fff5f7;
}

.focus\:placeholder-pink-100:focus::placeholder {
  color: #fff5f7;
}

.focus\:placeholder-pink-200:focus::-webkit-input-placeholder {
  color: #fed7e2;
}

.focus\:placeholder-pink-200:focus::-moz-placeholder {
  color: #fed7e2;
}

.focus\:placeholder-pink-200:focus:-ms-input-placeholder {
  color: #fed7e2;
}

.focus\:placeholder-pink-200:focus::-ms-input-placeholder {
  color: #fed7e2;
}

.focus\:placeholder-pink-200:focus::placeholder {
  color: #fed7e2;
}

.focus\:placeholder-pink-300:focus::-webkit-input-placeholder {
  color: #fbb6ce;
}

.focus\:placeholder-pink-300:focus::-moz-placeholder {
  color: #fbb6ce;
}

.focus\:placeholder-pink-300:focus:-ms-input-placeholder {
  color: #fbb6ce;
}

.focus\:placeholder-pink-300:focus::-ms-input-placeholder {
  color: #fbb6ce;
}

.focus\:placeholder-pink-300:focus::placeholder {
  color: #fbb6ce;
}

.focus\:placeholder-pink-400:focus::-webkit-input-placeholder {
  color: #f687b3;
}

.focus\:placeholder-pink-400:focus::-moz-placeholder {
  color: #f687b3;
}

.focus\:placeholder-pink-400:focus:-ms-input-placeholder {
  color: #f687b3;
}

.focus\:placeholder-pink-400:focus::-ms-input-placeholder {
  color: #f687b3;
}

.focus\:placeholder-pink-400:focus::placeholder {
  color: #f687b3;
}

.focus\:placeholder-pink-500:focus::-webkit-input-placeholder {
  color: #ed64a6;
}

.focus\:placeholder-pink-500:focus::-moz-placeholder {
  color: #ed64a6;
}

.focus\:placeholder-pink-500:focus:-ms-input-placeholder {
  color: #ed64a6;
}

.focus\:placeholder-pink-500:focus::-ms-input-placeholder {
  color: #ed64a6;
}

.focus\:placeholder-pink-500:focus::placeholder {
  color: #ed64a6;
}

.focus\:placeholder-pink-600:focus::-webkit-input-placeholder {
  color: #d53f8c;
}

.focus\:placeholder-pink-600:focus::-moz-placeholder {
  color: #d53f8c;
}

.focus\:placeholder-pink-600:focus:-ms-input-placeholder {
  color: #d53f8c;
}

.focus\:placeholder-pink-600:focus::-ms-input-placeholder {
  color: #d53f8c;
}

.focus\:placeholder-pink-600:focus::placeholder {
  color: #d53f8c;
}

.focus\:placeholder-pink-700:focus::-webkit-input-placeholder {
  color: #b83280;
}

.focus\:placeholder-pink-700:focus::-moz-placeholder {
  color: #b83280;
}

.focus\:placeholder-pink-700:focus:-ms-input-placeholder {
  color: #b83280;
}

.focus\:placeholder-pink-700:focus::-ms-input-placeholder {
  color: #b83280;
}

.focus\:placeholder-pink-700:focus::placeholder {
  color: #b83280;
}

.focus\:placeholder-pink-800:focus::-webkit-input-placeholder {
  color: #97266d;
}

.focus\:placeholder-pink-800:focus::-moz-placeholder {
  color: #97266d;
}

.focus\:placeholder-pink-800:focus:-ms-input-placeholder {
  color: #97266d;
}

.focus\:placeholder-pink-800:focus::-ms-input-placeholder {
  color: #97266d;
}

.focus\:placeholder-pink-800:focus::placeholder {
  color: #97266d;
}

.focus\:placeholder-pink-900:focus::-webkit-input-placeholder {
  color: #702459;
}

.focus\:placeholder-pink-900:focus::-moz-placeholder {
  color: #702459;
}

.focus\:placeholder-pink-900:focus:-ms-input-placeholder {
  color: #702459;
}

.focus\:placeholder-pink-900:focus::-ms-input-placeholder {
  color: #702459;
}

.focus\:placeholder-pink-900:focus::placeholder {
  color: #702459;
}

.pointer-events-none {
  pointer-events: none;
}

.pointer-events-auto {
  pointer-events: auto;
}

.static {
  position: static;
}

.fixed {
  position: fixed;
}

.absolute {
  position: absolute;
}

.relative {
  position: relative;
}

.sticky {
  position: -webkit-sticky;
  position: sticky;
}

.inset-0 {
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
}

.inset-auto {
  top: auto;
  right: auto;
  bottom: auto;
  left: auto;
}

.inset-y-0 {
  top: 0;
  bottom: 0;
}

.inset-x-0 {
  right: 0;
  left: 0;
}

.inset-y-auto {
  top: auto;
  bottom: auto;
}

.inset-x-auto {
  right: auto;
  left: auto;
}

.top-0 {
  top: 0;
}

.right-0 {
  right: 0;
}

.bottom-0 {
  bottom: 0;
}

.left-0 {
  left: 0;
}

.top-auto {
  top: auto;
}

.right-auto {
  right: auto;
}

.bottom-auto {
  bottom: auto;
}

.left-auto {
  left: auto;
}

.resize-none {
  resize: none;
}

.resize-y {
  resize: vertical;
}

.resize-x {
  resize: horizontal;
}

.resize {
  resize: both;
}

.shadow {
  box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06);
}

.shadow-md {
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
}

.shadow-lg {
  box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
}

.shadow-xl {
  box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
}

.shadow-2xl {
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
}

.shadow-inner {
  box-shadow: inset 0 2px 4px 0 rgba(0, 0, 0, 0.06);
}

.shadow-outline {
  box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.5);
}

.shadow-none {
  box-shadow: none;
}

.hover\:shadow:hover {
  box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06);
}

.hover\:shadow-md:hover {
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
}

.hover\:shadow-lg:hover {
  box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
}

.hover\:shadow-xl:hover {
  box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
}

.hover\:shadow-2xl:hover {
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
}

.hover\:shadow-inner:hover {
  box-shadow: inset 0 2px 4px 0 rgba(0, 0, 0, 0.06);
}

.hover\:shadow-outline:hover {
  box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.5);
}

.hover\:shadow-none:hover {
  box-shadow: none;
}

.focus\:shadow:focus {
  box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06);
}

.focus\:shadow-md:focus {
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
}

.focus\:shadow-lg:focus {
  box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
}

.focus\:shadow-xl:focus {
  box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
}

.focus\:shadow-2xl:focus {
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
}

.focus\:shadow-inner:focus {
  box-shadow: inset 0 2px 4px 0 rgba(0, 0, 0, 0.06);
}

.focus\:shadow-outline:focus {
  box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.5);
}

.focus\:shadow-none:focus {
  box-shadow: none;
}

.fill-current {
  fill: currentColor;
}

.stroke-current {
  stroke: currentColor;
}

.table-auto {
  table-layout: auto;
}

.table-fixed {
  table-layout: fixed;
}

.text-left {
  text-align: left;
}

.text-center {
  text-align: center;
}

.text-right {
  text-align: right;
}

.text-justify {
  text-align: justify;
}

.text-transparent {
  color: transparent;
}

.text-black {
  color: #000;
}

.text-white {
  color: #fff;
}

.text-gray-100 {
  color: #f7fafc;
}

.text-gray-200 {
  color: #edf2f7;
}

.text-gray-300 {
  color: #e2e8f0;
}

.text-gray-400 {
  color: #cbd5e0;
}

.text-gray-500 {
  color: #a0aec0;
}

.text-gray-600 {
  color: #718096;
}

.text-gray-700 {
  color: #4a5568;
}

.text-gray-800 {
  color: #2d3748;
}

.text-gray-900 {
  color: #1a202c;
}

.text-red-100 {
  color: #fff5f5;
}

.text-red-200 {
  color: #fed7d7;
}

.text-red-300 {
  color: #feb2b2;
}

.text-red-400 {
  color: #fc8181;
}

.text-red-500 {
  color: #f56565;
}

.text-red-600 {
  color: #e53e3e;
}

.text-red-700 {
  color: #c53030;
}

.text-red-800 {
  color: #9b2c2c;
}

.text-red-900 {
  color: #742a2a;
}

.text-orange-100 {
  color: #fffaf0;
}

.text-orange-200 {
  color: #feebc8;
}

.text-orange-300 {
  color: #fbd38d;
}

.text-orange-400 {
  color: #f6ad55;
}

.text-orange-500 {
  color: #ed8936;
}

.text-orange-600 {
  color: #dd6b20;
}

.text-orange-700 {
  color: #c05621;
}

.text-orange-800 {
  color: #9c4221;
}

.text-orange-900 {
  color: #7b341e;
}

.text-yellow-100 {
  color: #fffff0;
}

.text-yellow-200 {
  color: #fefcbf;
}

.text-yellow-300 {
  color: #faf089;
}

.text-yellow-400 {
  color: #f6e05e;
}

.text-yellow-500 {
  color: #ecc94b;
}

.text-yellow-600 {
  color: #d69e2e;
}

.text-yellow-700 {
  color: #b7791f;
}

.text-yellow-800 {
  color: #975a16;
}

.text-yellow-900 {
  color: #744210;
}

.text-green-100 {
  color: #f0fff4;
}

.text-green-200 {
  color: #c6f6d5;
}

.text-green-300 {
  color: #9ae6b4;
}

.text-green-400 {
  color: #68d391;
}

.text-green-500 {
  color: #48bb78;
}

.text-green-600 {
  color: #38a169;
}

.text-green-700 {
  color: #2f855a;
}

.text-green-800 {
  color: #276749;
}

.text-green-900 {
  color: #22543d;
}

.text-teal-100 {
  color: #e6fffa;
}

.text-teal-200 {
  color: #b2f5ea;
}

.text-teal-300 {
  color: #81e6d9;
}

.text-teal-400 {
  color: #4fd1c5;
}

.text-teal-500 {
  color: #38b2ac;
}

.text-teal-600 {
  color: #319795;
}

.text-teal-700 {
  color: #2c7a7b;
}

.text-teal-800 {
  color: #285e61;
}

.text-teal-900 {
  color: #234e52;
}

.text-blue-100 {
  color: #ebf8ff;
}

.text-blue-200 {
  color: #bee3f8;
}

.text-blue-300 {
  color: #90cdf4;
}

.text-blue-400 {
  color: #63b3ed;
}

.text-blue-500 {
  color: #4299e1;
}

.text-blue-600 {
  color: #3182ce;
}

.text-blue-700 {
  color: #2b6cb0;
}

.text-blue-800 {
  color: #2c5282;
}

.text-blue-900 {
  color: #2a4365;
}

.text-indigo-100 {
  color: #ebf4ff;
}

.text-indigo-200 {
  color: #c3dafe;
}

.text-indigo-300 {
  color: #a3bffa;
}

.text-indigo-400 {
  color: #7f9cf5;
}

.text-indigo-500 {
  color: #667eea;
}

.text-indigo-600 {
  color: #5a67d8;
}

.text-indigo-700 {
  color: #4c51bf;
}

.text-indigo-800 {
  color: #434190;
}

.text-indigo-900 {
  color: #3c366b;
}

.text-purple-100 {
  color: #faf5ff;
}

.text-purple-200 {
  color: #e9d8fd;
}

.text-purple-300 {
  color: #d6bcfa;
}

.text-purple-400 {
  color: #b794f4;
}

.text-purple-500 {
  color: #9f7aea;
}

.text-purple-600 {
  color: #805ad5;
}

.text-purple-700 {
  color: #6b46c1;
}

.text-purple-800 {
  color: #553c9a;
}

.text-purple-900 {
  color: #44337a;
}

.text-pink-100 {
  color: #fff5f7;
}

.text-pink-200 {
  color: #fed7e2;
}

.text-pink-300 {
  color: #fbb6ce;
}

.text-pink-400 {
  color: #f687b3;
}

.text-pink-500 {
  color: #ed64a6;
}

.text-pink-600 {
  color: #d53f8c;
}

.text-pink-700 {
  color: #b83280;
}

.text-pink-800 {
  color: #97266d;
}

.text-pink-900 {
  color: #702459;
}

.hover\:text-transparent:hover {
  color: transparent;
}

.hover\:text-black:hover {
  color: #000;
}

.hover\:text-white:hover {
  color: #fff;
}

.hover\:text-gray-100:hover {
  color: #f7fafc;
}

.hover\:text-gray-200:hover {
  color: #edf2f7;
}

.hover\:text-gray-300:hover {
  color: #e2e8f0;
}

.hover\:text-gray-400:hover {
  color: #cbd5e0;
}

.hover\:text-gray-500:hover {
  color: #a0aec0;
}

.hover\:text-gray-600:hover {
  color: #718096;
}

.hover\:text-gray-700:hover {
  color: #4a5568;
}

.hover\:text-gray-800:hover {
  color: #2d3748;
}

.hover\:text-gray-900:hover {
  color: #1a202c;
}

.hover\:text-red-100:hover {
  color: #fff5f5;
}

.hover\:text-red-200:hover {
  color: #fed7d7;
}

.hover\:text-red-300:hover {
  color: #feb2b2;
}

.hover\:text-red-400:hover {
  color: #fc8181;
}

.hover\:text-red-500:hover {
  color: #f56565;
}

.hover\:text-red-600:hover {
  color: #e53e3e;
}

.hover\:text-red-700:hover {
  color: #c53030;
}

.hover\:text-red-800:hover {
  color: #9b2c2c;
}

.hover\:text-red-900:hover {
  color: #742a2a;
}

.hover\:text-orange-100:hover {
  color: #fffaf0;
}

.hover\:text-orange-200:hover {
  color: #feebc8;
}

.hover\:text-orange-300:hover {
  color: #fbd38d;
}

.hover\:text-orange-400:hover {
  color: #f6ad55;
}

.hover\:text-orange-500:hover {
  color: #ed8936;
}

.hover\:text-orange-600:hover {
  color: #dd6b20;
}

.hover\:text-orange-700:hover {
  color: #c05621;
}

.hover\:text-orange-800:hover {
  color: #9c4221;
}

.hover\:text-orange-900:hover {
  color: #7b341e;
}

.hover\:text-yellow-100:hover {
  color: #fffff0;
}

.hover\:text-yellow-200:hover {
  color: #fefcbf;
}

.hover\:text-yellow-300:hover {
  color: #faf089;
}

.hover\:text-yellow-400:hover {
  color: #f6e05e;
}

.hover\:text-yellow-500:hover {
  color: #ecc94b;
}

.hover\:text-yellow-600:hover {
  color: #d69e2e;
}

.hover\:text-yellow-700:hover {
  color: #b7791f;
}

.hover\:text-yellow-800:hover {
  color: #975a16;
}

.hover\:text-yellow-900:hover {
  color: #744210;
}

.hover\:text-green-100:hover {
  color: #f0fff4;
}

.hover\:text-green-200:hover {
  color: #c6f6d5;
}

.hover\:text-green-300:hover {
  color: #9ae6b4;
}

.hover\:text-green-400:hover {
  color: #68d391;
}

.hover\:text-green-500:hover {
  color: #48bb78;
}

.hover\:text-green-600:hover {
  color: #38a169;
}

.hover\:text-green-700:hover {
  color: #2f855a;
}

.hover\:text-green-800:hover {
  color: #276749;
}

.hover\:text-green-900:hover {
  color: #22543d;
}

.hover\:text-teal-100:hover {
  color: #e6fffa;
}

.hover\:text-teal-200:hover {
  color: #b2f5ea;
}

.hover\:text-teal-300:hover {
  color: #81e6d9;
}

.hover\:text-teal-400:hover {
  color: #4fd1c5;
}

.hover\:text-teal-500:hover {
  color: #38b2ac;
}

.hover\:text-teal-600:hover {
  color: #319795;
}

.hover\:text-teal-700:hover {
  color: #2c7a7b;
}

.hover\:text-teal-800:hover {
  color: #285e61;
}

.hover\:text-teal-900:hover {
  color: #234e52;
}

.hover\:text-blue-100:hover {
  color: #ebf8ff;
}

.hover\:text-blue-200:hover {
  color: #bee3f8;
}

.hover\:text-blue-300:hover {
  color: #90cdf4;
}

.hover\:text-blue-400:hover {
  color: #63b3ed;
}

.hover\:text-blue-500:hover {
  color: #4299e1;
}

.hover\:text-blue-600:hover {
  color: #3182ce;
}

.hover\:text-blue-700:hover {
  color: #2b6cb0;
}

.hover\:text-blue-800:hover {
  color: #2c5282;
}

.hover\:text-blue-900:hover {
  color: #2a4365;
}

.hover\:text-indigo-100:hover {
  color: #ebf4ff;
}

.hover\:text-indigo-200:hover {
  color: #c3dafe;
}

.hover\:text-indigo-300:hover {
  color: #a3bffa;
}

.hover\:text-indigo-400:hover {
  color: #7f9cf5;
}

.hover\:text-indigo-500:hover {
  color: #667eea;
}

.hover\:text-indigo-600:hover {
  color: #5a67d8;
}

.hover\:text-indigo-700:hover {
  color: #4c51bf;
}

.hover\:text-indigo-800:hover {
  color: #434190;
}

.hover\:text-indigo-900:hover {
  color: #3c366b;
}

.hover\:text-purple-100:hover {
  color: #faf5ff;
}

.hover\:text-purple-200:hover {
  color: #e9d8fd;
}

.hover\:text-purple-300:hover {
  color: #d6bcfa;
}

.hover\:text-purple-400:hover {
  color: #b794f4;
}

.hover\:text-purple-500:hover {
  color: #9f7aea;
}

.hover\:text-purple-600:hover {
  color: #805ad5;
}

.hover\:text-purple-700:hover {
  color: #6b46c1;
}

.hover\:text-purple-800:hover {
  color: #553c9a;
}

.hover\:text-purple-900:hover {
  color: #44337a;
}

.hover\:text-pink-100:hover {
  color: #fff5f7;
}

.hover\:text-pink-200:hover {
  color: #fed7e2;
}

.hover\:text-pink-300:hover {
  color: #fbb6ce;
}

.hover\:text-pink-400:hover {
  color: #f687b3;
}

.hover\:text-pink-500:hover {
  color: #ed64a6;
}

.hover\:text-pink-600:hover {
  color: #d53f8c;
}

.hover\:text-pink-700:hover {
  color: #b83280;
}

.hover\:text-pink-800:hover {
  color: #97266d;
}

.hover\:text-pink-900:hover {
  color: #702459;
}

.focus\:text-transparent:focus {
  color: transparent;
}

.focus\:text-black:focus {
  color: #000;
}

.focus\:text-white:focus {
  color: #fff;
}

.focus\:text-gray-100:focus {
  color: #f7fafc;
}

.focus\:text-gray-200:focus {
  color: #edf2f7;
}

.focus\:text-gray-300:focus {
  color: #e2e8f0;
}

.focus\:text-gray-400:focus {
  color: #cbd5e0;
}

.focus\:text-gray-500:focus {
  color: #a0aec0;
}

.focus\:text-gray-600:focus {
  color: #718096;
}

.focus\:text-gray-700:focus {
  color: #4a5568;
}

.focus\:text-gray-800:focus {
  color: #2d3748;
}

.focus\:text-gray-900:focus {
  color: #1a202c;
}

.focus\:text-red-100:focus {
  color: #fff5f5;
}

.focus\:text-red-200:focus {
  color: #fed7d7;
}

.focus\:text-red-300:focus {
  color: #feb2b2;
}

.focus\:text-red-400:focus {
  color: #fc8181;
}

.focus\:text-red-500:focus {
  color: #f56565;
}

.focus\:text-red-600:focus {
  color: #e53e3e;
}

.focus\:text-red-700:focus {
  color: #c53030;
}

.focus\:text-red-800:focus {
  color: #9b2c2c;
}

.focus\:text-red-900:focus {
  color: #742a2a;
}

.focus\:text-orange-100:focus {
  color: #fffaf0;
}

.focus\:text-orange-200:focus {
  color: #feebc8;
}

.focus\:text-orange-300:focus {
  color: #fbd38d;
}

.focus\:text-orange-400:focus {
  color: #f6ad55;
}

.focus\:text-orange-500:focus {
  color: #ed8936;
}

.focus\:text-orange-600:focus {
  color: #dd6b20;
}

.focus\:text-orange-700:focus {
  color: #c05621;
}

.focus\:text-orange-800:focus {
  color: #9c4221;
}

.focus\:text-orange-900:focus {
  color: #7b341e;
}

.focus\:text-yellow-100:focus {
  color: #fffff0;
}

.focus\:text-yellow-200:focus {
  color: #fefcbf;
}

.focus\:text-yellow-300:focus {
  color: #faf089;
}

.focus\:text-yellow-400:focus {
  color: #f6e05e;
}

.focus\:text-yellow-500:focus {
  color: #ecc94b;
}

.focus\:text-yellow-600:focus {
  color: #d69e2e;
}

.focus\:text-yellow-700:focus {
  color: #b7791f;
}

.focus\:text-yellow-800:focus {
  color: #975a16;
}

.focus\:text-yellow-900:focus {
  color: #744210;
}

.focus\:text-green-100:focus {
  color: #f0fff4;
}

.focus\:text-green-200:focus {
  color: #c6f6d5;
}

.focus\:text-green-300:focus {
  color: #9ae6b4;
}

.focus\:text-green-400:focus {
  color: #68d391;
}

.focus\:text-green-500:focus {
  color: #48bb78;
}

.focus\:text-green-600:focus {
  color: #38a169;
}

.focus\:text-green-700:focus {
  color: #2f855a;
}

.focus\:text-green-800:focus {
  color: #276749;
}

.focus\:text-green-900:focus {
  color: #22543d;
}

.focus\:text-teal-100:focus {
  color: #e6fffa;
}

.focus\:text-teal-200:focus {
  color: #b2f5ea;
}

.focus\:text-teal-300:focus {
  color: #81e6d9;
}

.focus\:text-teal-400:focus {
  color: #4fd1c5;
}

.focus\:text-teal-500:focus {
  color: #38b2ac;
}

.focus\:text-teal-600:focus {
  color: #319795;
}

.focus\:text-teal-700:focus {
  color: #2c7a7b;
}

.focus\:text-teal-800:focus {
  color: #285e61;
}

.focus\:text-teal-900:focus {
  color: #234e52;
}

.focus\:text-blue-100:focus {
  color: #ebf8ff;
}

.focus\:text-blue-200:focus {
  color: #bee3f8;
}

.focus\:text-blue-300:focus {
  color: #90cdf4;
}

.focus\:text-blue-400:focus {
  color: #63b3ed;
}

.focus\:text-blue-500:focus {
  color: #4299e1;
}

.focus\:text-blue-600:focus {
  color: #3182ce;
}

.focus\:text-blue-700:focus {
  color: #2b6cb0;
}

.focus\:text-blue-800:focus {
  color: #2c5282;
}

.focus\:text-blue-900:focus {
  color: #2a4365;
}

.focus\:text-indigo-100:focus {
  color: #ebf4ff;
}

.focus\:text-indigo-200:focus {
  color: #c3dafe;
}

.focus\:text-indigo-300:focus {
  color: #a3bffa;
}

.focus\:text-indigo-400:focus {
  color: #7f9cf5;
}

.focus\:text-indigo-500:focus {
  color: #667eea;
}

.focus\:text-indigo-600:focus {
  color: #5a67d8;
}

.focus\:text-indigo-700:focus {
  color: #4c51bf;
}

.focus\:text-indigo-800:focus {
  color: #434190;
}

.focus\:text-indigo-900:focus {
  color: #3c366b;
}

.focus\:text-purple-100:focus {
  color: #faf5ff;
}

.focus\:text-purple-200:focus {
  color: #e9d8fd;
}

.focus\:text-purple-300:focus {
  color: #d6bcfa;
}

.focus\:text-purple-400:focus {
  color: #b794f4;
}

.focus\:text-purple-500:focus {
  color: #9f7aea;
}

.focus\:text-purple-600:focus {
  color: #805ad5;
}

.focus\:text-purple-700:focus {
  color: #6b46c1;
}

.focus\:text-purple-800:focus {
  color: #553c9a;
}

.focus\:text-purple-900:focus {
  color: #44337a;
}

.focus\:text-pink-100:focus {
  color: #fff5f7;
}

.focus\:text-pink-200:focus {
  color: #fed7e2;
}

.focus\:text-pink-300:focus {
  color: #fbb6ce;
}

.focus\:text-pink-400:focus {
  color: #f687b3;
}

.focus\:text-pink-500:focus {
  color: #ed64a6;
}

.focus\:text-pink-600:focus {
  color: #d53f8c;
}

.focus\:text-pink-700:focus {
  color: #b83280;
}

.focus\:text-pink-800:focus {
  color: #97266d;
}

.focus\:text-pink-900:focus {
  color: #702459;
}

.text-xs {
  font-size: 0.75rem;
}

.text-sm {
  font-size: 0.875rem;
}

.text-base {
  font-size: 1rem;
}

.text-lg {
  font-size: 1.125rem;
}

.text-xl {
  font-size: 1.25rem;
}

.text-2xl {
  font-size: 1.5rem;
}

.text-3xl {
  font-size: 1.875rem;
}

.text-4xl {
  font-size: 2.25rem;
}

.text-5xl {
  font-size: 3rem;
}

.text-6xl {
  font-size: 4rem;
}

.italic {
  font-style: italic;
}

.not-italic {
  font-style: normal;
}

.uppercase {
  text-transform: uppercase;
}

.lowercase {
  text-transform: lowercase;
}

.capitalize {
  text-transform: capitalize;
}

.normal-case {
  text-transform: none;
}

.underline {
  text-decoration: underline;
}

.line-through {
  text-decoration: line-through;
}

.no-underline {
  text-decoration: none;
}

.hover\:underline:hover {
  text-decoration: underline;
}

.hover\:line-through:hover {
  text-decoration: line-through;
}

.hover\:no-underline:hover {
  text-decoration: none;
}

.focus\:underline:focus {
  text-decoration: underline;
}

.focus\:line-through:focus {
  text-decoration: line-through;
}

.focus\:no-underline:focus {
  text-decoration: none;
}

.antialiased {
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

.subpixel-antialiased {
  -webkit-font-smoothing: auto;
  -moz-osx-font-smoothing: auto;
}

.tracking-tighter {
  letter-spacing: -0.05em;
}

.tracking-tight {
  letter-spacing: -0.025em;
}

.tracking-normal {
  letter-spacing: 0;
}

.tracking-wide {
  letter-spacing: 0.025em;
}

.tracking-wider {
  letter-spacing: 0.05em;
}

.tracking-widest {
  letter-spacing: 0.1em;
}

.select-none {
  -webkit-user-select: none;
     -moz-user-select: none;
      -ms-user-select: none;
          user-select: none;
}

.select-text {
  -webkit-user-select: text;
     -moz-user-select: text;
      -ms-user-select: text;
          user-select: text;
}

.select-all {
  -webkit-user-select: all;
     -moz-user-select: all;
      -ms-user-select: all;
          user-select: all;
}

.select-auto {
  -webkit-user-select: auto;
     -moz-user-select: auto;
      -ms-user-select: auto;
          user-select: auto;
}

.align-baseline {
  vertical-align: baseline;
}

.align-top {
  vertical-align: top;
}

.align-middle {
  vertical-align: middle;
}

.align-bottom {
  vertical-align: bottom;
}

.align-text-top {
  vertical-align: text-top;
}

.align-text-bottom {
  vertical-align: text-bottom;
}

.visible {
  visibility: visible;
}

.invisible {
  visibility: hidden;
}

.whitespace-normal {
  white-space: normal;
}

.whitespace-no-wrap {
  white-space: nowrap;
}

.whitespace-pre {
  white-space: pre;
}

.whitespace-pre-line {
  white-space: pre-line;
}

.whitespace-pre-wrap {
  white-space: pre-wrap;
}

.break-normal {
  overflow-wrap: normal;
  word-break: normal;
}

.break-words {
  overflow-wrap: break-word;
}

.break-all {
  word-break: break-all;
}

.truncate {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.w-0 {
  width: 0;
}

.w-1 {
  width: 0.25rem;
}

.w-2 {
  width: 0.5rem;
}

.w-3 {
  width: 0.75rem;
}

.w-4 {
  width: 1rem;
}

.w-5 {
  width: 1.25rem;
}

.w-6 {
  width: 1.5rem;
}

.w-8 {
  width: 2rem;
}

.w-10 {
  width: 2.5rem;
}

.w-12 {
  width: 3rem;
}

.w-16 {
  width: 4rem;
}

.w-20 {
  width: 5rem;
}

.w-24 {
  width: 6rem;
}

.w-32 {
  width: 8rem;
}

.w-40 {
  width: 10rem;
}

.w-48 {
  width: 12rem;
}

.w-56 {
  width: 14rem;
}

.w-64 {
  width: 16rem;
}

.w-auto {
  width: auto;
}

.w-px {
  width: 1px;
}

.w-1\/2 {
  width: 50%;
}

.w-1\/3 {
  width: 33.333333%;
}

.w-2\/3 {
  width: 66.666667%;
}

.w-1\/4 {
  width: 25%;
}

.w-2\/4 {
  width: 50%;
}

.w-3\/4 {
  width: 75%;
}

.w-1\/5 {
  width: 20%;
}

.w-2\/5 {
  width: 40%;
}

.w-3\/5 {
  width: 60%;
}

.w-4\/5 {
  width: 80%;
}

.w-1\/6 {
  width: 16.666667%;
}

.w-2\/6 {
  width: 33.333333%;
}

.w-3\/6 {
  width: 50%;
}

.w-4\/6 {
  width: 66.666667%;
}

.w-5\/6 {
  width: 83.333333%;
}

.w-1\/12 {
  width: 8.333333%;
}

.w-2\/12 {
  width: 16.666667%;
}

.w-3\/12 {
  width: 25%;
}

.w-4\/12 {
  width: 33.333333%;
}

.w-5\/12 {
  width: 41.666667%;
}

.w-6\/12 {
  width: 50%;
}

.w-7\/12 {
  width: 58.333333%;
}

.w-8\/12 {
  width: 66.666667%;
}

.w-9\/12 {
  width: 75%;
}

.w-10\/12 {
  width: 83.333333%;
}

.w-11\/12 {
  width: 91.666667%;
}

.w-full {
  width: 100%;
}

.w-screen {
  width: 100vw;
}

.z-0 {
  z-index: 0;
}

.z-10 {
  z-index: 10;
}

.z-20 {
  z-index: 20;
}

.z-30 {
  z-index: 30;
}

.z-40 {
  z-index: 40;
}

.z-50 {
  z-index: 50;
}

.z-auto {
  z-index: auto;
}

@media (min-width: 640px) {
  .sm\:sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border-width: 0;
  }

  .sm\:not-sr-only {
    position: static;
    width: auto;
    height: auto;
    padding: 0;
    margin: 0;
    overflow: visible;
    clip: auto;
    white-space: normal;
  }

  .sm\:focus\:sr-only:focus {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border-width: 0;
  }

  .sm\:focus\:not-sr-only:focus {
    position: static;
    width: auto;
    height: auto;
    padding: 0;
    margin: 0;
    overflow: visible;
    clip: auto;
    white-space: normal;
  }

  .sm\:appearance-none {
    -webkit-appearance: none;
       -moz-appearance: none;
            appearance: none;
  }

  .sm\:bg-fixed {
    background-attachment: fixed;
  }

  .sm\:bg-local {
    background-attachment: local;
  }

  .sm\:bg-scroll {
    background-attachment: scroll;
  }

  .sm\:bg-transparent {
    background-color: transparent;
  }

  .sm\:bg-black {
    background-color: #000;
  }

  .sm\:bg-white {
    background-color: #fff;
  }

  .sm\:bg-gray-100 {
    background-color: #f7fafc;
  }

  .sm\:bg-gray-200 {
    background-color: #edf2f7;
  }

  .sm\:bg-gray-300 {
    background-color: #e2e8f0;
  }

  .sm\:bg-gray-400 {
    background-color: #cbd5e0;
  }

  .sm\:bg-gray-500 {
    background-color: #a0aec0;
  }

  .sm\:bg-gray-600 {
    background-color: #718096;
  }

  .sm\:bg-gray-700 {
    background-color: #4a5568;
  }

  .sm\:bg-gray-800 {
    background-color: #2d3748;
  }

  .sm\:bg-gray-900 {
    background-color: #1a202c;
  }

  .sm\:bg-red-100 {
    background-color: #fff5f5;
  }

  .sm\:bg-red-200 {
    background-color: #fed7d7;
  }

  .sm\:bg-red-300 {
    background-color: #feb2b2;
  }

  .sm\:bg-red-400 {
    background-color: #fc8181;
  }

  .sm\:bg-red-500 {
    background-color: #f56565;
  }

  .sm\:bg-red-600 {
    background-color: #e53e3e;
  }

  .sm\:bg-red-700 {
    background-color: #c53030;
  }

  .sm\:bg-red-800 {
    background-color: #9b2c2c;
  }

  .sm\:bg-red-900 {
    background-color: #742a2a;
  }

  .sm\:bg-orange-100 {
    background-color: #fffaf0;
  }

  .sm\:bg-orange-200 {
    background-color: #feebc8;
  }

  .sm\:bg-orange-300 {
    background-color: #fbd38d;
  }

  .sm\:bg-orange-400 {
    background-color: #f6ad55;
  }

  .sm\:bg-orange-500 {
    background-color: #ed8936;
  }

  .sm\:bg-orange-600 {
    background-color: #dd6b20;
  }

  .sm\:bg-orange-700 {
    background-color: #c05621;
  }

  .sm\:bg-orange-800 {
    background-color: #9c4221;
  }

  .sm\:bg-orange-900 {
    background-color: #7b341e;
  }

  .sm\:bg-yellow-100 {
    background-color: #fffff0;
  }

  .sm\:bg-yellow-200 {
    background-color: #fefcbf;
  }

  .sm\:bg-yellow-300 {
    background-color: #faf089;
  }

  .sm\:bg-yellow-400 {
    background-color: #f6e05e;
  }

  .sm\:bg-yellow-500 {
    background-color: #ecc94b;
  }

  .sm\:bg-yellow-600 {
    background-color: #d69e2e;
  }

  .sm\:bg-yellow-700 {
    background-color: #b7791f;
  }

  .sm\:bg-yellow-800 {
    background-color: #975a16;
  }

  .sm\:bg-yellow-900 {
    background-color: #744210;
  }

  .sm\:bg-green-100 {
    background-color: #f0fff4;
  }

  .sm\:bg-green-200 {
    background-color: #c6f6d5;
  }

  .sm\:bg-green-300 {
    background-color: #9ae6b4;
  }

  .sm\:bg-green-400 {
    background-color: #68d391;
  }

  .sm\:bg-green-500 {
    background-color: #48bb78;
  }

  .sm\:bg-green-600 {
    background-color: #38a169;
  }

  .sm\:bg-green-700 {
    background-color: #2f855a;
  }

  .sm\:bg-green-800 {
    background-color: #276749;
  }

  .sm\:bg-green-900 {
    background-color: #22543d;
  }

  .sm\:bg-teal-100 {
    background-color: #e6fffa;
  }

  .sm\:bg-teal-200 {
    background-color: #b2f5ea;
  }

  .sm\:bg-teal-300 {
    background-color: #81e6d9;
  }

  .sm\:bg-teal-400 {
    background-color: #4fd1c5;
  }

  .sm\:bg-teal-500 {
    background-color: #38b2ac;
  }

  .sm\:bg-teal-600 {
    background-color: #319795;
  }

  .sm\:bg-teal-700 {
    background-color: #2c7a7b;
  }

  .sm\:bg-teal-800 {
    background-color: #285e61;
  }

  .sm\:bg-teal-900 {
    background-color: #234e52;
  }

  .sm\:bg-blue-100 {
    background-color: #ebf8ff;
  }

  .sm\:bg-blue-200 {
    background-color: #bee3f8;
  }

  .sm\:bg-blue-300 {
    background-color: #90cdf4;
  }

  .sm\:bg-blue-400 {
    background-color: #63b3ed;
  }

  .sm\:bg-blue-500 {
    background-color: #4299e1;
  }

  .sm\:bg-blue-600 {
    background-color: #3182ce;
  }

  .sm\:bg-blue-700 {
    background-color: #2b6cb0;
  }

  .sm\:bg-blue-800 {
    background-color: #2c5282;
  }

  .sm\:bg-blue-900 {
    background-color: #2a4365;
  }

  .sm\:bg-indigo-100 {
    background-color: #ebf4ff;
  }

  .sm\:bg-indigo-200 {
    background-color: #c3dafe;
  }

  .sm\:bg-indigo-300 {
    background-color: #a3bffa;
  }

  .sm\:bg-indigo-400 {
    background-color: #7f9cf5;
  }

  .sm\:bg-indigo-500 {
    background-color: #667eea;
  }

  .sm\:bg-indigo-600 {
    background-color: #5a67d8;
  }

  .sm\:bg-indigo-700 {
    background-color: #4c51bf;
  }

  .sm\:bg-indigo-800 {
    background-color: #434190;
  }

  .sm\:bg-indigo-900 {
    background-color: #3c366b;
  }

  .sm\:bg-purple-100 {
    background-color: #faf5ff;
  }

  .sm\:bg-purple-200 {
    background-color: #e9d8fd;
  }

  .sm\:bg-purple-300 {
    background-color: #d6bcfa;
  }

  .sm\:bg-purple-400 {
    background-color: #b794f4;
  }

  .sm\:bg-purple-500 {
    background-color: #9f7aea;
  }

  .sm\:bg-purple-600 {
    background-color: #805ad5;
  }

  .sm\:bg-purple-700 {
    background-color: #6b46c1;
  }

  .sm\:bg-purple-800 {
    background-color: #553c9a;
  }

  .sm\:bg-purple-900 {
    background-color: #44337a;
  }

  .sm\:bg-pink-100 {
    background-color: #fff5f7;
  }

  .sm\:bg-pink-200 {
    background-color: #fed7e2;
  }

  .sm\:bg-pink-300 {
    background-color: #fbb6ce;
  }

  .sm\:bg-pink-400 {
    background-color: #f687b3;
  }

  .sm\:bg-pink-500 {
    background-color: #ed64a6;
  }

  .sm\:bg-pink-600 {
    background-color: #d53f8c;
  }

  .sm\:bg-pink-700 {
    background-color: #b83280;
  }

  .sm\:bg-pink-800 {
    background-color: #97266d;
  }

  .sm\:bg-pink-900 {
    background-color: #702459;
  }

  .sm\:hover\:bg-transparent:hover {
    background-color: transparent;
  }

  .sm\:hover\:bg-black:hover {
    background-color: #000;
  }

  .sm\:hover\:bg-white:hover {
    background-color: #fff;
  }

  .sm\:hover\:bg-gray-100:hover {
    background-color: #f7fafc;
  }

  .sm\:hover\:bg-gray-200:hover {
    background-color: #edf2f7;
  }

  .sm\:hover\:bg-gray-300:hover {
    background-color: #e2e8f0;
  }

  .sm\:hover\:bg-gray-400:hover {
    background-color: #cbd5e0;
  }

  .sm\:hover\:bg-gray-500:hover {
    background-color: #a0aec0;
  }

  .sm\:hover\:bg-gray-600:hover {
    background-color: #718096;
  }

  .sm\:hover\:bg-gray-700:hover {
    background-color: #4a5568;
  }

  .sm\:hover\:bg-gray-800:hover {
    background-color: #2d3748;
  }

  .sm\:hover\:bg-gray-900:hover {
    background-color: #1a202c;
  }

  .sm\:hover\:bg-red-100:hover {
    background-color: #fff5f5;
  }

  .sm\:hover\:bg-red-200:hover {
    background-color: #fed7d7;
  }

  .sm\:hover\:bg-red-300:hover {
    background-color: #feb2b2;
  }

  .sm\:hover\:bg-red-400:hover {
    background-color: #fc8181;
  }

  .sm\:hover\:bg-red-500:hover {
    background-color: #f56565;
  }

  .sm\:hover\:bg-red-600:hover {
    background-color: #e53e3e;
  }

  .sm\:hover\:bg-red-700:hover {
    background-color: #c53030;
  }

  .sm\:hover\:bg-red-800:hover {
    background-color: #9b2c2c;
  }

  .sm\:hover\:bg-red-900:hover {
    background-color: #742a2a;
  }

  .sm\:hover\:bg-orange-100:hover {
    background-color: #fffaf0;
  }

  .sm\:hover\:bg-orange-200:hover {
    background-color: #feebc8;
  }

  .sm\:hover\:bg-orange-300:hover {
    background-color: #fbd38d;
  }

  .sm\:hover\:bg-orange-400:hover {
    background-color: #f6ad55;
  }

  .sm\:hover\:bg-orange-500:hover {
    background-color: #ed8936;
  }

  .sm\:hover\:bg-orange-600:hover {
    background-color: #dd6b20;
  }

  .sm\:hover\:bg-orange-700:hover {
    background-color: #c05621;
  }

  .sm\:hover\:bg-orange-800:hover {
    background-color: #9c4221;
  }

  .sm\:hover\:bg-orange-900:hover {
    background-color: #7b341e;
  }

  .sm\:hover\:bg-yellow-100:hover {
    background-color: #fffff0;
  }

  .sm\:hover\:bg-yellow-200:hover {
    background-color: #fefcbf;
  }

  .sm\:hover\:bg-yellow-300:hover {
    background-color: #faf089;
  }

  .sm\:hover\:bg-yellow-400:hover {
    background-color: #f6e05e;
  }

  .sm\:hover\:bg-yellow-500:hover {
    background-color: #ecc94b;
  }

  .sm\:hover\:bg-yellow-600:hover {
    background-color: #d69e2e;
  }

  .sm\:hover\:bg-yellow-700:hover {
    background-color: #b7791f;
  }

  .sm\:hover\:bg-yellow-800:hover {
    background-color: #975a16;
  }

  .sm\:hover\:bg-yellow-900:hover {
    background-color: #744210;
  }

  .sm\:hover\:bg-green-100:hover {
    background-color: #f0fff4;
  }

  .sm\:hover\:bg-green-200:hover {
    background-color: #c6f6d5;
  }

  .sm\:hover\:bg-green-300:hover {
    background-color: #9ae6b4;
  }

  .sm\:hover\:bg-green-400:hover {
    background-color: #68d391;
  }

  .sm\:hover\:bg-green-500:hover {
    background-color: #48bb78;
  }

  .sm\:hover\:bg-green-600:hover {
    background-color: #38a169;
  }

  .sm\:hover\:bg-green-700:hover {
    background-color: #2f855a;
  }

  .sm\:hover\:bg-green-800:hover {
    background-color: #276749;
  }

  .sm\:hover\:bg-green-900:hover {
    background-color: #22543d;
  }

  .sm\:hover\:bg-teal-100:hover {
    background-color: #e6fffa;
  }

  .sm\:hover\:bg-teal-200:hover {
    background-color: #b2f5ea;
  }

  .sm\:hover\:bg-teal-300:hover {
    background-color: #81e6d9;
  }

  .sm\:hover\:bg-teal-400:hover {
    background-color: #4fd1c5;
  }

  .sm\:hover\:bg-teal-500:hover {
    background-color: #38b2ac;
  }

  .sm\:hover\:bg-teal-600:hover {
    background-color: #319795;
  }

  .sm\:hover\:bg-teal-700:hover {
    background-color: #2c7a7b;
  }

  .sm\:hover\:bg-teal-800:hover {
    background-color: #285e61;
  }

  .sm\:hover\:bg-teal-900:hover {
    background-color: #234e52;
  }

  .sm\:hover\:bg-blue-100:hover {
    background-color: #ebf8ff;
  }

  .sm\:hover\:bg-blue-200:hover {
    background-color: #bee3f8;
  }

  .sm\:hover\:bg-blue-300:hover {
    background-color: #90cdf4;
  }

  .sm\:hover\:bg-blue-400:hover {
    background-color: #63b3ed;
  }

  .sm\:hover\:bg-blue-500:hover {
    background-color: #4299e1;
  }

  .sm\:hover\:bg-blue-600:hover {
    background-color: #3182ce;
  }

  .sm\:hover\:bg-blue-700:hover {
    background-color: #2b6cb0;
  }

  .sm\:hover\:bg-blue-800:hover {
    background-color: #2c5282;
  }

  .sm\:hover\:bg-blue-900:hover {
    background-color: #2a4365;
  }

  .sm\:hover\:bg-indigo-100:hover {
    background-color: #ebf4ff;
  }

  .sm\:hover\:bg-indigo-200:hover {
    background-color: #c3dafe;
  }

  .sm\:hover\:bg-indigo-300:hover {
    background-color: #a3bffa;
  }

  .sm\:hover\:bg-indigo-400:hover {
    background-color: #7f9cf5;
  }

  .sm\:hover\:bg-indigo-500:hover {
    background-color: #667eea;
  }

  .sm\:hover\:bg-indigo-600:hover {
    background-color: #5a67d8;
  }

  .sm\:hover\:bg-indigo-700:hover {
    background-color: #4c51bf;
  }

  .sm\:hover\:bg-indigo-800:hover {
    background-color: #434190;
  }

  .sm\:hover\:bg-indigo-900:hover {
    background-color: #3c366b;
  }

  .sm\:hover\:bg-purple-100:hover {
    background-color: #faf5ff;
  }

  .sm\:hover\:bg-purple-200:hover {
    background-color: #e9d8fd;
  }

  .sm\:hover\:bg-purple-300:hover {
    background-color: #d6bcfa;
  }

  .sm\:hover\:bg-purple-400:hover {
    background-color: #b794f4;
  }

  .sm\:hover\:bg-purple-500:hover {
    background-color: #9f7aea;
  }

  .sm\:hover\:bg-purple-600:hover {
    background-color: #805ad5;
  }

  .sm\:hover\:bg-purple-700:hover {
    background-color: #6b46c1;
  }

  .sm\:hover\:bg-purple-800:hover {
    background-color: #553c9a;
  }

  .sm\:hover\:bg-purple-900:hover {
    background-color: #44337a;
  }

  .sm\:hover\:bg-pink-100:hover {
    background-color: #fff5f7;
  }

  .sm\:hover\:bg-pink-200:hover {
    background-color: #fed7e2;
  }

  .sm\:hover\:bg-pink-300:hover {
    background-color: #fbb6ce;
  }

  .sm\:hover\:bg-pink-400:hover {
    background-color: #f687b3;
  }

  .sm\:hover\:bg-pink-500:hover {
    background-color: #ed64a6;
  }

  .sm\:hover\:bg-pink-600:hover {
    background-color: #d53f8c;
  }

  .sm\:hover\:bg-pink-700:hover {
    background-color: #b83280;
  }

  .sm\:hover\:bg-pink-800:hover {
    background-color: #97266d;
  }

  .sm\:hover\:bg-pink-900:hover {
    background-color: #702459;
  }

  .sm\:focus\:bg-transparent:focus {
    background-color: transparent;
  }

  .sm\:focus\:bg-black:focus {
    background-color: #000;
  }

  .sm\:focus\:bg-white:focus {
    background-color: #fff;
  }

  .sm\:focus\:bg-gray-100:focus {
    background-color: #f7fafc;
  }

  .sm\:focus\:bg-gray-200:focus {
    background-color: #edf2f7;
  }

  .sm\:focus\:bg-gray-300:focus {
    background-color: #e2e8f0;
  }

  .sm\:focus\:bg-gray-400:focus {
    background-color: #cbd5e0;
  }

  .sm\:focus\:bg-gray-500:focus {
    background-color: #a0aec0;
  }

  .sm\:focus\:bg-gray-600:focus {
    background-color: #718096;
  }

  .sm\:focus\:bg-gray-700:focus {
    background-color: #4a5568;
  }

  .sm\:focus\:bg-gray-800:focus {
    background-color: #2d3748;
  }

  .sm\:focus\:bg-gray-900:focus {
    background-color: #1a202c;
  }

  .sm\:focus\:bg-red-100:focus {
    background-color: #fff5f5;
  }

  .sm\:focus\:bg-red-200:focus {
    background-color: #fed7d7;
  }

  .sm\:focus\:bg-red-300:focus {
    background-color: #feb2b2;
  }

  .sm\:focus\:bg-red-400:focus {
    background-color: #fc8181;
  }

  .sm\:focus\:bg-red-500:focus {
    background-color: #f56565;
  }

  .sm\:focus\:bg-red-600:focus {
    background-color: #e53e3e;
  }

  .sm\:focus\:bg-red-700:focus {
    background-color: #c53030;
  }

  .sm\:focus\:bg-red-800:focus {
    background-color: #9b2c2c;
  }

  .sm\:focus\:bg-red-900:focus {
    background-color: #742a2a;
  }

  .sm\:focus\:bg-orange-100:focus {
    background-color: #fffaf0;
  }

  .sm\:focus\:bg-orange-200:focus {
    background-color: #feebc8;
  }

  .sm\:focus\:bg-orange-300:focus {
    background-color: #fbd38d;
  }

  .sm\:focus\:bg-orange-400:focus {
    background-color: #f6ad55;
  }

  .sm\:focus\:bg-orange-500:focus {
    background-color: #ed8936;
  }

  .sm\:focus\:bg-orange-600:focus {
    background-color: #dd6b20;
  }

  .sm\:focus\:bg-orange-700:focus {
    background-color: #c05621;
  }

  .sm\:focus\:bg-orange-800:focus {
    background-color: #9c4221;
  }

  .sm\:focus\:bg-orange-900:focus {
    background-color: #7b341e;
  }

  .sm\:focus\:bg-yellow-100:focus {
    background-color: #fffff0;
  }

  .sm\:focus\:bg-yellow-200:focus {
    background-color: #fefcbf;
  }

  .sm\:focus\:bg-yellow-300:focus {
    background-color: #faf089;
  }

  .sm\:focus\:bg-yellow-400:focus {
    background-color: #f6e05e;
  }

  .sm\:focus\:bg-yellow-500:focus {
    background-color: #ecc94b;
  }

  .sm\:focus\:bg-yellow-600:focus {
    background-color: #d69e2e;
  }

  .sm\:focus\:bg-yellow-700:focus {
    background-color: #b7791f;
  }

  .sm\:focus\:bg-yellow-800:focus {
    background-color: #975a16;
  }

  .sm\:focus\:bg-yellow-900:focus {
    background-color: #744210;
  }

  .sm\:focus\:bg-green-100:focus {
    background-color: #f0fff4;
  }

  .sm\:focus\:bg-green-200:focus {
    background-color: #c6f6d5;
  }

  .sm\:focus\:bg-green-300:focus {
    background-color: #9ae6b4;
  }

  .sm\:focus\:bg-green-400:focus {
    background-color: #68d391;
  }

  .sm\:focus\:bg-green-500:focus {
    background-color: #48bb78;
  }

  .sm\:focus\:bg-green-600:focus {
    background-color: #38a169;
  }

  .sm\:focus\:bg-green-700:focus {
    background-color: #2f855a;
  }

  .sm\:focus\:bg-green-800:focus {
    background-color: #276749;
  }

  .sm\:focus\:bg-green-900:focus {
    background-color: #22543d;
  }

  .sm\:focus\:bg-teal-100:focus {
    background-color: #e6fffa;
  }

  .sm\:focus\:bg-teal-200:focus {
    background-color: #b2f5ea;
  }

  .sm\:focus\:bg-teal-300:focus {
    background-color: #81e6d9;
  }

  .sm\:focus\:bg-teal-400:focus {
    background-color: #4fd1c5;
  }

  .sm\:focus\:bg-teal-500:focus {
    background-color: #38b2ac;
  }

  .sm\:focus\:bg-teal-600:focus {
    background-color: #319795;
  }

  .sm\:focus\:bg-teal-700:focus {
    background-color: #2c7a7b;
  }

  .sm\:focus\:bg-teal-800:focus {
    background-color: #285e61;
  }

  .sm\:focus\:bg-teal-900:focus {
    background-color: #234e52;
  }

  .sm\:focus\:bg-blue-100:focus {
    background-color: #ebf8ff;
  }

  .sm\:focus\:bg-blue-200:focus {
    background-color: #bee3f8;
  }

  .sm\:focus\:bg-blue-300:focus {
    background-color: #90cdf4;
  }

  .sm\:focus\:bg-blue-400:focus {
    background-color: #63b3ed;
  }

  .sm\:focus\:bg-blue-500:focus {
    background-color: #4299e1;
  }

  .sm\:focus\:bg-blue-600:focus {
    background-color: #3182ce;
  }

  .sm\:focus\:bg-blue-700:focus {
    background-color: #2b6cb0;
  }

  .sm\:focus\:bg-blue-800:focus {
    background-color: #2c5282;
  }

  .sm\:focus\:bg-blue-900:focus {
    background-color: #2a4365;
  }

  .sm\:focus\:bg-indigo-100:focus {
    background-color: #ebf4ff;
  }

  .sm\:focus\:bg-indigo-200:focus {
    background-color: #c3dafe;
  }

  .sm\:focus\:bg-indigo-300:focus {
    background-color: #a3bffa;
  }

  .sm\:focus\:bg-indigo-400:focus {
    background-color: #7f9cf5;
  }

  .sm\:focus\:bg-indigo-500:focus {
    background-color: #667eea;
  }

  .sm\:focus\:bg-indigo-600:focus {
    background-color: #5a67d8;
  }

  .sm\:focus\:bg-indigo-700:focus {
    background-color: #4c51bf;
  }

  .sm\:focus\:bg-indigo-800:focus {
    background-color: #434190;
  }

  .sm\:focus\:bg-indigo-900:focus {
    background-color: #3c366b;
  }

  .sm\:focus\:bg-purple-100:focus {
    background-color: #faf5ff;
  }

  .sm\:focus\:bg-purple-200:focus {
    background-color: #e9d8fd;
  }

  .sm\:focus\:bg-purple-300:focus {
    background-color: #d6bcfa;
  }

  .sm\:focus\:bg-purple-400:focus {
    background-color: #b794f4;
  }

  .sm\:focus\:bg-purple-500:focus {
    background-color: #9f7aea;
  }

  .sm\:focus\:bg-purple-600:focus {
    background-color: #805ad5;
  }

  .sm\:focus\:bg-purple-700:focus {
    background-color: #6b46c1;
  }

  .sm\:focus\:bg-purple-800:focus {
    background-color: #553c9a;
  }

  .sm\:focus\:bg-purple-900:focus {
    background-color: #44337a;
  }

  .sm\:focus\:bg-pink-100:focus {
    background-color: #fff5f7;
  }

  .sm\:focus\:bg-pink-200:focus {
    background-color: #fed7e2;
  }

  .sm\:focus\:bg-pink-300:focus {
    background-color: #fbb6ce;
  }

  .sm\:focus\:bg-pink-400:focus {
    background-color: #f687b3;
  }

  .sm\:focus\:bg-pink-500:focus {
    background-color: #ed64a6;
  }

  .sm\:focus\:bg-pink-600:focus {
    background-color: #d53f8c;
  }

  .sm\:focus\:bg-pink-700:focus {
    background-color: #b83280;
  }

  .sm\:focus\:bg-pink-800:focus {
    background-color: #97266d;
  }

  .sm\:focus\:bg-pink-900:focus {
    background-color: #702459;
  }

  .sm\:bg-bottom {
    background-position: bottom;
  }

  .sm\:bg-center {
    background-position: center;
  }

  .sm\:bg-left {
    background-position: left;
  }

  .sm\:bg-left-bottom {
    background-position: left bottom;
  }

  .sm\:bg-left-top {
    background-position: left top;
  }

  .sm\:bg-right {
    background-position: right;
  }

  .sm\:bg-right-bottom {
    background-position: right bottom;
  }

  .sm\:bg-right-top {
    background-position: right top;
  }

  .sm\:bg-top {
    background-position: top;
  }

  .sm\:bg-repeat {
    background-repeat: repeat;
  }

  .sm\:bg-no-repeat {
    background-repeat: no-repeat;
  }

  .sm\:bg-repeat-x {
    background-repeat: repeat-x;
  }

  .sm\:bg-repeat-y {
    background-repeat: repeat-y;
  }

  .sm\:bg-repeat-round {
    background-repeat: round;
  }

  .sm\:bg-repeat-space {
    background-repeat: space;
  }

  .sm\:bg-auto {
    background-size: auto;
  }

  .sm\:bg-cover {
    background-size: cover;
  }

  .sm\:bg-contain {
    background-size: contain;
  }

  .sm\:border-collapse {
    border-collapse: collapse;
  }

  .sm\:border-separate {
    border-collapse: separate;
  }

  .sm\:border-transparent {
    border-color: transparent;
  }

  .sm\:border-black {
    border-color: #000;
  }

  .sm\:border-white {
    border-color: #fff;
  }

  .sm\:border-gray-100 {
    border-color: #f7fafc;
  }

  .sm\:border-gray-200 {
    border-color: #edf2f7;
  }

  .sm\:border-gray-300 {
    border-color: #e2e8f0;
  }

  .sm\:border-gray-400 {
    border-color: #cbd5e0;
  }

  .sm\:border-gray-500 {
    border-color: #a0aec0;
  }

  .sm\:border-gray-600 {
    border-color: #718096;
  }

  .sm\:border-gray-700 {
    border-color: #4a5568;
  }

  .sm\:border-gray-800 {
    border-color: #2d3748;
  }

  .sm\:border-gray-900 {
    border-color: #1a202c;
  }

  .sm\:border-red-100 {
    border-color: #fff5f5;
  }

  .sm\:border-red-200 {
    border-color: #fed7d7;
  }

  .sm\:border-red-300 {
    border-color: #feb2b2;
  }

  .sm\:border-red-400 {
    border-color: #fc8181;
  }

  .sm\:border-red-500 {
    border-color: #f56565;
  }

  .sm\:border-red-600 {
    border-color: #e53e3e;
  }

  .sm\:border-red-700 {
    border-color: #c53030;
  }

  .sm\:border-red-800 {
    border-color: #9b2c2c;
  }

  .sm\:border-red-900 {
    border-color: #742a2a;
  }

  .sm\:border-orange-100 {
    border-color: #fffaf0;
  }

  .sm\:border-orange-200 {
    border-color: #feebc8;
  }

  .sm\:border-orange-300 {
    border-color: #fbd38d;
  }

  .sm\:border-orange-400 {
    border-color: #f6ad55;
  }

  .sm\:border-orange-500 {
    border-color: #ed8936;
  }

  .sm\:border-orange-600 {
    border-color: #dd6b20;
  }

  .sm\:border-orange-700 {
    border-color: #c05621;
  }

  .sm\:border-orange-800 {
    border-color: #9c4221;
  }

  .sm\:border-orange-900 {
    border-color: #7b341e;
  }

  .sm\:border-yellow-100 {
    border-color: #fffff0;
  }

  .sm\:border-yellow-200 {
    border-color: #fefcbf;
  }

  .sm\:border-yellow-300 {
    border-color: #faf089;
  }

  .sm\:border-yellow-400 {
    border-color: #f6e05e;
  }

  .sm\:border-yellow-500 {
    border-color: #ecc94b;
  }

  .sm\:border-yellow-600 {
    border-color: #d69e2e;
  }

  .sm\:border-yellow-700 {
    border-color: #b7791f;
  }

  .sm\:border-yellow-800 {
    border-color: #975a16;
  }

  .sm\:border-yellow-900 {
    border-color: #744210;
  }

  .sm\:border-green-100 {
    border-color: #f0fff4;
  }

  .sm\:border-green-200 {
    border-color: #c6f6d5;
  }

  .sm\:border-green-300 {
    border-color: #9ae6b4;
  }

  .sm\:border-green-400 {
    border-color: #68d391;
  }

  .sm\:border-green-500 {
    border-color: #48bb78;
  }

  .sm\:border-green-600 {
    border-color: #38a169;
  }

  .sm\:border-green-700 {
    border-color: #2f855a;
  }

  .sm\:border-green-800 {
    border-color: #276749;
  }

  .sm\:border-green-900 {
    border-color: #22543d;
  }

  .sm\:border-teal-100 {
    border-color: #e6fffa;
  }

  .sm\:border-teal-200 {
    border-color: #b2f5ea;
  }

  .sm\:border-teal-300 {
    border-color: #81e6d9;
  }

  .sm\:border-teal-400 {
    border-color: #4fd1c5;
  }

  .sm\:border-teal-500 {
    border-color: #38b2ac;
  }

  .sm\:border-teal-600 {
    border-color: #319795;
  }

  .sm\:border-teal-700 {
    border-color: #2c7a7b;
  }

  .sm\:border-teal-800 {
    border-color: #285e61;
  }

  .sm\:border-teal-900 {
    border-color: #234e52;
  }

  .sm\:border-blue-100 {
    border-color: #ebf8ff;
  }

  .sm\:border-blue-200 {
    border-color: #bee3f8;
  }

  .sm\:border-blue-300 {
    border-color: #90cdf4;
  }

  .sm\:border-blue-400 {
    border-color: #63b3ed;
  }

  .sm\:border-blue-500 {
    border-color: #4299e1;
  }

  .sm\:border-blue-600 {
    border-color: #3182ce;
  }

  .sm\:border-blue-700 {
    border-color: #2b6cb0;
  }

  .sm\:border-blue-800 {
    border-color: #2c5282;
  }

  .sm\:border-blue-900 {
    border-color: #2a4365;
  }

  .sm\:border-indigo-100 {
    border-color: #ebf4ff;
  }

  .sm\:border-indigo-200 {
    border-color: #c3dafe;
  }

  .sm\:border-indigo-300 {
    border-color: #a3bffa;
  }

  .sm\:border-indigo-400 {
    border-color: #7f9cf5;
  }

  .sm\:border-indigo-500 {
    border-color: #667eea;
  }

  .sm\:border-indigo-600 {
    border-color: #5a67d8;
  }

  .sm\:border-indigo-700 {
    border-color: #4c51bf;
  }

  .sm\:border-indigo-800 {
    border-color: #434190;
  }

  .sm\:border-indigo-900 {
    border-color: #3c366b;
  }

  .sm\:border-purple-100 {
    border-color: #faf5ff;
  }

  .sm\:border-purple-200 {
    border-color: #e9d8fd;
  }

  .sm\:border-purple-300 {
    border-color: #d6bcfa;
  }

  .sm\:border-purple-400 {
    border-color: #b794f4;
  }

  .sm\:border-purple-500 {
    border-color: #9f7aea;
  }

  .sm\:border-purple-600 {
    border-color: #805ad5;
  }

  .sm\:border-purple-700 {
    border-color: #6b46c1;
  }

  .sm\:border-purple-800 {
    border-color: #553c9a;
  }

  .sm\:border-purple-900 {
    border-color: #44337a;
  }

  .sm\:border-pink-100 {
    border-color: #fff5f7;
  }

  .sm\:border-pink-200 {
    border-color: #fed7e2;
  }

  .sm\:border-pink-300 {
    border-color: #fbb6ce;
  }

  .sm\:border-pink-400 {
    border-color: #f687b3;
  }

  .sm\:border-pink-500 {
    border-color: #ed64a6;
  }

  .sm\:border-pink-600 {
    border-color: #d53f8c;
  }

  .sm\:border-pink-700 {
    border-color: #b83280;
  }

  .sm\:border-pink-800 {
    border-color: #97266d;
  }

  .sm\:border-pink-900 {
    border-color: #702459;
  }

  .sm\:hover\:border-transparent:hover {
    border-color: transparent;
  }

  .sm\:hover\:border-black:hover {
    border-color: #000;
  }

  .sm\:hover\:border-white:hover {
    border-color: #fff;
  }

  .sm\:hover\:border-gray-100:hover {
    border-color: #f7fafc;
  }

  .sm\:hover\:border-gray-200:hover {
    border-color: #edf2f7;
  }

  .sm\:hover\:border-gray-300:hover {
    border-color: #e2e8f0;
  }

  .sm\:hover\:border-gray-400:hover {
    border-color: #cbd5e0;
  }

  .sm\:hover\:border-gray-500:hover {
    border-color: #a0aec0;
  }

  .sm\:hover\:border-gray-600:hover {
    border-color: #718096;
  }

  .sm\:hover\:border-gray-700:hover {
    border-color: #4a5568;
  }

  .sm\:hover\:border-gray-800:hover {
    border-color: #2d3748;
  }

  .sm\:hover\:border-gray-900:hover {
    border-color: #1a202c;
  }

  .sm\:hover\:border-red-100:hover {
    border-color: #fff5f5;
  }

  .sm\:hover\:border-red-200:hover {
    border-color: #fed7d7;
  }

  .sm\:hover\:border-red-300:hover {
    border-color: #feb2b2;
  }

  .sm\:hover\:border-red-400:hover {
    border-color: #fc8181;
  }

  .sm\:hover\:border-red-500:hover {
    border-color: #f56565;
  }

  .sm\:hover\:border-red-600:hover {
    border-color: #e53e3e;
  }

  .sm\:hover\:border-red-700:hover {
    border-color: #c53030;
  }

  .sm\:hover\:border-red-800:hover {
    border-color: #9b2c2c;
  }

  .sm\:hover\:border-red-900:hover {
    border-color: #742a2a;
  }

  .sm\:hover\:border-orange-100:hover {
    border-color: #fffaf0;
  }

  .sm\:hover\:border-orange-200:hover {
    border-color: #feebc8;
  }

  .sm\:hover\:border-orange-300:hover {
    border-color: #fbd38d;
  }

  .sm\:hover\:border-orange-400:hover {
    border-color: #f6ad55;
  }

  .sm\:hover\:border-orange-500:hover {
    border-color: #ed8936;
  }

  .sm\:hover\:border-orange-600:hover {
    border-color: #dd6b20;
  }

  .sm\:hover\:border-orange-700:hover {
    border-color: #c05621;
  }

  .sm\:hover\:border-orange-800:hover {
    border-color: #9c4221;
  }

  .sm\:hover\:border-orange-900:hover {
    border-color: #7b341e;
  }

  .sm\:hover\:border-yellow-100:hover {
    border-color: #fffff0;
  }

  .sm\:hover\:border-yellow-200:hover {
    border-color: #fefcbf;
  }

  .sm\:hover\:border-yellow-300:hover {
    border-color: #faf089;
  }

  .sm\:hover\:border-yellow-400:hover {
    border-color: #f6e05e;
  }

  .sm\:hover\:border-yellow-500:hover {
    border-color: #ecc94b;
  }

  .sm\:hover\:border-yellow-600:hover {
    border-color: #d69e2e;
  }

  .sm\:hover\:border-yellow-700:hover {
    border-color: #b7791f;
  }

  .sm\:hover\:border-yellow-800:hover {
    border-color: #975a16;
  }

  .sm\:hover\:border-yellow-900:hover {
    border-color: #744210;
  }

  .sm\:hover\:border-green-100:hover {
    border-color: #f0fff4;
  }

  .sm\:hover\:border-green-200:hover {
    border-color: #c6f6d5;
  }

  .sm\:hover\:border-green-300:hover {
    border-color: #9ae6b4;
  }

  .sm\:hover\:border-green-400:hover {
    border-color: #68d391;
  }

  .sm\:hover\:border-green-500:hover {
    border-color: #48bb78;
  }

  .sm\:hover\:border-green-600:hover {
    border-color: #38a169;
  }

  .sm\:hover\:border-green-700:hover {
    border-color: #2f855a;
  }

  .sm\:hover\:border-green-800:hover {
    border-color: #276749;
  }

  .sm\:hover\:border-green-900:hover {
    border-color: #22543d;
  }

  .sm\:hover\:border-teal-100:hover {
    border-color: #e6fffa;
  }

  .sm\:hover\:border-teal-200:hover {
    border-color: #b2f5ea;
  }

  .sm\:hover\:border-teal-300:hover {
    border-color: #81e6d9;
  }

  .sm\:hover\:border-teal-400:hover {
    border-color: #4fd1c5;
  }

  .sm\:hover\:border-teal-500:hover {
    border-color: #38b2ac;
  }

  .sm\:hover\:border-teal-600:hover {
    border-color: #319795;
  }

  .sm\:hover\:border-teal-700:hover {
    border-color: #2c7a7b;
  }

  .sm\:hover\:border-teal-800:hover {
    border-color: #285e61;
  }

  .sm\:hover\:border-teal-900:hover {
    border-color: #234e52;
  }

  .sm\:hover\:border-blue-100:hover {
    border-color: #ebf8ff;
  }

  .sm\:hover\:border-blue-200:hover {
    border-color: #bee3f8;
  }

  .sm\:hover\:border-blue-300:hover {
    border-color: #90cdf4;
  }

  .sm\:hover\:border-blue-400:hover {
    border-color: #63b3ed;
  }

  .sm\:hover\:border-blue-500:hover {
    border-color: #4299e1;
  }

  .sm\:hover\:border-blue-600:hover {
    border-color: #3182ce;
  }

  .sm\:hover\:border-blue-700:hover {
    border-color: #2b6cb0;
  }

  .sm\:hover\:border-blue-800:hover {
    border-color: #2c5282;
  }

  .sm\:hover\:border-blue-900:hover {
    border-color: #2a4365;
  }

  .sm\:hover\:border-indigo-100:hover {
    border-color: #ebf4ff;
  }

  .sm\:hover\:border-indigo-200:hover {
    border-color: #c3dafe;
  }

  .sm\:hover\:border-indigo-300:hover {
    border-color: #a3bffa;
  }

  .sm\:hover\:border-indigo-400:hover {
    border-color: #7f9cf5;
  }

  .sm\:hover\:border-indigo-500:hover {
    border-color: #667eea;
  }

  .sm\:hover\:border-indigo-600:hover {
    border-color: #5a67d8;
  }

  .sm\:hover\:border-indigo-700:hover {
    border-color: #4c51bf;
  }

  .sm\:hover\:border-indigo-800:hover {
    border-color: #434190;
  }

  .sm\:hover\:border-indigo-900:hover {
    border-color: #3c366b;
  }

  .sm\:hover\:border-purple-100:hover {
    border-color: #faf5ff;
  }

  .sm\:hover\:border-purple-200:hover {
    border-color: #e9d8fd;
  }

  .sm\:hover\:border-purple-300:hover {
    border-color: #d6bcfa;
  }

  .sm\:hover\:border-purple-400:hover {
    border-color: #b794f4;
  }

  .sm\:hover\:border-purple-500:hover {
    border-color: #9f7aea;
  }

  .sm\:hover\:border-purple-600:hover {
    border-color: #805ad5;
  }

  .sm\:hover\:border-purple-700:hover {
    border-color: #6b46c1;
  }

  .sm\:hover\:border-purple-800:hover {
    border-color: #553c9a;
  }

  .sm\:hover\:border-purple-900:hover {
    border-color: #44337a;
  }

  .sm\:hover\:border-pink-100:hover {
    border-color: #fff5f7;
  }

  .sm\:hover\:border-pink-200:hover {
    border-color: #fed7e2;
  }

  .sm\:hover\:border-pink-300:hover {
    border-color: #fbb6ce;
  }

  .sm\:hover\:border-pink-400:hover {
    border-color: #f687b3;
  }

  .sm\:hover\:border-pink-500:hover {
    border-color: #ed64a6;
  }

  .sm\:hover\:border-pink-600:hover {
    border-color: #d53f8c;
  }

  .sm\:hover\:border-pink-700:hover {
    border-color: #b83280;
  }

  .sm\:hover\:border-pink-800:hover {
    border-color: #97266d;
  }

  .sm\:hover\:border-pink-900:hover {
    border-color: #702459;
  }

  .sm\:focus\:border-transparent:focus {
    border-color: transparent;
  }

  .sm\:focus\:border-black:focus {
    border-color: #000;
  }

  .sm\:focus\:border-white:focus {
    border-color: #fff;
  }

  .sm\:focus\:border-gray-100:focus {
    border-color: #f7fafc;
  }

  .sm\:focus\:border-gray-200:focus {
    border-color: #edf2f7;
  }

  .sm\:focus\:border-gray-300:focus {
    border-color: #e2e8f0;
  }

  .sm\:focus\:border-gray-400:focus {
    border-color: #cbd5e0;
  }

  .sm\:focus\:border-gray-500:focus {
    border-color: #a0aec0;
  }

  .sm\:focus\:border-gray-600:focus {
    border-color: #718096;
  }

  .sm\:focus\:border-gray-700:focus {
    border-color: #4a5568;
  }

  .sm\:focus\:border-gray-800:focus {
    border-color: #2d3748;
  }

  .sm\:focus\:border-gray-900:focus {
    border-color: #1a202c;
  }

  .sm\:focus\:border-red-100:focus {
    border-color: #fff5f5;
  }

  .sm\:focus\:border-red-200:focus {
    border-color: #fed7d7;
  }

  .sm\:focus\:border-red-300:focus {
    border-color: #feb2b2;
  }

  .sm\:focus\:border-red-400:focus {
    border-color: #fc8181;
  }

  .sm\:focus\:border-red-500:focus {
    border-color: #f56565;
  }

  .sm\:focus\:border-red-600:focus {
    border-color: #e53e3e;
  }

  .sm\:focus\:border-red-700:focus {
    border-color: #c53030;
  }

  .sm\:focus\:border-red-800:focus {
    border-color: #9b2c2c;
  }

  .sm\:focus\:border-red-900:focus {
    border-color: #742a2a;
  }

  .sm\:focus\:border-orange-100:focus {
    border-color: #fffaf0;
  }

  .sm\:focus\:border-orange-200:focus {
    border-color: #feebc8;
  }

  .sm\:focus\:border-orange-300:focus {
    border-color: #fbd38d;
  }

  .sm\:focus\:border-orange-400:focus {
    border-color: #f6ad55;
  }

  .sm\:focus\:border-orange-500:focus {
    border-color: #ed8936;
  }

  .sm\:focus\:border-orange-600:focus {
    border-color: #dd6b20;
  }

  .sm\:focus\:border-orange-700:focus {
    border-color: #c05621;
  }

  .sm\:focus\:border-orange-800:focus {
    border-color: #9c4221;
  }

  .sm\:focus\:border-orange-900:focus {
    border-color: #7b341e;
  }

  .sm\:focus\:border-yellow-100:focus {
    border-color: #fffff0;
  }

  .sm\:focus\:border-yellow-200:focus {
    border-color: #fefcbf;
  }

  .sm\:focus\:border-yellow-300:focus {
    border-color: #faf089;
  }

  .sm\:focus\:border-yellow-400:focus {
    border-color: #f6e05e;
  }

  .sm\:focus\:border-yellow-500:focus {
    border-color: #ecc94b;
  }

  .sm\:focus\:border-yellow-600:focus {
    border-color: #d69e2e;
  }

  .sm\:focus\:border-yellow-700:focus {
    border-color: #b7791f;
  }

  .sm\:focus\:border-yellow-800:focus {
    border-color: #975a16;
  }

  .sm\:focus\:border-yellow-900:focus {
    border-color: #744210;
  }

  .sm\:focus\:border-green-100:focus {
    border-color: #f0fff4;
  }

  .sm\:focus\:border-green-200:focus {
    border-color: #c6f6d5;
  }

  .sm\:focus\:border-green-300:focus {
    border-color: #9ae6b4;
  }

  .sm\:focus\:border-green-400:focus {
    border-color: #68d391;
  }

  .sm\:focus\:border-green-500:focus {
    border-color: #48bb78;
  }

  .sm\:focus\:border-green-600:focus {
    border-color: #38a169;
  }

  .sm\:focus\:border-green-700:focus {
    border-color: #2f855a;
  }

  .sm\:focus\:border-green-800:focus {
    border-color: #276749;
  }

  .sm\:focus\:border-green-900:focus {
    border-color: #22543d;
  }

  .sm\:focus\:border-teal-100:focus {
    border-color: #e6fffa;
  }

  .sm\:focus\:border-teal-200:focus {
    border-color: #b2f5ea;
  }

  .sm\:focus\:border-teal-300:focus {
    border-color: #81e6d9;
  }

  .sm\:focus\:border-teal-400:focus {
    border-color: #4fd1c5;
  }

  .sm\:focus\:border-teal-500:focus {
    border-color: #38b2ac;
  }

  .sm\:focus\:border-teal-600:focus {
    border-color: #319795;
  }

  .sm\:focus\:border-teal-700:focus {
    border-color: #2c7a7b;
  }

  .sm\:focus\:border-teal-800:focus {
    border-color: #285e61;
  }

  .sm\:focus\:border-teal-900:focus {
    border-color: #234e52;
  }

  .sm\:focus\:border-blue-100:focus {
    border-color: #ebf8ff;
  }

  .sm\:focus\:border-blue-200:focus {
    border-color: #bee3f8;
  }

  .sm\:focus\:border-blue-300:focus {
    border-color: #90cdf4;
  }

  .sm\:focus\:border-blue-400:focus {
    border-color: #63b3ed;
  }

  .sm\:focus\:border-blue-500:focus {
    border-color: #4299e1;
  }

  .sm\:focus\:border-blue-600:focus {
    border-color: #3182ce;
  }

  .sm\:focus\:border-blue-700:focus {
    border-color: #2b6cb0;
  }

  .sm\:focus\:border-blue-800:focus {
    border-color: #2c5282;
  }

  .sm\:focus\:border-blue-900:focus {
    border-color: #2a4365;
  }

  .sm\:focus\:border-indigo-100:focus {
    border-color: #ebf4ff;
  }

  .sm\:focus\:border-indigo-200:focus {
    border-color: #c3dafe;
  }

  .sm\:focus\:border-indigo-300:focus {
    border-color: #a3bffa;
  }

  .sm\:focus\:border-indigo-400:focus {
    border-color: #7f9cf5;
  }

  .sm\:focus\:border-indigo-500:focus {
    border-color: #667eea;
  }

  .sm\:focus\:border-indigo-600:focus {
    border-color: #5a67d8;
  }

  .sm\:focus\:border-indigo-700:focus {
    border-color: #4c51bf;
  }

  .sm\:focus\:border-indigo-800:focus {
    border-color: #434190;
  }

  .sm\:focus\:border-indigo-900:focus {
    border-color: #3c366b;
  }

  .sm\:focus\:border-purple-100:focus {
    border-color: #faf5ff;
  }

  .sm\:focus\:border-purple-200:focus {
    border-color: #e9d8fd;
  }

  .sm\:focus\:border-purple-300:focus {
    border-color: #d6bcfa;
  }

  .sm\:focus\:border-purple-400:focus {
    border-color: #b794f4;
  }

  .sm\:focus\:border-purple-500:focus {
    border-color: #9f7aea;
  }

  .sm\:focus\:border-purple-600:focus {
    border-color: #805ad5;
  }

  .sm\:focus\:border-purple-700:focus {
    border-color: #6b46c1;
  }

  .sm\:focus\:border-purple-800:focus {
    border-color: #553c9a;
  }

  .sm\:focus\:border-purple-900:focus {
    border-color: #44337a;
  }

  .sm\:focus\:border-pink-100:focus {
    border-color: #fff5f7;
  }

  .sm\:focus\:border-pink-200:focus {
    border-color: #fed7e2;
  }

  .sm\:focus\:border-pink-300:focus {
    border-color: #fbb6ce;
  }

  .sm\:focus\:border-pink-400:focus {
    border-color: #f687b3;
  }

  .sm\:focus\:border-pink-500:focus {
    border-color: #ed64a6;
  }

  .sm\:focus\:border-pink-600:focus {
    border-color: #d53f8c;
  }

  .sm\:focus\:border-pink-700:focus {
    border-color: #b83280;
  }

  .sm\:focus\:border-pink-800:focus {
    border-color: #97266d;
  }

  .sm\:focus\:border-pink-900:focus {
    border-color: #702459;
  }

  .sm\:rounded-none {
    border-radius: 0;
  }

  .sm\:rounded-sm {
    border-radius: 0.125rem;
  }

  .sm\:rounded {
    border-radius: 0.25rem;
  }

  .sm\:rounded-lg {
    border-radius: 0.5rem;
  }

  .sm\:rounded-full {
    border-radius: 9999px;
  }

  .sm\:rounded-t-none {
    border-top-left-radius: 0;
    border-top-right-radius: 0;
  }

  .sm\:rounded-r-none {
    border-top-right-radius: 0;
    border-bottom-right-radius: 0;
  }

  .sm\:rounded-b-none {
    border-bottom-right-radius: 0;
    border-bottom-left-radius: 0;
  }

  .sm\:rounded-l-none {
    border-top-left-radius: 0;
    border-bottom-left-radius: 0;
  }

  .sm\:rounded-t-sm {
    border-top-left-radius: 0.125rem;
    border-top-right-radius: 0.125rem;
  }

  .sm\:rounded-r-sm {
    border-top-right-radius: 0.125rem;
    border-bottom-right-radius: 0.125rem;
  }

  .sm\:rounded-b-sm {
    border-bottom-right-radius: 0.125rem;
    border-bottom-left-radius: 0.125rem;
  }

  .sm\:rounded-l-sm {
    border-top-left-radius: 0.125rem;
    border-bottom-left-radius: 0.125rem;
  }

  .sm\:rounded-t {
    border-top-left-radius: 0.25rem;
    border-top-right-radius: 0.25rem;
  }

  .sm\:rounded-r {
    border-top-right-radius: 0.25rem;
    border-bottom-right-radius: 0.25rem;
  }

  .sm\:rounded-b {
    border-bottom-right-radius: 0.25rem;
    border-bottom-left-radius: 0.25rem;
  }

  .sm\:rounded-l {
    border-top-left-radius: 0.25rem;
    border-bottom-left-radius: 0.25rem;
  }

  .sm\:rounded-t-lg {
    border-top-left-radius: 0.5rem;
    border-top-right-radius: 0.5rem;
  }

  .sm\:rounded-r-lg {
    border-top-right-radius: 0.5rem;
    border-bottom-right-radius: 0.5rem;
  }

  .sm\:rounded-b-lg {
    border-bottom-right-radius: 0.5rem;
    border-bottom-left-radius: 0.5rem;
  }

  .sm\:rounded-l-lg {
    border-top-left-radius: 0.5rem;
    border-bottom-left-radius: 0.5rem;
  }

  .sm\:rounded-t-full {
    border-top-left-radius: 9999px;
    border-top-right-radius: 9999px;
  }

  .sm\:rounded-r-full {
    border-top-right-radius: 9999px;
    border-bottom-right-radius: 9999px;
  }

  .sm\:rounded-b-full {
    border-bottom-right-radius: 9999px;
    border-bottom-left-radius: 9999px;
  }

  .sm\:rounded-l-full {
    border-top-left-radius: 9999px;
    border-bottom-left-radius: 9999px;
  }

  .sm\:rounded-tl-none {
    border-top-left-radius: 0;
  }

  .sm\:rounded-tr-none {
    border-top-right-radius: 0;
  }

  .sm\:rounded-br-none {
    border-bottom-right-radius: 0;
  }

  .sm\:rounded-bl-none {
    border-bottom-left-radius: 0;
  }

  .sm\:rounded-tl-sm {
    border-top-left-radius: 0.125rem;
  }

  .sm\:rounded-tr-sm {
    border-top-right-radius: 0.125rem;
  }

  .sm\:rounded-br-sm {
    border-bottom-right-radius: 0.125rem;
  }

  .sm\:rounded-bl-sm {
    border-bottom-left-radius: 0.125rem;
  }

  .sm\:rounded-tl {
    border-top-left-radius: 0.25rem;
  }

  .sm\:rounded-tr {
    border-top-right-radius: 0.25rem;
  }

  .sm\:rounded-br {
    border-bottom-right-radius: 0.25rem;
  }

  .sm\:rounded-bl {
    border-bottom-left-radius: 0.25rem;
  }

  .sm\:rounded-tl-lg {
    border-top-left-radius: 0.5rem;
  }

  .sm\:rounded-tr-lg {
    border-top-right-radius: 0.5rem;
  }

  .sm\:rounded-br-lg {
    border-bottom-right-radius: 0.5rem;
  }

  .sm\:rounded-bl-lg {
    border-bottom-left-radius: 0.5rem;
  }

  .sm\:rounded-tl-full {
    border-top-left-radius: 9999px;
  }

  .sm\:rounded-tr-full {
    border-top-right-radius: 9999px;
  }

  .sm\:rounded-br-full {
    border-bottom-right-radius: 9999px;
  }

  .sm\:rounded-bl-full {
    border-bottom-left-radius: 9999px;
  }

  .sm\:border-solid {
    border-style: solid;
  }

  .sm\:border-dashed {
    border-style: dashed;
  }

  .sm\:border-dotted {
    border-style: dotted;
  }

  .sm\:border-double {
    border-style: double;
  }

  .sm\:border-none {
    border-style: none;
  }

  .sm\:border-0 {
    border-width: 0;
  }

  .sm\:border-2 {
    border-width: 2px;
  }

  .sm\:border-4 {
    border-width: 4px;
  }

  .sm\:border-8 {
    border-width: 8px;
  }

  .sm\:border {
    border-width: 1px;
  }

  .sm\:border-t-0 {
    border-top-width: 0;
  }

  .sm\:border-r-0 {
    border-right-width: 0;
  }

  .sm\:border-b-0 {
    border-bottom-width: 0;
  }

  .sm\:border-l-0 {
    border-left-width: 0;
  }

  .sm\:border-t-2 {
    border-top-width: 2px;
  }

  .sm\:border-r-2 {
    border-right-width: 2px;
  }

  .sm\:border-b-2 {
    border-bottom-width: 2px;
  }

  .sm\:border-l-2 {
    border-left-width: 2px;
  }

  .sm\:border-t-4 {
    border-top-width: 4px;
  }

  .sm\:border-r-4 {
    border-right-width: 4px;
  }

  .sm\:border-b-4 {
    border-bottom-width: 4px;
  }

  .sm\:border-l-4 {
    border-left-width: 4px;
  }

  .sm\:border-t-8 {
    border-top-width: 8px;
  }

  .sm\:border-r-8 {
    border-right-width: 8px;
  }

  .sm\:border-b-8 {
    border-bottom-width: 8px;
  }

  .sm\:border-l-8 {
    border-left-width: 8px;
  }

  .sm\:border-t {
    border-top-width: 1px;
  }

  .sm\:border-r {
    border-right-width: 1px;
  }

  .sm\:border-b {
    border-bottom-width: 1px;
  }

  .sm\:border-l {
    border-left-width: 1px;
  }

  .sm\:cursor-auto {
    cursor: auto;
  }

  .sm\:cursor-default {
    cursor: default;
  }

  .sm\:cursor-pointer {
    cursor: pointer;
  }

  .sm\:cursor-wait {
    cursor: wait;
  }

  .sm\:cursor-text {
    cursor: text;
  }

  .sm\:cursor-move {
    cursor: move;
  }

  .sm\:cursor-not-allowed {
    cursor: not-allowed;
  }

  .sm\:block {
    display: block;
  }

  .sm\:inline-block {
    display: inline-block;
  }

  .sm\:inline {
    display: inline;
  }

  .sm\:flex {
    display: -webkit-box;
    display: flex;
  }

  .sm\:inline-flex {
    display: -webkit-inline-box;
    display: inline-flex;
  }

  .sm\:table {
    display: table;
  }

  .sm\:table-row {
    display: table-row;
  }

  .sm\:table-cell {
    display: table-cell;
  }

  .sm\:hidden {
    display: none;
  }

  .sm\:flex-row {
    -webkit-box-orient: horizontal;
    -webkit-box-direction: normal;
            flex-direction: row;
  }

  .sm\:flex-row-reverse {
    -webkit-box-orient: horizontal;
    -webkit-box-direction: reverse;
            flex-direction: row-reverse;
  }

  .sm\:flex-col {
    -webkit-box-orient: vertical;
    -webkit-box-direction: normal;
            flex-direction: column;
  }

  .sm\:flex-col-reverse {
    -webkit-box-orient: vertical;
    -webkit-box-direction: reverse;
            flex-direction: column-reverse;
  }

  .sm\:flex-wrap {
    flex-wrap: wrap;
  }

  .sm\:flex-wrap-reverse {
    flex-wrap: wrap-reverse;
  }

  .sm\:flex-no-wrap {
    flex-wrap: nowrap;
  }

  .sm\:items-start {
    -webkit-box-align: start;
            align-items: flex-start;
  }

  .sm\:items-end {
    -webkit-box-align: end;
            align-items: flex-end;
  }

  .sm\:items-center {
    -webkit-box-align: center;
            align-items: center;
  }

  .sm\:items-baseline {
    -webkit-box-align: baseline;
            align-items: baseline;
  }

  .sm\:items-stretch {
    -webkit-box-align: stretch;
            align-items: stretch;
  }

  .sm\:self-auto {
    align-self: auto;
  }

  .sm\:self-start {
    align-self: flex-start;
  }

  .sm\:self-end {
    align-self: flex-end;
  }

  .sm\:self-center {
    align-self: center;
  }

  .sm\:self-stretch {
    align-self: stretch;
  }

  .sm\:justify-start {
    -webkit-box-pack: start;
            justify-content: flex-start;
  }

  .sm\:justify-end {
    -webkit-box-pack: end;
            justify-content: flex-end;
  }

  .sm\:justify-center {
    -webkit-box-pack: center;
            justify-content: center;
  }

  .sm\:justify-between {
    -webkit-box-pack: justify;
            justify-content: space-between;
  }

  .sm\:justify-around {
    justify-content: space-around;
  }

  .sm\:content-center {
    align-content: center;
  }

  .sm\:content-start {
    align-content: flex-start;
  }

  .sm\:content-end {
    align-content: flex-end;
  }

  .sm\:content-between {
    align-content: space-between;
  }

  .sm\:content-around {
    align-content: space-around;
  }

  .sm\:flex-1 {
    -webkit-box-flex: 1;
            flex: 1 1 0%;
  }

  .sm\:flex-auto {
    -webkit-box-flex: 1;
            flex: 1 1 auto;
  }

  .sm\:flex-initial {
    -webkit-box-flex: 0;
            flex: 0 1 auto;
  }

  .sm\:flex-none {
    -webkit-box-flex: 0;
            flex: none;
  }

  .sm\:flex-grow-0 {
    -webkit-box-flex: 0;
            flex-grow: 0;
  }

  .sm\:flex-grow {
    -webkit-box-flex: 1;
            flex-grow: 1;
  }

  .sm\:flex-shrink-0 {
    flex-shrink: 0;
  }

  .sm\:flex-shrink {
    flex-shrink: 1;
  }

  .sm\:order-1 {
    -webkit-box-ordinal-group: 2;
            order: 1;
  }

  .sm\:order-2 {
    -webkit-box-ordinal-group: 3;
            order: 2;
  }

  .sm\:order-3 {
    -webkit-box-ordinal-group: 4;
            order: 3;
  }

  .sm\:order-4 {
    -webkit-box-ordinal-group: 5;
            order: 4;
  }

  .sm\:order-5 {
    -webkit-box-ordinal-group: 6;
            order: 5;
  }

  .sm\:order-6 {
    -webkit-box-ordinal-group: 7;
            order: 6;
  }

  .sm\:order-7 {
    -webkit-box-ordinal-group: 8;
            order: 7;
  }

  .sm\:order-8 {
    -webkit-box-ordinal-group: 9;
            order: 8;
  }

  .sm\:order-9 {
    -webkit-box-ordinal-group: 10;
            order: 9;
  }

  .sm\:order-10 {
    -webkit-box-ordinal-group: 11;
            order: 10;
  }

  .sm\:order-11 {
    -webkit-box-ordinal-group: 12;
            order: 11;
  }

  .sm\:order-12 {
    -webkit-box-ordinal-group: 13;
            order: 12;
  }

  .sm\:order-first {
    -webkit-box-ordinal-group: -9998;
            order: -9999;
  }

  .sm\:order-last {
    -webkit-box-ordinal-group: 10000;
            order: 9999;
  }

  .sm\:order-none {
    -webkit-box-ordinal-group: 1;
            order: 0;
  }

  .sm\:float-right {
    float: right;
  }

  .sm\:float-left {
    float: left;
  }

  .sm\:float-none {
    float: none;
  }

  .sm\:clearfix:after {
    content: "";
    display: table;
    clear: both;
  }

  .sm\:font-sans {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji";
  }

  .sm\:font-serif {
    font-family: Georgia, Cambria, "Times New Roman", Times, serif;
  }

  .sm\:font-mono {
    font-family: Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  }

  .sm\:font-hairline {
    font-weight: 100;
  }

  .sm\:font-thin {
    font-weight: 200;
  }

  .sm\:font-light {
    font-weight: 300;
  }

  .sm\:font-normal {
    font-weight: 400;
  }

  .sm\:font-medium {
    font-weight: 500;
  }

  .sm\:font-semibold {
    font-weight: 600;
  }

  .sm\:font-bold {
    font-weight: 700;
  }

  .sm\:font-extrabold {
    font-weight: 800;
  }

  .sm\:font-black {
    font-weight: 900;
  }

  .sm\:hover\:font-hairline:hover {
    font-weight: 100;
  }

  .sm\:hover\:font-thin:hover {
    font-weight: 200;
  }

  .sm\:hover\:font-light:hover {
    font-weight: 300;
  }

  .sm\:hover\:font-normal:hover {
    font-weight: 400;
  }

  .sm\:hover\:font-medium:hover {
    font-weight: 500;
  }

  .sm\:hover\:font-semibold:hover {
    font-weight: 600;
  }

  .sm\:hover\:font-bold:hover {
    font-weight: 700;
  }

  .sm\:hover\:font-extrabold:hover {
    font-weight: 800;
  }

  .sm\:hover\:font-black:hover {
    font-weight: 900;
  }

  .sm\:focus\:font-hairline:focus {
    font-weight: 100;
  }

  .sm\:focus\:font-thin:focus {
    font-weight: 200;
  }

  .sm\:focus\:font-light:focus {
    font-weight: 300;
  }

  .sm\:focus\:font-normal:focus {
    font-weight: 400;
  }

  .sm\:focus\:font-medium:focus {
    font-weight: 500;
  }

  .sm\:focus\:font-semibold:focus {
    font-weight: 600;
  }

  .sm\:focus\:font-bold:focus {
    font-weight: 700;
  }

  .sm\:focus\:font-extrabold:focus {
    font-weight: 800;
  }

  .sm\:focus\:font-black:focus {
    font-weight: 900;
  }

  .sm\:h-0 {
    height: 0;
  }

  .sm\:h-1 {
    height: 0.25rem;
  }

  .sm\:h-2 {
    height: 0.5rem;
  }

  .sm\:h-3 {
    height: 0.75rem;
  }

  .sm\:h-4 {
    height: 1rem;
  }

  .sm\:h-5 {
    height: 1.25rem;
  }

  .sm\:h-6 {
    height: 1.5rem;
  }

  .sm\:h-8 {
    height: 2rem;
  }

  .sm\:h-10 {
    height: 2.5rem;
  }

  .sm\:h-12 {
    height: 3rem;
  }

  .sm\:h-16 {
    height: 4rem;
  }

  .sm\:h-20 {
    height: 5rem;
  }

  .sm\:h-24 {
    height: 6rem;
  }

  .sm\:h-32 {
    height: 8rem;
  }

  .sm\:h-40 {
    height: 10rem;
  }

  .sm\:h-48 {
    height: 12rem;
  }

  .sm\:h-56 {
    height: 14rem;
  }

  .sm\:h-64 {
    height: 16rem;
  }

  .sm\:h-auto {
    height: auto;
  }

  .sm\:h-px {
    height: 1px;
  }

  .sm\:h-full {
    height: 100%;
  }

  .sm\:h-screen {
    height: 100vh;
  }

  .sm\:leading-none {
    line-height: 1;
  }

  .sm\:leading-tight {
    line-height: 1.25;
  }

  .sm\:leading-snug {
    line-height: 1.375;
  }

  .sm\:leading-normal {
    line-height: 1.5;
  }

  .sm\:leading-relaxed {
    line-height: 1.625;
  }

  .sm\:leading-loose {
    line-height: 2;
  }

  .sm\:list-inside {
    list-style-position: inside;
  }

  .sm\:list-outside {
    list-style-position: outside;
  }

  .sm\:list-none {
    list-style-type: none;
  }

  .sm\:list-disc {
    list-style-type: disc;
  }

  .sm\:list-decimal {
    list-style-type: decimal;
  }

  .sm\:m-0 {
    margin: 0;
  }

  .sm\:m-1 {
    margin: 0.25rem;
  }

  .sm\:m-2 {
    margin: 0.5rem;
  }

  .sm\:m-3 {
    margin: 0.75rem;
  }

  .sm\:m-4 {
    margin: 1rem;
  }

  .sm\:m-5 {
    margin: 1.25rem;
  }

  .sm\:m-6 {
    margin: 1.5rem;
  }

  .sm\:m-8 {
    margin: 2rem;
  }

  .sm\:m-10 {
    margin: 2.5rem;
  }

  .sm\:m-12 {
    margin: 3rem;
  }

  .sm\:m-16 {
    margin: 4rem;
  }

  .sm\:m-20 {
    margin: 5rem;
  }

  .sm\:m-24 {
    margin: 6rem;
  }

  .sm\:m-32 {
    margin: 8rem;
  }

  .sm\:m-40 {
    margin: 10rem;
  }

  .sm\:m-48 {
    margin: 12rem;
  }

  .sm\:m-56 {
    margin: 14rem;
  }

  .sm\:m-64 {
    margin: 16rem;
  }

  .sm\:m-auto {
    margin: auto;
  }

  .sm\:m-px {
    margin: 1px;
  }

  .sm\:-m-1 {
    margin: -0.25rem;
  }

  .sm\:-m-2 {
    margin: -0.5rem;
  }

  .sm\:-m-3 {
    margin: -0.75rem;
  }

  .sm\:-m-4 {
    margin: -1rem;
  }

  .sm\:-m-5 {
    margin: -1.25rem;
  }

  .sm\:-m-6 {
    margin: -1.5rem;
  }

  .sm\:-m-8 {
    margin: -2rem;
  }

  .sm\:-m-10 {
    margin: -2.5rem;
  }

  .sm\:-m-12 {
    margin: -3rem;
  }

  .sm\:-m-16 {
    margin: -4rem;
  }

  .sm\:-m-20 {
    margin: -5rem;
  }

  .sm\:-m-24 {
    margin: -6rem;
  }

  .sm\:-m-32 {
    margin: -8rem;
  }

  .sm\:-m-40 {
    margin: -10rem;
  }

  .sm\:-m-48 {
    margin: -12rem;
  }

  .sm\:-m-56 {
    margin: -14rem;
  }

  .sm\:-m-64 {
    margin: -16rem;
  }

  .sm\:-m-px {
    margin: -1px;
  }

  .sm\:my-0 {
    margin-top: 0;
    margin-bottom: 0;
  }

  .sm\:mx-0 {
    margin-left: 0;
    margin-right: 0;
  }

  .sm\:my-1 {
    margin-top: 0.25rem;
    margin-bottom: 0.25rem;
  }

  .sm\:mx-1 {
    margin-left: 0.25rem;
    margin-right: 0.25rem;
  }

  .sm\:my-2 {
    margin-top: 0.5rem;
    margin-bottom: 0.5rem;
  }

  .sm\:mx-2 {
    margin-left: 0.5rem;
    margin-right: 0.5rem;
  }

  .sm\:my-3 {
    margin-top: 0.75rem;
    margin-bottom: 0.75rem;
  }

  .sm\:mx-3 {
    margin-left: 0.75rem;
    margin-right: 0.75rem;
  }

  .sm\:my-4 {
    margin-top: 1rem;
    margin-bottom: 1rem;
  }

  .sm\:mx-4 {
    margin-left: 1rem;
    margin-right: 1rem;
  }

  .sm\:my-5 {
    margin-top: 1.25rem;
    margin-bottom: 1.25rem;
  }

  .sm\:mx-5 {
    margin-left: 1.25rem;
    margin-right: 1.25rem;
  }

  .sm\:my-6 {
    margin-top: 1.5rem;
    margin-bottom: 1.5rem;
  }

  .sm\:mx-6 {
    margin-left: 1.5rem;
    margin-right: 1.5rem;
  }

  .sm\:my-8 {
    margin-top: 2rem;
    margin-bottom: 2rem;
  }

  .sm\:mx-8 {
    margin-left: 2rem;
    margin-right: 2rem;
  }

  .sm\:my-10 {
    margin-top: 2.5rem;
    margin-bottom: 2.5rem;
  }

  .sm\:mx-10 {
    margin-left: 2.5rem;
    margin-right: 2.5rem;
  }

  .sm\:my-12 {
    margin-top: 3rem;
    margin-bottom: 3rem;
  }

  .sm\:mx-12 {
    margin-left: 3rem;
    margin-right: 3rem;
  }

  .sm\:my-16 {
    margin-top: 4rem;
    margin-bottom: 4rem;
  }

  .sm\:mx-16 {
    margin-left: 4rem;
    margin-right: 4rem;
  }

  .sm\:my-20 {
    margin-top: 5rem;
    margin-bottom: 5rem;
  }

  .sm\:mx-20 {
    margin-left: 5rem;
    margin-right: 5rem;
  }

  .sm\:my-24 {
    margin-top: 6rem;
    margin-bottom: 6rem;
  }

  .sm\:mx-24 {
    margin-left: 6rem;
    margin-right: 6rem;
  }

  .sm\:my-32 {
    margin-top: 8rem;
    margin-bottom: 8rem;
  }

  .sm\:mx-32 {
    margin-left: 8rem;
    margin-right: 8rem;
  }

  .sm\:my-40 {
    margin-top: 10rem;
    margin-bottom: 10rem;
  }

  .sm\:mx-40 {
    margin-left: 10rem;
    margin-right: 10rem;
  }

  .sm\:my-48 {
    margin-top: 12rem;
    margin-bottom: 12rem;
  }

  .sm\:mx-48 {
    margin-left: 12rem;
    margin-right: 12rem;
  }

  .sm\:my-56 {
    margin-top: 14rem;
    margin-bottom: 14rem;
  }

  .sm\:mx-56 {
    margin-left: 14rem;
    margin-right: 14rem;
  }

  .sm\:my-64 {
    margin-top: 16rem;
    margin-bottom: 16rem;
  }

  .sm\:mx-64 {
    margin-left: 16rem;
    margin-right: 16rem;
  }

  .sm\:my-auto {
    margin-top: auto;
    margin-bottom: auto;
  }

  .sm\:mx-auto {
    margin-left: auto;
    margin-right: auto;
  }

  .sm\:my-px {
    margin-top: 1px;
    margin-bottom: 1px;
  }

  .sm\:mx-px {
    margin-left: 1px;
    margin-right: 1px;
  }

  .sm\:-my-1 {
    margin-top: -0.25rem;
    margin-bottom: -0.25rem;
  }

  .sm\:-mx-1 {
    margin-left: -0.25rem;
    margin-right: -0.25rem;
  }

  .sm\:-my-2 {
    margin-top: -0.5rem;
    margin-bottom: -0.5rem;
  }

  .sm\:-mx-2 {
    margin-left: -0.5rem;
    margin-right: -0.5rem;
  }

  .sm\:-my-3 {
    margin-top: -0.75rem;
    margin-bottom: -0.75rem;
  }

  .sm\:-mx-3 {
    margin-left: -0.75rem;
    margin-right: -0.75rem;
  }

  .sm\:-my-4 {
    margin-top: -1rem;
    margin-bottom: -1rem;
  }

  .sm\:-mx-4 {
    margin-left: -1rem;
    margin-right: -1rem;
  }

  .sm\:-my-5 {
    margin-top: -1.25rem;
    margin-bottom: -1.25rem;
  }

  .sm\:-mx-5 {
    margin-left: -1.25rem;
    margin-right: -1.25rem;
  }

  .sm\:-my-6 {
    margin-top: -1.5rem;
    margin-bottom: -1.5rem;
  }

  .sm\:-mx-6 {
    margin-left: -1.5rem;
    margin-right: -1.5rem;
  }

  .sm\:-my-8 {
    margin-top: -2rem;
    margin-bottom: -2rem;
  }

  .sm\:-mx-8 {
    margin-left: -2rem;
    margin-right: -2rem;
  }

  .sm\:-my-10 {
    margin-top: -2.5rem;
    margin-bottom: -2.5rem;
  }

  .sm\:-mx-10 {
    margin-left: -2.5rem;
    margin-right: -2.5rem;
  }

  .sm\:-my-12 {
    margin-top: -3rem;
    margin-bottom: -3rem;
  }

  .sm\:-mx-12 {
    margin-left: -3rem;
    margin-right: -3rem;
  }

  .sm\:-my-16 {
    margin-top: -4rem;
    margin-bottom: -4rem;
  }

  .sm\:-mx-16 {
    margin-left: -4rem;
    margin-right: -4rem;
  }

  .sm\:-my-20 {
    margin-top: -5rem;
    margin-bottom: -5rem;
  }

  .sm\:-mx-20 {
    margin-left: -5rem;
    margin-right: -5rem;
  }

  .sm\:-my-24 {
    margin-top: -6rem;
    margin-bottom: -6rem;
  }

  .sm\:-mx-24 {
    margin-left: -6rem;
    margin-right: -6rem;
  }

  .sm\:-my-32 {
    margin-top: -8rem;
    margin-bottom: -8rem;
  }

  .sm\:-mx-32 {
    margin-left: -8rem;
    margin-right: -8rem;
  }

  .sm\:-my-40 {
    margin-top: -10rem;
    margin-bottom: -10rem;
  }

  .sm\:-mx-40 {
    margin-left: -10rem;
    margin-right: -10rem;
  }

  .sm\:-my-48 {
    margin-top: -12rem;
    margin-bottom: -12rem;
  }

  .sm\:-mx-48 {
    margin-left: -12rem;
    margin-right: -12rem;
  }

  .sm\:-my-56 {
    margin-top: -14rem;
    margin-bottom: -14rem;
  }

  .sm\:-mx-56 {
    margin-left: -14rem;
    margin-right: -14rem;
  }

  .sm\:-my-64 {
    margin-top: -16rem;
    margin-bottom: -16rem;
  }

  .sm\:-mx-64 {
    margin-left: -16rem;
    margin-right: -16rem;
  }

  .sm\:-my-px {
    margin-top: -1px;
    margin-bottom: -1px;
  }

  .sm\:-mx-px {
    margin-left: -1px;
    margin-right: -1px;
  }

  .sm\:mt-0 {
    margin-top: 0;
  }

  .sm\:mr-0 {
    margin-right: 0;
  }

  .sm\:mb-0 {
    margin-bottom: 0;
  }

  .sm\:ml-0 {
    margin-left: 0;
  }

  .sm\:mt-1 {
    margin-top: 0.25rem;
  }

  .sm\:mr-1 {
    margin-right: 0.25rem;
  }

  .sm\:mb-1 {
    margin-bottom: 0.25rem;
  }

  .sm\:ml-1 {
    margin-left: 0.25rem;
  }

  .sm\:mt-2 {
    margin-top: 0.5rem;
  }

  .sm\:mr-2 {
    margin-right: 0.5rem;
  }

  .sm\:mb-2 {
    margin-bottom: 0.5rem;
  }

  .sm\:ml-2 {
    margin-left: 0.5rem;
  }

  .sm\:mt-3 {
    margin-top: 0.75rem;
  }

  .sm\:mr-3 {
    margin-right: 0.75rem;
  }

  .sm\:mb-3 {
    margin-bottom: 0.75rem;
  }

  .sm\:ml-3 {
    margin-left: 0.75rem;
  }

  .sm\:mt-4 {
    margin-top: 1rem;
  }

  .sm\:mr-4 {
    margin-right: 1rem;
  }

  .sm\:mb-4 {
    margin-bottom: 1rem;
  }

  .sm\:ml-4 {
    margin-left: 1rem;
  }

  .sm\:mt-5 {
    margin-top: 1.25rem;
  }

  .sm\:mr-5 {
    margin-right: 1.25rem;
  }

  .sm\:mb-5 {
    margin-bottom: 1.25rem;
  }

  .sm\:ml-5 {
    margin-left: 1.25rem;
  }

  .sm\:mt-6 {
    margin-top: 1.5rem;
  }

  .sm\:mr-6 {
    margin-right: 1.5rem;
  }

  .sm\:mb-6 {
    margin-bottom: 1.5rem;
  }

  .sm\:ml-6 {
    margin-left: 1.5rem;
  }

  .sm\:mt-8 {
    margin-top: 2rem;
  }

  .sm\:mr-8 {
    margin-right: 2rem;
  }

  .sm\:mb-8 {
    margin-bottom: 2rem;
  }

  .sm\:ml-8 {
    margin-left: 2rem;
  }

  .sm\:mt-10 {
    margin-top: 2.5rem;
  }

  .sm\:mr-10 {
    margin-right: 2.5rem;
  }

  .sm\:mb-10 {
    margin-bottom: 2.5rem;
  }

  .sm\:ml-10 {
    margin-left: 2.5rem;
  }

  .sm\:mt-12 {
    margin-top: 3rem;
  }

  .sm\:mr-12 {
    margin-right: 3rem;
  }

  .sm\:mb-12 {
    margin-bottom: 3rem;
  }

  .sm\:ml-12 {
    margin-left: 3rem;
  }

  .sm\:mt-16 {
    margin-top: 4rem;
  }

  .sm\:mr-16 {
    margin-right: 4rem;
  }

  .sm\:mb-16 {
    margin-bottom: 4rem;
  }

  .sm\:ml-16 {
    margin-left: 4rem;
  }

  .sm\:mt-20 {
    margin-top: 5rem;
  }

  .sm\:mr-20 {
    margin-right: 5rem;
  }

  .sm\:mb-20 {
    margin-bottom: 5rem;
  }

  .sm\:ml-20 {
    margin-left: 5rem;
  }

  .sm\:mt-24 {
    margin-top: 6rem;
  }

  .sm\:mr-24 {
    margin-right: 6rem;
  }

  .sm\:mb-24 {
    margin-bottom: 6rem;
  }

  .sm\:ml-24 {
    margin-left: 6rem;
  }

  .sm\:mt-32 {
    margin-top: 8rem;
  }

  .sm\:mr-32 {
    margin-right: 8rem;
  }

  .sm\:mb-32 {
    margin-bottom: 8rem;
  }

  .sm\:ml-32 {
    margin-left: 8rem;
  }

  .sm\:mt-40 {
    margin-top: 10rem;
  }

  .sm\:mr-40 {
    margin-right: 10rem;
  }

  .sm\:mb-40 {
    margin-bottom: 10rem;
  }

  .sm\:ml-40 {
    margin-left: 10rem;
  }

  .sm\:mt-48 {
    margin-top: 12rem;
  }

  .sm\:mr-48 {
    margin-right: 12rem;
  }

  .sm\:mb-48 {
    margin-bottom: 12rem;
  }

  .sm\:ml-48 {
    margin-left: 12rem;
  }

  .sm\:mt-56 {
    margin-top: 14rem;
  }

  .sm\:mr-56 {
    margin-right: 14rem;
  }

  .sm\:mb-56 {
    margin-bottom: 14rem;
  }

  .sm\:ml-56 {
    margin-left: 14rem;
  }

  .sm\:mt-64 {
    margin-top: 16rem;
  }

  .sm\:mr-64 {
    margin-right: 16rem;
  }

  .sm\:mb-64 {
    margin-bottom: 16rem;
  }

  .sm\:ml-64 {
    margin-left: 16rem;
  }

  .sm\:mt-auto {
    margin-top: auto;
  }

  .sm\:mr-auto {
    margin-right: auto;
  }

  .sm\:mb-auto {
    margin-bottom: auto;
  }

  .sm\:ml-auto {
    margin-left: auto;
  }

  .sm\:mt-px {
    margin-top: 1px;
  }

  .sm\:mr-px {
    margin-right: 1px;
  }

  .sm\:mb-px {
    margin-bottom: 1px;
  }

  .sm\:ml-px {
    margin-left: 1px;
  }

  .sm\:-mt-1 {
    margin-top: -0.25rem;
  }

  .sm\:-mr-1 {
    margin-right: -0.25rem;
  }

  .sm\:-mb-1 {
    margin-bottom: -0.25rem;
  }

  .sm\:-ml-1 {
    margin-left: -0.25rem;
  }

  .sm\:-mt-2 {
    margin-top: -0.5rem;
  }

  .sm\:-mr-2 {
    margin-right: -0.5rem;
  }

  .sm\:-mb-2 {
    margin-bottom: -0.5rem;
  }

  .sm\:-ml-2 {
    margin-left: -0.5rem;
  }

  .sm\:-mt-3 {
    margin-top: -0.75rem;
  }

  .sm\:-mr-3 {
    margin-right: -0.75rem;
  }

  .sm\:-mb-3 {
    margin-bottom: -0.75rem;
  }

  .sm\:-ml-3 {
    margin-left: -0.75rem;
  }

  .sm\:-mt-4 {
    margin-top: -1rem;
  }

  .sm\:-mr-4 {
    margin-right: -1rem;
  }

  .sm\:-mb-4 {
    margin-bottom: -1rem;
  }

  .sm\:-ml-4 {
    margin-left: -1rem;
  }

  .sm\:-mt-5 {
    margin-top: -1.25rem;
  }

  .sm\:-mr-5 {
    margin-right: -1.25rem;
  }

  .sm\:-mb-5 {
    margin-bottom: -1.25rem;
  }

  .sm\:-ml-5 {
    margin-left: -1.25rem;
  }

  .sm\:-mt-6 {
    margin-top: -1.5rem;
  }

  .sm\:-mr-6 {
    margin-right: -1.5rem;
  }

  .sm\:-mb-6 {
    margin-bottom: -1.5rem;
  }

  .sm\:-ml-6 {
    margin-left: -1.5rem;
  }

  .sm\:-mt-8 {
    margin-top: -2rem;
  }

  .sm\:-mr-8 {
    margin-right: -2rem;
  }

  .sm\:-mb-8 {
    margin-bottom: -2rem;
  }

  .sm\:-ml-8 {
    margin-left: -2rem;
  }

  .sm\:-mt-10 {
    margin-top: -2.5rem;
  }

  .sm\:-mr-10 {
    margin-right: -2.5rem;
  }

  .sm\:-mb-10 {
    margin-bottom: -2.5rem;
  }

  .sm\:-ml-10 {
    margin-left: -2.5rem;
  }

  .sm\:-mt-12 {
    margin-top: -3rem;
  }

  .sm\:-mr-12 {
    margin-right: -3rem;
  }

  .sm\:-mb-12 {
    margin-bottom: -3rem;
  }

  .sm\:-ml-12 {
    margin-left: -3rem;
  }

  .sm\:-mt-16 {
    margin-top: -4rem;
  }

  .sm\:-mr-16 {
    margin-right: -4rem;
  }

  .sm\:-mb-16 {
    margin-bottom: -4rem;
  }

  .sm\:-ml-16 {
    margin-left: -4rem;
  }

  .sm\:-mt-20 {
    margin-top: -5rem;
  }

  .sm\:-mr-20 {
    margin-right: -5rem;
  }

  .sm\:-mb-20 {
    margin-bottom: -5rem;
  }

  .sm\:-ml-20 {
    margin-left: -5rem;
  }

  .sm\:-mt-24 {
    margin-top: -6rem;
  }

  .sm\:-mr-24 {
    margin-right: -6rem;
  }

  .sm\:-mb-24 {
    margin-bottom: -6rem;
  }

  .sm\:-ml-24 {
    margin-left: -6rem;
  }

  .sm\:-mt-32 {
    margin-top: -8rem;
  }

  .sm\:-mr-32 {
    margin-right: -8rem;
  }

  .sm\:-mb-32 {
    margin-bottom: -8rem;
  }

  .sm\:-ml-32 {
    margin-left: -8rem;
  }

  .sm\:-mt-40 {
    margin-top: -10rem;
  }

  .sm\:-mr-40 {
    margin-right: -10rem;
  }

  .sm\:-mb-40 {
    margin-bottom: -10rem;
  }

  .sm\:-ml-40 {
    margin-left: -10rem;
  }

  .sm\:-mt-48 {
    margin-top: -12rem;
  }

  .sm\:-mr-48 {
    margin-right: -12rem;
  }

  .sm\:-mb-48 {
    margin-bottom: -12rem;
  }

  .sm\:-ml-48 {
    margin-left: -12rem;
  }

  .sm\:-mt-56 {
    margin-top: -14rem;
  }

  .sm\:-mr-56 {
    margin-right: -14rem;
  }

  .sm\:-mb-56 {
    margin-bottom: -14rem;
  }

  .sm\:-ml-56 {
    margin-left: -14rem;
  }

  .sm\:-mt-64 {
    margin-top: -16rem;
  }

  .sm\:-mr-64 {
    margin-right: -16rem;
  }

  .sm\:-mb-64 {
    margin-bottom: -16rem;
  }

  .sm\:-ml-64 {
    margin-left: -16rem;
  }

  .sm\:-mt-px {
    margin-top: -1px;
  }

  .sm\:-mr-px {
    margin-right: -1px;
  }

  .sm\:-mb-px {
    margin-bottom: -1px;
  }

  .sm\:-ml-px {
    margin-left: -1px;
  }

  .sm\:max-h-full {
    max-height: 100%;
  }

  .sm\:max-h-screen {
    max-height: 100vh;
  }

  .sm\:max-w-xs {
    max-width: 20rem;
  }

  .sm\:max-w-sm {
    max-width: 24rem;
  }

  .sm\:max-w-md {
    max-width: 28rem;
  }

  .sm\:max-w-lg {
    max-width: 32rem;
  }

  .sm\:max-w-xl {
    max-width: 36rem;
  }

  .sm\:max-w-2xl {
    max-width: 42rem;
  }

  .sm\:max-w-3xl {
    max-width: 48rem;
  }

  .sm\:max-w-4xl {
    max-width: 56rem;
  }

  .sm\:max-w-5xl {
    max-width: 64rem;
  }

  .sm\:max-w-6xl {
    max-width: 72rem;
  }

  .sm\:max-w-full {
    max-width: 100%;
  }

  .sm\:min-h-0 {
    min-height: 0;
  }

  .sm\:min-h-full {
    min-height: 100%;
  }

  .sm\:min-h-screen {
    min-height: 100vh;
  }

  .sm\:min-w-0 {
    min-width: 0;
  }

  .sm\:min-w-full {
    min-width: 100%;
  }

  .sm\:object-contain {
    -o-object-fit: contain;
       object-fit: contain;
  }

  .sm\:object-cover {
    -o-object-fit: cover;
       object-fit: cover;
  }

  .sm\:object-fill {
    -o-object-fit: fill;
       object-fit: fill;
  }

  .sm\:object-none {
    -o-object-fit: none;
       object-fit: none;
  }

  .sm\:object-scale-down {
    -o-object-fit: scale-down;
       object-fit: scale-down;
  }

  .sm\:object-bottom {
    -o-object-position: bottom;
       object-position: bottom;
  }

  .sm\:object-center {
    -o-object-position: center;
       object-position: center;
  }

  .sm\:object-left {
    -o-object-position: left;
       object-position: left;
  }

  .sm\:object-left-bottom {
    -o-object-position: left bottom;
       object-position: left bottom;
  }

  .sm\:object-left-top {
    -o-object-position: left top;
       object-position: left top;
  }

  .sm\:object-right {
    -o-object-position: right;
       object-position: right;
  }

  .sm\:object-right-bottom {
    -o-object-position: right bottom;
       object-position: right bottom;
  }

  .sm\:object-right-top {
    -o-object-position: right top;
       object-position: right top;
  }

  .sm\:object-top {
    -o-object-position: top;
       object-position: top;
  }

  .sm\:opacity-0 {
    opacity: 0;
  }

  .sm\:opacity-25 {
    opacity: 0.25;
  }

  .sm\:opacity-50 {
    opacity: 0.5;
  }

  .sm\:opacity-75 {
    opacity: 0.75;
  }

  .sm\:opacity-100 {
    opacity: 1;
  }

  .sm\:hover\:opacity-0:hover {
    opacity: 0;
  }

  .sm\:hover\:opacity-25:hover {
    opacity: 0.25;
  }

  .sm\:hover\:opacity-50:hover {
    opacity: 0.5;
  }

  .sm\:hover\:opacity-75:hover {
    opacity: 0.75;
  }

  .sm\:hover\:opacity-100:hover {
    opacity: 1;
  }

  .sm\:focus\:opacity-0:focus {
    opacity: 0;
  }

  .sm\:focus\:opacity-25:focus {
    opacity: 0.25;
  }

  .sm\:focus\:opacity-50:focus {
    opacity: 0.5;
  }

  .sm\:focus\:opacity-75:focus {
    opacity: 0.75;
  }

  .sm\:focus\:opacity-100:focus {
    opacity: 1;
  }

  .sm\:outline-none {
    outline: 0;
  }

  .sm\:focus\:outline-none:focus {
    outline: 0;
  }

  .sm\:overflow-auto {
    overflow: auto;
  }

  .sm\:overflow-hidden {
    overflow: hidden;
  }

  .sm\:overflow-visible {
    overflow: visible;
  }

  .sm\:overflow-scroll {
    overflow: scroll;
  }

  .sm\:overflow-x-auto {
    overflow-x: auto;
  }

  .sm\:overflow-y-auto {
    overflow-y: auto;
  }

  .sm\:overflow-x-hidden {
    overflow-x: hidden;
  }

  .sm\:overflow-y-hidden {
    overflow-y: hidden;
  }

  .sm\:overflow-x-visible {
    overflow-x: visible;
  }

  .sm\:overflow-y-visible {
    overflow-y: visible;
  }

  .sm\:overflow-x-scroll {
    overflow-x: scroll;
  }

  .sm\:overflow-y-scroll {
    overflow-y: scroll;
  }

  .sm\:scrolling-touch {
    -webkit-overflow-scrolling: touch;
  }

  .sm\:scrolling-auto {
    -webkit-overflow-scrolling: auto;
  }

  .sm\:p-0 {
    padding: 0;
  }

  .sm\:p-1 {
    padding: 0.25rem;
  }

  .sm\:p-2 {
    padding: 0.5rem;
  }

  .sm\:p-3 {
    padding: 0.75rem;
  }

  .sm\:p-4 {
    padding: 1rem;
  }

  .sm\:p-5 {
    padding: 1.25rem;
  }

  .sm\:p-6 {
    padding: 1.5rem;
  }

  .sm\:p-8 {
    padding: 2rem;
  }

  .sm\:p-10 {
    padding: 2.5rem;
  }

  .sm\:p-12 {
    padding: 3rem;
  }

  .sm\:p-16 {
    padding: 4rem;
  }

  .sm\:p-20 {
    padding: 5rem;
  }

  .sm\:p-24 {
    padding: 6rem;
  }

  .sm\:p-32 {
    padding: 8rem;
  }

  .sm\:p-40 {
    padding: 10rem;
  }

  .sm\:p-48 {
    padding: 12rem;
  }

  .sm\:p-56 {
    padding: 14rem;
  }

  .sm\:p-64 {
    padding: 16rem;
  }

  .sm\:p-px {
    padding: 1px;
  }

  .sm\:py-0 {
    padding-top: 0;
    padding-bottom: 0;
  }

  .sm\:px-0 {
    padding-left: 0;
    padding-right: 0;
  }

  .sm\:py-1 {
    padding-top: 0.25rem;
    padding-bottom: 0.25rem;
  }

  .sm\:px-1 {
    padding-left: 0.25rem;
    padding-right: 0.25rem;
  }

  .sm\:py-2 {
    padding-top: 0.5rem;
    padding-bottom: 0.5rem;
  }

  .sm\:px-2 {
    padding-left: 0.5rem;
    padding-right: 0.5rem;
  }

  .sm\:py-3 {
    padding-top: 0.75rem;
    padding-bottom: 0.75rem;
  }

  .sm\:px-3 {
    padding-left: 0.75rem;
    padding-right: 0.75rem;
  }

  .sm\:py-4 {
    padding-top: 1rem;
    padding-bottom: 1rem;
  }

  .sm\:px-4 {
    padding-left: 1rem;
    padding-right: 1rem;
  }

  .sm\:py-5 {
    padding-top: 1.25rem;
    padding-bottom: 1.25rem;
  }

  .sm\:px-5 {
    padding-left: 1.25rem;
    padding-right: 1.25rem;
  }

  .sm\:py-6 {
    padding-top: 1.5rem;
    padding-bottom: 1.5rem;
  }

  .sm\:px-6 {
    padding-left: 1.5rem;
    padding-right: 1.5rem;
  }

  .sm\:py-8 {
    padding-top: 2rem;
    padding-bottom: 2rem;
  }

  .sm\:px-8 {
    padding-left: 2rem;
    padding-right: 2rem;
  }

  .sm\:py-10 {
    padding-top: 2.5rem;
    padding-bottom: 2.5rem;
  }

  .sm\:px-10 {
    padding-left: 2.5rem;
    padding-right: 2.5rem;
  }

  .sm\:py-12 {
    padding-top: 3rem;
    padding-bottom: 3rem;
  }

  .sm\:px-12 {
    padding-left: 3rem;
    padding-right: 3rem;
  }

  .sm\:py-16 {
    padding-top: 4rem;
    padding-bottom: 4rem;
  }

  .sm\:px-16 {
    padding-left: 4rem;
    padding-right: 4rem;
  }

  .sm\:py-20 {
    padding-top: 5rem;
    padding-bottom: 5rem;
  }

  .sm\:px-20 {
    padding-left: 5rem;
    padding-right: 5rem;
  }

  .sm\:py-24 {
    padding-top: 6rem;
    padding-bottom: 6rem;
  }

  .sm\:px-24 {
    padding-left: 6rem;
    padding-right: 6rem;
  }

  .sm\:py-32 {
    padding-top: 8rem;
    padding-bottom: 8rem;
  }

  .sm\:px-32 {
    padding-left: 8rem;
    padding-right: 8rem;
  }

  .sm\:py-40 {
    padding-top: 10rem;
    padding-bottom: 10rem;
  }

  .sm\:px-40 {
    padding-left: 10rem;
    padding-right: 10rem;
  }

  .sm\:py-48 {
    padding-top: 12rem;
    padding-bottom: 12rem;
  }

  .sm\:px-48 {
    padding-left: 12rem;
    padding-right: 12rem;
  }

  .sm\:py-56 {
    padding-top: 14rem;
    padding-bottom: 14rem;
  }

  .sm\:px-56 {
    padding-left: 14rem;
    padding-right: 14rem;
  }

  .sm\:py-64 {
    padding-top: 16rem;
    padding-bottom: 16rem;
  }

  .sm\:px-64 {
    padding-left: 16rem;
    padding-right: 16rem;
  }

  .sm\:py-px {
    padding-top: 1px;
    padding-bottom: 1px;
  }

  .sm\:px-px {
    padding-left: 1px;
    padding-right: 1px;
  }

  .sm\:pt-0 {
    padding-top: 0;
  }

  .sm\:pr-0 {
    padding-right: 0;
  }

  .sm\:pb-0 {
    padding-bottom: 0;
  }

  .sm\:pl-0 {
    padding-left: 0;
  }

  .sm\:pt-1 {
    padding-top: 0.25rem;
  }

  .sm\:pr-1 {
    padding-right: 0.25rem;
  }

  .sm\:pb-1 {
    padding-bottom: 0.25rem;
  }

  .sm\:pl-1 {
    padding-left: 0.25rem;
  }

  .sm\:pt-2 {
    padding-top: 0.5rem;
  }

  .sm\:pr-2 {
    padding-right: 0.5rem;
  }

  .sm\:pb-2 {
    padding-bottom: 0.5rem;
  }

  .sm\:pl-2 {
    padding-left: 0.5rem;
  }

  .sm\:pt-3 {
    padding-top: 0.75rem;
  }

  .sm\:pr-3 {
    padding-right: 0.75rem;
  }

  .sm\:pb-3 {
    padding-bottom: 0.75rem;
  }

  .sm\:pl-3 {
    padding-left: 0.75rem;
  }

  .sm\:pt-4 {
    padding-top: 1rem;
  }

  .sm\:pr-4 {
    padding-right: 1rem;
  }

  .sm\:pb-4 {
    padding-bottom: 1rem;
  }

  .sm\:pl-4 {
    padding-left: 1rem;
  }

  .sm\:pt-5 {
    padding-top: 1.25rem;
  }

  .sm\:pr-5 {
    padding-right: 1.25rem;
  }

  .sm\:pb-5 {
    padding-bottom: 1.25rem;
  }

  .sm\:pl-5 {
    padding-left: 1.25rem;
  }

  .sm\:pt-6 {
    padding-top: 1.5rem;
  }

  .sm\:pr-6 {
    padding-right: 1.5rem;
  }

  .sm\:pb-6 {
    padding-bottom: 1.5rem;
  }

  .sm\:pl-6 {
    padding-left: 1.5rem;
  }

  .sm\:pt-8 {
    padding-top: 2rem;
  }

  .sm\:pr-8 {
    padding-right: 2rem;
  }

  .sm\:pb-8 {
    padding-bottom: 2rem;
  }

  .sm\:pl-8 {
    padding-left: 2rem;
  }

  .sm\:pt-10 {
    padding-top: 2.5rem;
  }

  .sm\:pr-10 {
    padding-right: 2.5rem;
  }

  .sm\:pb-10 {
    padding-bottom: 2.5rem;
  }

  .sm\:pl-10 {
    padding-left: 2.5rem;
  }

  .sm\:pt-12 {
    padding-top: 3rem;
  }

  .sm\:pr-12 {
    padding-right: 3rem;
  }

  .sm\:pb-12 {
    padding-bottom: 3rem;
  }

  .sm\:pl-12 {
    padding-left: 3rem;
  }

  .sm\:pt-16 {
    padding-top: 4rem;
  }

  .sm\:pr-16 {
    padding-right: 4rem;
  }

  .sm\:pb-16 {
    padding-bottom: 4rem;
  }

  .sm\:pl-16 {
    padding-left: 4rem;
  }

  .sm\:pt-20 {
    padding-top: 5rem;
  }

  .sm\:pr-20 {
    padding-right: 5rem;
  }

  .sm\:pb-20 {
    padding-bottom: 5rem;
  }

  .sm\:pl-20 {
    padding-left: 5rem;
  }

  .sm\:pt-24 {
    padding-top: 6rem;
  }

  .sm\:pr-24 {
    padding-right: 6rem;
  }

  .sm\:pb-24 {
    padding-bottom: 6rem;
  }

  .sm\:pl-24 {
    padding-left: 6rem;
  }

  .sm\:pt-32 {
    padding-top: 8rem;
  }

  .sm\:pr-32 {
    padding-right: 8rem;
  }

  .sm\:pb-32 {
    padding-bottom: 8rem;
  }

  .sm\:pl-32 {
    padding-left: 8rem;
  }

  .sm\:pt-40 {
    padding-top: 10rem;
  }

  .sm\:pr-40 {
    padding-right: 10rem;
  }

  .sm\:pb-40 {
    padding-bottom: 10rem;
  }

  .sm\:pl-40 {
    padding-left: 10rem;
  }

  .sm\:pt-48 {
    padding-top: 12rem;
  }

  .sm\:pr-48 {
    padding-right: 12rem;
  }

  .sm\:pb-48 {
    padding-bottom: 12rem;
  }

  .sm\:pl-48 {
    padding-left: 12rem;
  }

  .sm\:pt-56 {
    padding-top: 14rem;
  }

  .sm\:pr-56 {
    padding-right: 14rem;
  }

  .sm\:pb-56 {
    padding-bottom: 14rem;
  }

  .sm\:pl-56 {
    padding-left: 14rem;
  }

  .sm\:pt-64 {
    padding-top: 16rem;
  }

  .sm\:pr-64 {
    padding-right: 16rem;
  }

  .sm\:pb-64 {
    padding-bottom: 16rem;
  }

  .sm\:pl-64 {
    padding-left: 16rem;
  }

  .sm\:pt-px {
    padding-top: 1px;
  }

  .sm\:pr-px {
    padding-right: 1px;
  }

  .sm\:pb-px {
    padding-bottom: 1px;
  }

  .sm\:pl-px {
    padding-left: 1px;
  }

  .sm\:placeholder-transparent::-webkit-input-placeholder {
    color: transparent;
  }

  .sm\:placeholder-transparent::-moz-placeholder {
    color: transparent;
  }

  .sm\:placeholder-transparent:-ms-input-placeholder {
    color: transparent;
  }

  .sm\:placeholder-transparent::-ms-input-placeholder {
    color: transparent;
  }

  .sm\:placeholder-transparent::placeholder {
    color: transparent;
  }

  .sm\:placeholder-black::-webkit-input-placeholder {
    color: #000;
  }

  .sm\:placeholder-black::-moz-placeholder {
    color: #000;
  }

  .sm\:placeholder-black:-ms-input-placeholder {
    color: #000;
  }

  .sm\:placeholder-black::-ms-input-placeholder {
    color: #000;
  }

  .sm\:placeholder-black::placeholder {
    color: #000;
  }

  .sm\:placeholder-white::-webkit-input-placeholder {
    color: #fff;
  }

  .sm\:placeholder-white::-moz-placeholder {
    color: #fff;
  }

  .sm\:placeholder-white:-ms-input-placeholder {
    color: #fff;
  }

  .sm\:placeholder-white::-ms-input-placeholder {
    color: #fff;
  }

  .sm\:placeholder-white::placeholder {
    color: #fff;
  }

  .sm\:placeholder-gray-100::-webkit-input-placeholder {
    color: #f7fafc;
  }

  .sm\:placeholder-gray-100::-moz-placeholder {
    color: #f7fafc;
  }

  .sm\:placeholder-gray-100:-ms-input-placeholder {
    color: #f7fafc;
  }

  .sm\:placeholder-gray-100::-ms-input-placeholder {
    color: #f7fafc;
  }

  .sm\:placeholder-gray-100::placeholder {
    color: #f7fafc;
  }

  .sm\:placeholder-gray-200::-webkit-input-placeholder {
    color: #edf2f7;
  }

  .sm\:placeholder-gray-200::-moz-placeholder {
    color: #edf2f7;
  }

  .sm\:placeholder-gray-200:-ms-input-placeholder {
    color: #edf2f7;
  }

  .sm\:placeholder-gray-200::-ms-input-placeholder {
    color: #edf2f7;
  }

  .sm\:placeholder-gray-200::placeholder {
    color: #edf2f7;
  }

  .sm\:placeholder-gray-300::-webkit-input-placeholder {
    color: #e2e8f0;
  }

  .sm\:placeholder-gray-300::-moz-placeholder {
    color: #e2e8f0;
  }

  .sm\:placeholder-gray-300:-ms-input-placeholder {
    color: #e2e8f0;
  }

  .sm\:placeholder-gray-300::-ms-input-placeholder {
    color: #e2e8f0;
  }

  .sm\:placeholder-gray-300::placeholder {
    color: #e2e8f0;
  }

  .sm\:placeholder-gray-400::-webkit-input-placeholder {
    color: #cbd5e0;
  }

  .sm\:placeholder-gray-400::-moz-placeholder {
    color: #cbd5e0;
  }

  .sm\:placeholder-gray-400:-ms-input-placeholder {
    color: #cbd5e0;
  }

  .sm\:placeholder-gray-400::-ms-input-placeholder {
    color: #cbd5e0;
  }

  .sm\:placeholder-gray-400::placeholder {
    color: #cbd5e0;
  }

  .sm\:placeholder-gray-500::-webkit-input-placeholder {
    color: #a0aec0;
  }

  .sm\:placeholder-gray-500::-moz-placeholder {
    color: #a0aec0;
  }

  .sm\:placeholder-gray-500:-ms-input-placeholder {
    color: #a0aec0;
  }

  .sm\:placeholder-gray-500::-ms-input-placeholder {
    color: #a0aec0;
  }

  .sm\:placeholder-gray-500::placeholder {
    color: #a0aec0;
  }

  .sm\:placeholder-gray-600::-webkit-input-placeholder {
    color: #718096;
  }

  .sm\:placeholder-gray-600::-moz-placeholder {
    color: #718096;
  }

  .sm\:placeholder-gray-600:-ms-input-placeholder {
    color: #718096;
  }

  .sm\:placeholder-gray-600::-ms-input-placeholder {
    color: #718096;
  }

  .sm\:placeholder-gray-600::placeholder {
    color: #718096;
  }

  .sm\:placeholder-gray-700::-webkit-input-placeholder {
    color: #4a5568;
  }

  .sm\:placeholder-gray-700::-moz-placeholder {
    color: #4a5568;
  }

  .sm\:placeholder-gray-700:-ms-input-placeholder {
    color: #4a5568;
  }

  .sm\:placeholder-gray-700::-ms-input-placeholder {
    color: #4a5568;
  }

  .sm\:placeholder-gray-700::placeholder {
    color: #4a5568;
  }

  .sm\:placeholder-gray-800::-webkit-input-placeholder {
    color: #2d3748;
  }

  .sm\:placeholder-gray-800::-moz-placeholder {
    color: #2d3748;
  }

  .sm\:placeholder-gray-800:-ms-input-placeholder {
    color: #2d3748;
  }

  .sm\:placeholder-gray-800::-ms-input-placeholder {
    color: #2d3748;
  }

  .sm\:placeholder-gray-800::placeholder {
    color: #2d3748;
  }

  .sm\:placeholder-gray-900::-webkit-input-placeholder {
    color: #1a202c;
  }

  .sm\:placeholder-gray-900::-moz-placeholder {
    color: #1a202c;
  }

  .sm\:placeholder-gray-900:-ms-input-placeholder {
    color: #1a202c;
  }

  .sm\:placeholder-gray-900::-ms-input-placeholder {
    color: #1a202c;
  }

  .sm\:placeholder-gray-900::placeholder {
    color: #1a202c;
  }

  .sm\:placeholder-red-100::-webkit-input-placeholder {
    color: #fff5f5;
  }

  .sm\:placeholder-red-100::-moz-placeholder {
    color: #fff5f5;
  }

  .sm\:placeholder-red-100:-ms-input-placeholder {
    color: #fff5f5;
  }

  .sm\:placeholder-red-100::-ms-input-placeholder {
    color: #fff5f5;
  }

  .sm\:placeholder-red-100::placeholder {
    color: #fff5f5;
  }

  .sm\:placeholder-red-200::-webkit-input-placeholder {
    color: #fed7d7;
  }

  .sm\:placeholder-red-200::-moz-placeholder {
    color: #fed7d7;
  }

  .sm\:placeholder-red-200:-ms-input-placeholder {
    color: #fed7d7;
  }

  .sm\:placeholder-red-200::-ms-input-placeholder {
    color: #fed7d7;
  }

  .sm\:placeholder-red-200::placeholder {
    color: #fed7d7;
  }

  .sm\:placeholder-red-300::-webkit-input-placeholder {
    color: #feb2b2;
  }

  .sm\:placeholder-red-300::-moz-placeholder {
    color: #feb2b2;
  }

  .sm\:placeholder-red-300:-ms-input-placeholder {
    color: #feb2b2;
  }

  .sm\:placeholder-red-300::-ms-input-placeholder {
    color: #feb2b2;
  }

  .sm\:placeholder-red-300::placeholder {
    color: #feb2b2;
  }

  .sm\:placeholder-red-400::-webkit-input-placeholder {
    color: #fc8181;
  }

  .sm\:placeholder-red-400::-moz-placeholder {
    color: #fc8181;
  }

  .sm\:placeholder-red-400:-ms-input-placeholder {
    color: #fc8181;
  }

  .sm\:placeholder-red-400::-ms-input-placeholder {
    color: #fc8181;
  }

  .sm\:placeholder-red-400::placeholder {
    color: #fc8181;
  }

  .sm\:placeholder-red-500::-webkit-input-placeholder {
    color: #f56565;
  }

  .sm\:placeholder-red-500::-moz-placeholder {
    color: #f56565;
  }

  .sm\:placeholder-red-500:-ms-input-placeholder {
    color: #f56565;
  }

  .sm\:placeholder-red-500::-ms-input-placeholder {
    color: #f56565;
  }

  .sm\:placeholder-red-500::placeholder {
    color: #f56565;
  }

  .sm\:placeholder-red-600::-webkit-input-placeholder {
    color: #e53e3e;
  }

  .sm\:placeholder-red-600::-moz-placeholder {
    color: #e53e3e;
  }

  .sm\:placeholder-red-600:-ms-input-placeholder {
    color: #e53e3e;
  }

  .sm\:placeholder-red-600::-ms-input-placeholder {
    color: #e53e3e;
  }

  .sm\:placeholder-red-600::placeholder {
    color: #e53e3e;
  }

  .sm\:placeholder-red-700::-webkit-input-placeholder {
    color: #c53030;
  }

  .sm\:placeholder-red-700::-moz-placeholder {
    color: #c53030;
  }

  .sm\:placeholder-red-700:-ms-input-placeholder {
    color: #c53030;
  }

  .sm\:placeholder-red-700::-ms-input-placeholder {
    color: #c53030;
  }

  .sm\:placeholder-red-700::placeholder {
    color: #c53030;
  }

  .sm\:placeholder-red-800::-webkit-input-placeholder {
    color: #9b2c2c;
  }

  .sm\:placeholder-red-800::-moz-placeholder {
    color: #9b2c2c;
  }

  .sm\:placeholder-red-800:-ms-input-placeholder {
    color: #9b2c2c;
  }

  .sm\:placeholder-red-800::-ms-input-placeholder {
    color: #9b2c2c;
  }

  .sm\:placeholder-red-800::placeholder {
    color: #9b2c2c;
  }

  .sm\:placeholder-red-900::-webkit-input-placeholder {
    color: #742a2a;
  }

  .sm\:placeholder-red-900::-moz-placeholder {
    color: #742a2a;
  }

  .sm\:placeholder-red-900:-ms-input-placeholder {
    color: #742a2a;
  }

  .sm\:placeholder-red-900::-ms-input-placeholder {
    color: #742a2a;
  }

  .sm\:placeholder-red-900::placeholder {
    color: #742a2a;
  }

  .sm\:placeholder-orange-100::-webkit-input-placeholder {
    color: #fffaf0;
  }

  .sm\:placeholder-orange-100::-moz-placeholder {
    color: #fffaf0;
  }

  .sm\:placeholder-orange-100:-ms-input-placeholder {
    color: #fffaf0;
  }

  .sm\:placeholder-orange-100::-ms-input-placeholder {
    color: #fffaf0;
  }

  .sm\:placeholder-orange-100::placeholder {
    color: #fffaf0;
  }

  .sm\:placeholder-orange-200::-webkit-input-placeholder {
    color: #feebc8;
  }

  .sm\:placeholder-orange-200::-moz-placeholder {
    color: #feebc8;
  }

  .sm\:placeholder-orange-200:-ms-input-placeholder {
    color: #feebc8;
  }

  .sm\:placeholder-orange-200::-ms-input-placeholder {
    color: #feebc8;
  }

  .sm\:placeholder-orange-200::placeholder {
    color: #feebc8;
  }

  .sm\:placeholder-orange-300::-webkit-input-placeholder {
    color: #fbd38d;
  }

  .sm\:placeholder-orange-300::-moz-placeholder {
    color: #fbd38d;
  }

  .sm\:placeholder-orange-300:-ms-input-placeholder {
    color: #fbd38d;
  }

  .sm\:placeholder-orange-300::-ms-input-placeholder {
    color: #fbd38d;
  }

  .sm\:placeholder-orange-300::placeholder {
    color: #fbd38d;
  }

  .sm\:placeholder-orange-400::-webkit-input-placeholder {
    color: #f6ad55;
  }

  .sm\:placeholder-orange-400::-moz-placeholder {
    color: #f6ad55;
  }

  .sm\:placeholder-orange-400:-ms-input-placeholder {
    color: #f6ad55;
  }

  .sm\:placeholder-orange-400::-ms-input-placeholder {
    color: #f6ad55;
  }

  .sm\:placeholder-orange-400::placeholder {
    color: #f6ad55;
  }

  .sm\:placeholder-orange-500::-webkit-input-placeholder {
    color: #ed8936;
  }

  .sm\:placeholder-orange-500::-moz-placeholder {
    color: #ed8936;
  }

  .sm\:placeholder-orange-500:-ms-input-placeholder {
    color: #ed8936;
  }

  .sm\:placeholder-orange-500::-ms-input-placeholder {
    color: #ed8936;
  }

  .sm\:placeholder-orange-500::placeholder {
    color: #ed8936;
  }

  .sm\:placeholder-orange-600::-webkit-input-placeholder {
    color: #dd6b20;
  }

  .sm\:placeholder-orange-600::-moz-placeholder {
    color: #dd6b20;
  }

  .sm\:placeholder-orange-600:-ms-input-placeholder {
    color: #dd6b20;
  }

  .sm\:placeholder-orange-600::-ms-input-placeholder {
    color: #dd6b20;
  }

  .sm\:placeholder-orange-600::placeholder {
    color: #dd6b20;
  }

  .sm\:placeholder-orange-700::-webkit-input-placeholder {
    color: #c05621;
  }

  .sm\:placeholder-orange-700::-moz-placeholder {
    color: #c05621;
  }

  .sm\:placeholder-orange-700:-ms-input-placeholder {
    color: #c05621;
  }

  .sm\:placeholder-orange-700::-ms-input-placeholder {
    color: #c05621;
  }

  .sm\:placeholder-orange-700::placeholder {
    color: #c05621;
  }

  .sm\:placeholder-orange-800::-webkit-input-placeholder {
    color: #9c4221;
  }

  .sm\:placeholder-orange-800::-moz-placeholder {
    color: #9c4221;
  }

  .sm\:placeholder-orange-800:-ms-input-placeholder {
    color: #9c4221;
  }

  .sm\:placeholder-orange-800::-ms-input-placeholder {
    color: #9c4221;
  }

  .sm\:placeholder-orange-800::placeholder {
    color: #9c4221;
  }

  .sm\:placeholder-orange-900::-webkit-input-placeholder {
    color: #7b341e;
  }

  .sm\:placeholder-orange-900::-moz-placeholder {
    color: #7b341e;
  }

  .sm\:placeholder-orange-900:-ms-input-placeholder {
    color: #7b341e;
  }

  .sm\:placeholder-orange-900::-ms-input-placeholder {
    color: #7b341e;
  }

  .sm\:placeholder-orange-900::placeholder {
    color: #7b341e;
  }

  .sm\:placeholder-yellow-100::-webkit-input-placeholder {
    color: #fffff0;
  }

  .sm\:placeholder-yellow-100::-moz-placeholder {
    color: #fffff0;
  }

  .sm\:placeholder-yellow-100:-ms-input-placeholder {
    color: #fffff0;
  }

  .sm\:placeholder-yellow-100::-ms-input-placeholder {
    color: #fffff0;
  }

  .sm\:placeholder-yellow-100::placeholder {
    color: #fffff0;
  }

  .sm\:placeholder-yellow-200::-webkit-input-placeholder {
    color: #fefcbf;
  }

  .sm\:placeholder-yellow-200::-moz-placeholder {
    color: #fefcbf;
  }

  .sm\:placeholder-yellow-200:-ms-input-placeholder {
    color: #fefcbf;
  }

  .sm\:placeholder-yellow-200::-ms-input-placeholder {
    color: #fefcbf;
  }

  .sm\:placeholder-yellow-200::placeholder {
    color: #fefcbf;
  }

  .sm\:placeholder-yellow-300::-webkit-input-placeholder {
    color: #faf089;
  }

  .sm\:placeholder-yellow-300::-moz-placeholder {
    color: #faf089;
  }

  .sm\:placeholder-yellow-300:-ms-input-placeholder {
    color: #faf089;
  }

  .sm\:placeholder-yellow-300::-ms-input-placeholder {
    color: #faf089;
  }

  .sm\:placeholder-yellow-300::placeholder {
    color: #faf089;
  }

  .sm\:placeholder-yellow-400::-webkit-input-placeholder {
    color: #f6e05e;
  }

  .sm\:placeholder-yellow-400::-moz-placeholder {
    color: #f6e05e;
  }

  .sm\:placeholder-yellow-400:-ms-input-placeholder {
    color: #f6e05e;
  }

  .sm\:placeholder-yellow-400::-ms-input-placeholder {
    color: #f6e05e;
  }

  .sm\:placeholder-yellow-400::placeholder {
    color: #f6e05e;
  }

  .sm\:placeholder-yellow-500::-webkit-input-placeholder {
    color: #ecc94b;
  }

  .sm\:placeholder-yellow-500::-moz-placeholder {
    color: #ecc94b;
  }

  .sm\:placeholder-yellow-500:-ms-input-placeholder {
    color: #ecc94b;
  }

  .sm\:placeholder-yellow-500::-ms-input-placeholder {
    color: #ecc94b;
  }

  .sm\:placeholder-yellow-500::placeholder {
    color: #ecc94b;
  }

  .sm\:placeholder-yellow-600::-webkit-input-placeholder {
    color: #d69e2e;
  }

  .sm\:placeholder-yellow-600::-moz-placeholder {
    color: #d69e2e;
  }

  .sm\:placeholder-yellow-600:-ms-input-placeholder {
    color: #d69e2e;
  }

  .sm\:placeholder-yellow-600::-ms-input-placeholder {
    color: #d69e2e;
  }

  .sm\:placeholder-yellow-600::placeholder {
    color: #d69e2e;
  }

  .sm\:placeholder-yellow-700::-webkit-input-placeholder {
    color: #b7791f;
  }

  .sm\:placeholder-yellow-700::-moz-placeholder {
    color: #b7791f;
  }

  .sm\:placeholder-yellow-700:-ms-input-placeholder {
    color: #b7791f;
  }

  .sm\:placeholder-yellow-700::-ms-input-placeholder {
    color: #b7791f;
  }

  .sm\:placeholder-yellow-700::placeholder {
    color: #b7791f;
  }

  .sm\:placeholder-yellow-800::-webkit-input-placeholder {
    color: #975a16;
  }

  .sm\:placeholder-yellow-800::-moz-placeholder {
    color: #975a16;
  }

  .sm\:placeholder-yellow-800:-ms-input-placeholder {
    color: #975a16;
  }

  .sm\:placeholder-yellow-800::-ms-input-placeholder {
    color: #975a16;
  }

  .sm\:placeholder-yellow-800::placeholder {
    color: #975a16;
  }

  .sm\:placeholder-yellow-900::-webkit-input-placeholder {
    color: #744210;
  }

  .sm\:placeholder-yellow-900::-moz-placeholder {
    color: #744210;
  }

  .sm\:placeholder-yellow-900:-ms-input-placeholder {
    color: #744210;
  }

  .sm\:placeholder-yellow-900::-ms-input-placeholder {
    color: #744210;
  }

  .sm\:placeholder-yellow-900::placeholder {
    color: #744210;
  }

  .sm\:placeholder-green-100::-webkit-input-placeholder {
    color: #f0fff4;
  }

  .sm\:placeholder-green-100::-moz-placeholder {
    color: #f0fff4;
  }

  .sm\:placeholder-green-100:-ms-input-placeholder {
    color: #f0fff4;
  }

  .sm\:placeholder-green-100::-ms-input-placeholder {
    color: #f0fff4;
  }

  .sm\:placeholder-green-100::placeholder {
    color: #f0fff4;
  }

  .sm\:placeholder-green-200::-webkit-input-placeholder {
    color: #c6f6d5;
  }

  .sm\:placeholder-green-200::-moz-placeholder {
    color: #c6f6d5;
  }

  .sm\:placeholder-green-200:-ms-input-placeholder {
    color: #c6f6d5;
  }

  .sm\:placeholder-green-200::-ms-input-placeholder {
    color: #c6f6d5;
  }

  .sm\:placeholder-green-200::placeholder {
    color: #c6f6d5;
  }

  .sm\:placeholder-green-300::-webkit-input-placeholder {
    color: #9ae6b4;
  }

  .sm\:placeholder-green-300::-moz-placeholder {
    color: #9ae6b4;
  }

  .sm\:placeholder-green-300:-ms-input-placeholder {
    color: #9ae6b4;
  }

  .sm\:placeholder-green-300::-ms-input-placeholder {
    color: #9ae6b4;
  }

  .sm\:placeholder-green-300::placeholder {
    color: #9ae6b4;
  }

  .sm\:placeholder-green-400::-webkit-input-placeholder {
    color: #68d391;
  }

  .sm\:placeholder-green-400::-moz-placeholder {
    color: #68d391;
  }

  .sm\:placeholder-green-400:-ms-input-placeholder {
    color: #68d391;
  }

  .sm\:placeholder-green-400::-ms-input-placeholder {
    color: #68d391;
  }

  .sm\:placeholder-green-400::placeholder {
    color: #68d391;
  }

  .sm\:placeholder-green-500::-webkit-input-placeholder {
    color: #48bb78;
  }

  .sm\:placeholder-green-500::-moz-placeholder {
    color: #48bb78;
  }

  .sm\:placeholder-green-500:-ms-input-placeholder {
    color: #48bb78;
  }

  .sm\:placeholder-green-500::-ms-input-placeholder {
    color: #48bb78;
  }

  .sm\:placeholder-green-500::placeholder {
    color: #48bb78;
  }

  .sm\:placeholder-green-600::-webkit-input-placeholder {
    color: #38a169;
  }

  .sm\:placeholder-green-600::-moz-placeholder {
    color: #38a169;
  }

  .sm\:placeholder-green-600:-ms-input-placeholder {
    color: #38a169;
  }

  .sm\:placeholder-green-600::-ms-input-placeholder {
    color: #38a169;
  }

  .sm\:placeholder-green-600::placeholder {
    color: #38a169;
  }

  .sm\:placeholder-green-700::-webkit-input-placeholder {
    color: #2f855a;
  }

  .sm\:placeholder-green-700::-moz-placeholder {
    color: #2f855a;
  }

  .sm\:placeholder-green-700:-ms-input-placeholder {
    color: #2f855a;
  }

  .sm\:placeholder-green-700::-ms-input-placeholder {
    color: #2f855a;
  }

  .sm\:placeholder-green-700::placeholder {
    color: #2f855a;
  }

  .sm\:placeholder-green-800::-webkit-input-placeholder {
    color: #276749;
  }

  .sm\:placeholder-green-800::-moz-placeholder {
    color: #276749;
  }

  .sm\:placeholder-green-800:-ms-input-placeholder {
    color: #276749;
  }

  .sm\:placeholder-green-800::-ms-input-placeholder {
    color: #276749;
  }

  .sm\:placeholder-green-800::placeholder {
    color: #276749;
  }

  .sm\:placeholder-green-900::-webkit-input-placeholder {
    color: #22543d;
  }

  .sm\:placeholder-green-900::-moz-placeholder {
    color: #22543d;
  }

  .sm\:placeholder-green-900:-ms-input-placeholder {
    color: #22543d;
  }

  .sm\:placeholder-green-900::-ms-input-placeholder {
    color: #22543d;
  }

  .sm\:placeholder-green-900::placeholder {
    color: #22543d;
  }

  .sm\:placeholder-teal-100::-webkit-input-placeholder {
    color: #e6fffa;
  }

  .sm\:placeholder-teal-100::-moz-placeholder {
    color: #e6fffa;
  }

  .sm\:placeholder-teal-100:-ms-input-placeholder {
    color: #e6fffa;
  }

  .sm\:placeholder-teal-100::-ms-input-placeholder {
    color: #e6fffa;
  }

  .sm\:placeholder-teal-100::placeholder {
    color: #e6fffa;
  }

  .sm\:placeholder-teal-200::-webkit-input-placeholder {
    color: #b2f5ea;
  }

  .sm\:placeholder-teal-200::-moz-placeholder {
    color: #b2f5ea;
  }

  .sm\:placeholder-teal-200:-ms-input-placeholder {
    color: #b2f5ea;
  }

  .sm\:placeholder-teal-200::-ms-input-placeholder {
    color: #b2f5ea;
  }

  .sm\:placeholder-teal-200::placeholder {
    color: #b2f5ea;
  }

  .sm\:placeholder-teal-300::-webkit-input-placeholder {
    color: #81e6d9;
  }

  .sm\:placeholder-teal-300::-moz-placeholder {
    color: #81e6d9;
  }

  .sm\:placeholder-teal-300:-ms-input-placeholder {
    color: #81e6d9;
  }

  .sm\:placeholder-teal-300::-ms-input-placeholder {
    color: #81e6d9;
  }

  .sm\:placeholder-teal-300::placeholder {
    color: #81e6d9;
  }

  .sm\:placeholder-teal-400::-webkit-input-placeholder {
    color: #4fd1c5;
  }

  .sm\:placeholder-teal-400::-moz-placeholder {
    color: #4fd1c5;
  }

  .sm\:placeholder-teal-400:-ms-input-placeholder {
    color: #4fd1c5;
  }

  .sm\:placeholder-teal-400::-ms-input-placeholder {
    color: #4fd1c5;
  }

  .sm\:placeholder-teal-400::placeholder {
    color: #4fd1c5;
  }

  .sm\:placeholder-teal-500::-webkit-input-placeholder {
    color: #38b2ac;
  }

  .sm\:placeholder-teal-500::-moz-placeholder {
    color: #38b2ac;
  }

  .sm\:placeholder-teal-500:-ms-input-placeholder {
    color: #38b2ac;
  }

  .sm\:placeholder-teal-500::-ms-input-placeholder {
    color: #38b2ac;
  }

  .sm\:placeholder-teal-500::placeholder {
    color: #38b2ac;
  }

  .sm\:placeholder-teal-600::-webkit-input-placeholder {
    color: #319795;
  }

  .sm\:placeholder-teal-600::-moz-placeholder {
    color: #319795;
  }

  .sm\:placeholder-teal-600:-ms-input-placeholder {
    color: #319795;
  }

  .sm\:placeholder-teal-600::-ms-input-placeholder {
    color: #319795;
  }

  .sm\:placeholder-teal-600::placeholder {
    color: #319795;
  }

  .sm\:placeholder-teal-700::-webkit-input-placeholder {
    color: #2c7a7b;
  }

  .sm\:placeholder-teal-700::-moz-placeholder {
    color: #2c7a7b;
  }

  .sm\:placeholder-teal-700:-ms-input-placeholder {
    color: #2c7a7b;
  }

  .sm\:placeholder-teal-700::-ms-input-placeholder {
    color: #2c7a7b;
  }

  .sm\:placeholder-teal-700::placeholder {
    color: #2c7a7b;
  }

  .sm\:placeholder-teal-800::-webkit-input-placeholder {
    color: #285e61;
  }

  .sm\:placeholder-teal-800::-moz-placeholder {
    color: #285e61;
  }

  .sm\:placeholder-teal-800:-ms-input-placeholder {
    color: #285e61;
  }

  .sm\:placeholder-teal-800::-ms-input-placeholder {
    color: #285e61;
  }

  .sm\:placeholder-teal-800::placeholder {
    color: #285e61;
  }

  .sm\:placeholder-teal-900::-webkit-input-placeholder {
    color: #234e52;
  }

  .sm\:placeholder-teal-900::-moz-placeholder {
    color: #234e52;
  }

  .sm\:placeholder-teal-900:-ms-input-placeholder {
    color: #234e52;
  }

  .sm\:placeholder-teal-900::-ms-input-placeholder {
    color: #234e52;
  }

  .sm\:placeholder-teal-900::placeholder {
    color: #234e52;
  }

  .sm\:placeholder-blue-100::-webkit-input-placeholder {
    color: #ebf8ff;
  }

  .sm\:placeholder-blue-100::-moz-placeholder {
    color: #ebf8ff;
  }

  .sm\:placeholder-blue-100:-ms-input-placeholder {
    color: #ebf8ff;
  }

  .sm\:placeholder-blue-100::-ms-input-placeholder {
    color: #ebf8ff;
  }

  .sm\:placeholder-blue-100::placeholder {
    color: #ebf8ff;
  }

  .sm\:placeholder-blue-200::-webkit-input-placeholder {
    color: #bee3f8;
  }

  .sm\:placeholder-blue-200::-moz-placeholder {
    color: #bee3f8;
  }

  .sm\:placeholder-blue-200:-ms-input-placeholder {
    color: #bee3f8;
  }

  .sm\:placeholder-blue-200::-ms-input-placeholder {
    color: #bee3f8;
  }

  .sm\:placeholder-blue-200::placeholder {
    color: #bee3f8;
  }

  .sm\:placeholder-blue-300::-webkit-input-placeholder {
    color: #90cdf4;
  }

  .sm\:placeholder-blue-300::-moz-placeholder {
    color: #90cdf4;
  }

  .sm\:placeholder-blue-300:-ms-input-placeholder {
    color: #90cdf4;
  }

  .sm\:placeholder-blue-300::-ms-input-placeholder {
    color: #90cdf4;
  }

  .sm\:placeholder-blue-300::placeholder {
    color: #90cdf4;
  }

  .sm\:placeholder-blue-400::-webkit-input-placeholder {
    color: #63b3ed;
  }

  .sm\:placeholder-blue-400::-moz-placeholder {
    color: #63b3ed;
  }

  .sm\:placeholder-blue-400:-ms-input-placeholder {
    color: #63b3ed;
  }

  .sm\:placeholder-blue-400::-ms-input-placeholder {
    color: #63b3ed;
  }

  .sm\:placeholder-blue-400::placeholder {
    color: #63b3ed;
  }

  .sm\:placeholder-blue-500::-webkit-input-placeholder {
    color: #4299e1;
  }

  .sm\:placeholder-blue-500::-moz-placeholder {
    color: #4299e1;
  }

  .sm\:placeholder-blue-500:-ms-input-placeholder {
    color: #4299e1;
  }

  .sm\:placeholder-blue-500::-ms-input-placeholder {
    color: #4299e1;
  }

  .sm\:placeholder-blue-500::placeholder {
    color: #4299e1;
  }

  .sm\:placeholder-blue-600::-webkit-input-placeholder {
    color: #3182ce;
  }

  .sm\:placeholder-blue-600::-moz-placeholder {
    color: #3182ce;
  }

  .sm\:placeholder-blue-600:-ms-input-placeholder {
    color: #3182ce;
  }

  .sm\:placeholder-blue-600::-ms-input-placeholder {
    color: #3182ce;
  }

  .sm\:placeholder-blue-600::placeholder {
    color: #3182ce;
  }

  .sm\:placeholder-blue-700::-webkit-input-placeholder {
    color: #2b6cb0;
  }

  .sm\:placeholder-blue-700::-moz-placeholder {
    color: #2b6cb0;
  }

  .sm\:placeholder-blue-700:-ms-input-placeholder {
    color: #2b6cb0;
  }

  .sm\:placeholder-blue-700::-ms-input-placeholder {
    color: #2b6cb0;
  }

  .sm\:placeholder-blue-700::placeholder {
    color: #2b6cb0;
  }

  .sm\:placeholder-blue-800::-webkit-input-placeholder {
    color: #2c5282;
  }

  .sm\:placeholder-blue-800::-moz-placeholder {
    color: #2c5282;
  }

  .sm\:placeholder-blue-800:-ms-input-placeholder {
    color: #2c5282;
  }

  .sm\:placeholder-blue-800::-ms-input-placeholder {
    color: #2c5282;
  }

  .sm\:placeholder-blue-800::placeholder {
    color: #2c5282;
  }

  .sm\:placeholder-blue-900::-webkit-input-placeholder {
    color: #2a4365;
  }

  .sm\:placeholder-blue-900::-moz-placeholder {
    color: #2a4365;
  }

  .sm\:placeholder-blue-900:-ms-input-placeholder {
    color: #2a4365;
  }

  .sm\:placeholder-blue-900::-ms-input-placeholder {
    color: #2a4365;
  }

  .sm\:placeholder-blue-900::placeholder {
    color: #2a4365;
  }

  .sm\:placeholder-indigo-100::-webkit-input-placeholder {
    color: #ebf4ff;
  }

  .sm\:placeholder-indigo-100::-moz-placeholder {
    color: #ebf4ff;
  }

  .sm\:placeholder-indigo-100:-ms-input-placeholder {
    color: #ebf4ff;
  }

  .sm\:placeholder-indigo-100::-ms-input-placeholder {
    color: #ebf4ff;
  }

  .sm\:placeholder-indigo-100::placeholder {
    color: #ebf4ff;
  }

  .sm\:placeholder-indigo-200::-webkit-input-placeholder {
    color: #c3dafe;
  }

  .sm\:placeholder-indigo-200::-moz-placeholder {
    color: #c3dafe;
  }

  .sm\:placeholder-indigo-200:-ms-input-placeholder {
    color: #c3dafe;
  }

  .sm\:placeholder-indigo-200::-ms-input-placeholder {
    color: #c3dafe;
  }

  .sm\:placeholder-indigo-200::placeholder {
    color: #c3dafe;
  }

  .sm\:placeholder-indigo-300::-webkit-input-placeholder {
    color: #a3bffa;
  }

  .sm\:placeholder-indigo-300::-moz-placeholder {
    color: #a3bffa;
  }

  .sm\:placeholder-indigo-300:-ms-input-placeholder {
    color: #a3bffa;
  }

  .sm\:placeholder-indigo-300::-ms-input-placeholder {
    color: #a3bffa;
  }

  .sm\:placeholder-indigo-300::placeholder {
    color: #a3bffa;
  }

  .sm\:placeholder-indigo-400::-webkit-input-placeholder {
    color: #7f9cf5;
  }

  .sm\:placeholder-indigo-400::-moz-placeholder {
    color: #7f9cf5;
  }

  .sm\:placeholder-indigo-400:-ms-input-placeholder {
    color: #7f9cf5;
  }

  .sm\:placeholder-indigo-400::-ms-input-placeholder {
    color: #7f9cf5;
  }

  .sm\:placeholder-indigo-400::placeholder {
    color: #7f9cf5;
  }

  .sm\:placeholder-indigo-500::-webkit-input-placeholder {
    color: #667eea;
  }

  .sm\:placeholder-indigo-500::-moz-placeholder {
    color: #667eea;
  }

  .sm\:placeholder-indigo-500:-ms-input-placeholder {
    color: #667eea;
  }

  .sm\:placeholder-indigo-500::-ms-input-placeholder {
    color: #667eea;
  }

  .sm\:placeholder-indigo-500::placeholder {
    color: #667eea;
  }

  .sm\:placeholder-indigo-600::-webkit-input-placeholder {
    color: #5a67d8;
  }

  .sm\:placeholder-indigo-600::-moz-placeholder {
    color: #5a67d8;
  }

  .sm\:placeholder-indigo-600:-ms-input-placeholder {
    color: #5a67d8;
  }

  .sm\:placeholder-indigo-600::-ms-input-placeholder {
    color: #5a67d8;
  }

  .sm\:placeholder-indigo-600::placeholder {
    color: #5a67d8;
  }

  .sm\:placeholder-indigo-700::-webkit-input-placeholder {
    color: #4c51bf;
  }

  .sm\:placeholder-indigo-700::-moz-placeholder {
    color: #4c51bf;
  }

  .sm\:placeholder-indigo-700:-ms-input-placeholder {
    color: #4c51bf;
  }

  .sm\:placeholder-indigo-700::-ms-input-placeholder {
    color: #4c51bf;
  }

  .sm\:placeholder-indigo-700::placeholder {
    color: #4c51bf;
  }

  .sm\:placeholder-indigo-800::-webkit-input-placeholder {
    color: #434190;
  }

  .sm\:placeholder-indigo-800::-moz-placeholder {
    color: #434190;
  }

  .sm\:placeholder-indigo-800:-ms-input-placeholder {
    color: #434190;
  }

  .sm\:placeholder-indigo-800::-ms-input-placeholder {
    color: #434190;
  }

  .sm\:placeholder-indigo-800::placeholder {
    color: #434190;
  }

  .sm\:placeholder-indigo-900::-webkit-input-placeholder {
    color: #3c366b;
  }

  .sm\:placeholder-indigo-900::-moz-placeholder {
    color: #3c366b;
  }

  .sm\:placeholder-indigo-900:-ms-input-placeholder {
    color: #3c366b;
  }

  .sm\:placeholder-indigo-900::-ms-input-placeholder {
    color: #3c366b;
  }

  .sm\:placeholder-indigo-900::placeholder {
    color: #3c366b;
  }

  .sm\:placeholder-purple-100::-webkit-input-placeholder {
    color: #faf5ff;
  }

  .sm\:placeholder-purple-100::-moz-placeholder {
    color: #faf5ff;
  }

  .sm\:placeholder-purple-100:-ms-input-placeholder {
    color: #faf5ff;
  }

  .sm\:placeholder-purple-100::-ms-input-placeholder {
    color: #faf5ff;
  }

  .sm\:placeholder-purple-100::placeholder {
    color: #faf5ff;
  }

  .sm\:placeholder-purple-200::-webkit-input-placeholder {
    color: #e9d8fd;
  }

  .sm\:placeholder-purple-200::-moz-placeholder {
    color: #e9d8fd;
  }

  .sm\:placeholder-purple-200:-ms-input-placeholder {
    color: #e9d8fd;
  }

  .sm\:placeholder-purple-200::-ms-input-placeholder {
    color: #e9d8fd;
  }

  .sm\:placeholder-purple-200::placeholder {
    color: #e9d8fd;
  }

  .sm\:placeholder-purple-300::-webkit-input-placeholder {
    color: #d6bcfa;
  }

  .sm\:placeholder-purple-300::-moz-placeholder {
    color: #d6bcfa;
  }

  .sm\:placeholder-purple-300:-ms-input-placeholder {
    color: #d6bcfa;
  }

  .sm\:placeholder-purple-300::-ms-input-placeholder {
    color: #d6bcfa;
  }

  .sm\:placeholder-purple-300::placeholder {
    color: #d6bcfa;
  }

  .sm\:placeholder-purple-400::-webkit-input-placeholder {
    color: #b794f4;
  }

  .sm\:placeholder-purple-400::-moz-placeholder {
    color: #b794f4;
  }

  .sm\:placeholder-purple-400:-ms-input-placeholder {
    color: #b794f4;
  }

  .sm\:placeholder-purple-400::-ms-input-placeholder {
    color: #b794f4;
  }

  .sm\:placeholder-purple-400::placeholder {
    color: #b794f4;
  }

  .sm\:placeholder-purple-500::-webkit-input-placeholder {
    color: #9f7aea;
  }

  .sm\:placeholder-purple-500::-moz-placeholder {
    color: #9f7aea;
  }

  .sm\:placeholder-purple-500:-ms-input-placeholder {
    color: #9f7aea;
  }

  .sm\:placeholder-purple-500::-ms-input-placeholder {
    color: #9f7aea;
  }

  .sm\:placeholder-purple-500::placeholder {
    color: #9f7aea;
  }

  .sm\:placeholder-purple-600::-webkit-input-placeholder {
    color: #805ad5;
  }

  .sm\:placeholder-purple-600::-moz-placeholder {
    color: #805ad5;
  }

  .sm\:placeholder-purple-600:-ms-input-placeholder {
    color: #805ad5;
  }

  .sm\:placeholder-purple-600::-ms-input-placeholder {
    color: #805ad5;
  }

  .sm\:placeholder-purple-600::placeholder {
    color: #805ad5;
  }

  .sm\:placeholder-purple-700::-webkit-input-placeholder {
    color: #6b46c1;
  }

  .sm\:placeholder-purple-700::-moz-placeholder {
    color: #6b46c1;
  }

  .sm\:placeholder-purple-700:-ms-input-placeholder {
    color: #6b46c1;
  }

  .sm\:placeholder-purple-700::-ms-input-placeholder {
    color: #6b46c1;
  }

  .sm\:placeholder-purple-700::placeholder {
    color: #6b46c1;
  }

  .sm\:placeholder-purple-800::-webkit-input-placeholder {
    color: #553c9a;
  }

  .sm\:placeholder-purple-800::-moz-placeholder {
    color: #553c9a;
  }

  .sm\:placeholder-purple-800:-ms-input-placeholder {
    color: #553c9a;
  }

  .sm\:placeholder-purple-800::-ms-input-placeholder {
    color: #553c9a;
  }

  .sm\:placeholder-purple-800::placeholder {
    color: #553c9a;
  }

  .sm\:placeholder-purple-900::-webkit-input-placeholder {
    color: #44337a;
  }

  .sm\:placeholder-purple-900::-moz-placeholder {
    color: #44337a;
  }

  .sm\:placeholder-purple-900:-ms-input-placeholder {
    color: #44337a;
  }

  .sm\:placeholder-purple-900::-ms-input-placeholder {
    color: #44337a;
  }

  .sm\:placeholder-purple-900::placeholder {
    color: #44337a;
  }

  .sm\:placeholder-pink-100::-webkit-input-placeholder {
    color: #fff5f7;
  }

  .sm\:placeholder-pink-100::-moz-placeholder {
    color: #fff5f7;
  }

  .sm\:placeholder-pink-100:-ms-input-placeholder {
    color: #fff5f7;
  }

  .sm\:placeholder-pink-100::-ms-input-placeholder {
    color: #fff5f7;
  }

  .sm\:placeholder-pink-100::placeholder {
    color: #fff5f7;
  }

  .sm\:placeholder-pink-200::-webkit-input-placeholder {
    color: #fed7e2;
  }

  .sm\:placeholder-pink-200::-moz-placeholder {
    color: #fed7e2;
  }

  .sm\:placeholder-pink-200:-ms-input-placeholder {
    color: #fed7e2;
  }

  .sm\:placeholder-pink-200::-ms-input-placeholder {
    color: #fed7e2;
  }

  .sm\:placeholder-pink-200::placeholder {
    color: #fed7e2;
  }

  .sm\:placeholder-pink-300::-webkit-input-placeholder {
    color: #fbb6ce;
  }

  .sm\:placeholder-pink-300::-moz-placeholder {
    color: #fbb6ce;
  }

  .sm\:placeholder-pink-300:-ms-input-placeholder {
    color: #fbb6ce;
  }

  .sm\:placeholder-pink-300::-ms-input-placeholder {
    color: #fbb6ce;
  }

  .sm\:placeholder-pink-300::placeholder {
    color: #fbb6ce;
  }

  .sm\:placeholder-pink-400::-webkit-input-placeholder {
    color: #f687b3;
  }

  .sm\:placeholder-pink-400::-moz-placeholder {
    color: #f687b3;
  }

  .sm\:placeholder-pink-400:-ms-input-placeholder {
    color: #f687b3;
  }

  .sm\:placeholder-pink-400::-ms-input-placeholder {
    color: #f687b3;
  }

  .sm\:placeholder-pink-400::placeholder {
    color: #f687b3;
  }

  .sm\:placeholder-pink-500::-webkit-input-placeholder {
    color: #ed64a6;
  }

  .sm\:placeholder-pink-500::-moz-placeholder {
    color: #ed64a6;
  }

  .sm\:placeholder-pink-500:-ms-input-placeholder {
    color: #ed64a6;
  }

  .sm\:placeholder-pink-500::-ms-input-placeholder {
    color: #ed64a6;
  }

  .sm\:placeholder-pink-500::placeholder {
    color: #ed64a6;
  }

  .sm\:placeholder-pink-600::-webkit-input-placeholder {
    color: #d53f8c;
  }

  .sm\:placeholder-pink-600::-moz-placeholder {
    color: #d53f8c;
  }

  .sm\:placeholder-pink-600:-ms-input-placeholder {
    color: #d53f8c;
  }

  .sm\:placeholder-pink-600::-ms-input-placeholder {
    color: #d53f8c;
  }

  .sm\:placeholder-pink-600::placeholder {
    color: #d53f8c;
  }

  .sm\:placeholder-pink-700::-webkit-input-placeholder {
    color: #b83280;
  }

  .sm\:placeholder-pink-700::-moz-placeholder {
    color: #b83280;
  }

  .sm\:placeholder-pink-700:-ms-input-placeholder {
    color: #b83280;
  }

  .sm\:placeholder-pink-700::-ms-input-placeholder {
    color: #b83280;
  }

  .sm\:placeholder-pink-700::placeholder {
    color: #b83280;
  }

  .sm\:placeholder-pink-800::-webkit-input-placeholder {
    color: #97266d;
  }

  .sm\:placeholder-pink-800::-moz-placeholder {
    color: #97266d;
  }

  .sm\:placeholder-pink-800:-ms-input-placeholder {
    color: #97266d;
  }

  .sm\:placeholder-pink-800::-ms-input-placeholder {
    color: #97266d;
  }

  .sm\:placeholder-pink-800::placeholder {
    color: #97266d;
  }

  .sm\:placeholder-pink-900::-webkit-input-placeholder {
    color: #702459;
  }

  .sm\:placeholder-pink-900::-moz-placeholder {
    color: #702459;
  }

  .sm\:placeholder-pink-900:-ms-input-placeholder {
    color: #702459;
  }

  .sm\:placeholder-pink-900::-ms-input-placeholder {
    color: #702459;
  }

  .sm\:placeholder-pink-900::placeholder {
    color: #702459;
  }

  .sm\:focus\:placeholder-transparent:focus::-webkit-input-placeholder {
    color: transparent;
  }

  .sm\:focus\:placeholder-transparent:focus::-moz-placeholder {
    color: transparent;
  }

  .sm\:focus\:placeholder-transparent:focus:-ms-input-placeholder {
    color: transparent;
  }

  .sm\:focus\:placeholder-transparent:focus::-ms-input-placeholder {
    color: transparent;
  }

  .sm\:focus\:placeholder-transparent:focus::placeholder {
    color: transparent;
  }

  .sm\:focus\:placeholder-black:focus::-webkit-input-placeholder {
    color: #000;
  }

  .sm\:focus\:placeholder-black:focus::-moz-placeholder {
    color: #000;
  }

  .sm\:focus\:placeholder-black:focus:-ms-input-placeholder {
    color: #000;
  }

  .sm\:focus\:placeholder-black:focus::-ms-input-placeholder {
    color: #000;
  }

  .sm\:focus\:placeholder-black:focus::placeholder {
    color: #000;
  }

  .sm\:focus\:placeholder-white:focus::-webkit-input-placeholder {
    color: #fff;
  }

  .sm\:focus\:placeholder-white:focus::-moz-placeholder {
    color: #fff;
  }

  .sm\:focus\:placeholder-white:focus:-ms-input-placeholder {
    color: #fff;
  }

  .sm\:focus\:placeholder-white:focus::-ms-input-placeholder {
    color: #fff;
  }

  .sm\:focus\:placeholder-white:focus::placeholder {
    color: #fff;
  }

  .sm\:focus\:placeholder-gray-100:focus::-webkit-input-placeholder {
    color: #f7fafc;
  }

  .sm\:focus\:placeholder-gray-100:focus::-moz-placeholder {
    color: #f7fafc;
  }

  .sm\:focus\:placeholder-gray-100:focus:-ms-input-placeholder {
    color: #f7fafc;
  }

  .sm\:focus\:placeholder-gray-100:focus::-ms-input-placeholder {
    color: #f7fafc;
  }

  .sm\:focus\:placeholder-gray-100:focus::placeholder {
    color: #f7fafc;
  }

  .sm\:focus\:placeholder-gray-200:focus::-webkit-input-placeholder {
    color: #edf2f7;
  }

  .sm\:focus\:placeholder-gray-200:focus::-moz-placeholder {
    color: #edf2f7;
  }

  .sm\:focus\:placeholder-gray-200:focus:-ms-input-placeholder {
    color: #edf2f7;
  }

  .sm\:focus\:placeholder-gray-200:focus::-ms-input-placeholder {
    color: #edf2f7;
  }

  .sm\:focus\:placeholder-gray-200:focus::placeholder {
    color: #edf2f7;
  }

  .sm\:focus\:placeholder-gray-300:focus::-webkit-input-placeholder {
    color: #e2e8f0;
  }

  .sm\:focus\:placeholder-gray-300:focus::-moz-placeholder {
    color: #e2e8f0;
  }

  .sm\:focus\:placeholder-gray-300:focus:-ms-input-placeholder {
    color: #e2e8f0;
  }

  .sm\:focus\:placeholder-gray-300:focus::-ms-input-placeholder {
    color: #e2e8f0;
  }

  .sm\:focus\:placeholder-gray-300:focus::placeholder {
    color: #e2e8f0;
  }

  .sm\:focus\:placeholder-gray-400:focus::-webkit-input-placeholder {
    color: #cbd5e0;
  }

  .sm\:focus\:placeholder-gray-400:focus::-moz-placeholder {
    color: #cbd5e0;
  }

  .sm\:focus\:placeholder-gray-400:focus:-ms-input-placeholder {
    color: #cbd5e0;
  }

  .sm\:focus\:placeholder-gray-400:focus::-ms-input-placeholder {
    color: #cbd5e0;
  }

  .sm\:focus\:placeholder-gray-400:focus::placeholder {
    color: #cbd5e0;
  }

  .sm\:focus\:placeholder-gray-500:focus::-webkit-input-placeholder {
    color: #a0aec0;
  }

  .sm\:focus\:placeholder-gray-500:focus::-moz-placeholder {
    color: #a0aec0;
  }

  .sm\:focus\:placeholder-gray-500:focus:-ms-input-placeholder {
    color: #a0aec0;
  }

  .sm\:focus\:placeholder-gray-500:focus::-ms-input-placeholder {
    color: #a0aec0;
  }

  .sm\:focus\:placeholder-gray-500:focus::placeholder {
    color: #a0aec0;
  }

  .sm\:focus\:placeholder-gray-600:focus::-webkit-input-placeholder {
    color: #718096;
  }

  .sm\:focus\:placeholder-gray-600:focus::-moz-placeholder {
    color: #718096;
  }

  .sm\:focus\:placeholder-gray-600:focus:-ms-input-placeholder {
    color: #718096;
  }

  .sm\:focus\:placeholder-gray-600:focus::-ms-input-placeholder {
    color: #718096;
  }

  .sm\:focus\:placeholder-gray-600:focus::placeholder {
    color: #718096;
  }

  .sm\:focus\:placeholder-gray-700:focus::-webkit-input-placeholder {
    color: #4a5568;
  }

  .sm\:focus\:placeholder-gray-700:focus::-moz-placeholder {
    color: #4a5568;
  }

  .sm\:focus\:placeholder-gray-700:focus:-ms-input-placeholder {
    color: #4a5568;
  }

  .sm\:focus\:placeholder-gray-700:focus::-ms-input-placeholder {
    color: #4a5568;
  }

  .sm\:focus\:placeholder-gray-700:focus::placeholder {
    color: #4a5568;
  }

  .sm\:focus\:placeholder-gray-800:focus::-webkit-input-placeholder {
    color: #2d3748;
  }

  .sm\:focus\:placeholder-gray-800:focus::-moz-placeholder {
    color: #2d3748;
  }

  .sm\:focus\:placeholder-gray-800:focus:-ms-input-placeholder {
    color: #2d3748;
  }

  .sm\:focus\:placeholder-gray-800:focus::-ms-input-placeholder {
    color: #2d3748;
  }

  .sm\:focus\:placeholder-gray-800:focus::placeholder {
    color: #2d3748;
  }

  .sm\:focus\:placeholder-gray-900:focus::-webkit-input-placeholder {
    color: #1a202c;
  }

  .sm\:focus\:placeholder-gray-900:focus::-moz-placeholder {
    color: #1a202c;
  }

  .sm\:focus\:placeholder-gray-900:focus:-ms-input-placeholder {
    color: #1a202c;
  }

  .sm\:focus\:placeholder-gray-900:focus::-ms-input-placeholder {
    color: #1a202c;
  }

  .sm\:focus\:placeholder-gray-900:focus::placeholder {
    color: #1a202c;
  }

  .sm\:focus\:placeholder-red-100:focus::-webkit-input-placeholder {
    color: #fff5f5;
  }

  .sm\:focus\:placeholder-red-100:focus::-moz-placeholder {
    color: #fff5f5;
  }

  .sm\:focus\:placeholder-red-100:focus:-ms-input-placeholder {
    color: #fff5f5;
  }

  .sm\:focus\:placeholder-red-100:focus::-ms-input-placeholder {
    color: #fff5f5;
  }

  .sm\:focus\:placeholder-red-100:focus::placeholder {
    color: #fff5f5;
  }

  .sm\:focus\:placeholder-red-200:focus::-webkit-input-placeholder {
    color: #fed7d7;
  }

  .sm\:focus\:placeholder-red-200:focus::-moz-placeholder {
    color: #fed7d7;
  }

  .sm\:focus\:placeholder-red-200:focus:-ms-input-placeholder {
    color: #fed7d7;
  }

  .sm\:focus\:placeholder-red-200:focus::-ms-input-placeholder {
    color: #fed7d7;
  }

  .sm\:focus\:placeholder-red-200:focus::placeholder {
    color: #fed7d7;
  }

  .sm\:focus\:placeholder-red-300:focus::-webkit-input-placeholder {
    color: #feb2b2;
  }

  .sm\:focus\:placeholder-red-300:focus::-moz-placeholder {
    color: #feb2b2;
  }

  .sm\:focus\:placeholder-red-300:focus:-ms-input-placeholder {
    color: #feb2b2;
  }

  .sm\:focus\:placeholder-red-300:focus::-ms-input-placeholder {
    color: #feb2b2;
  }

  .sm\:focus\:placeholder-red-300:focus::placeholder {
    color: #feb2b2;
  }

  .sm\:focus\:placeholder-red-400:focus::-webkit-input-placeholder {
    color: #fc8181;
  }

  .sm\:focus\:placeholder-red-400:focus::-moz-placeholder {
    color: #fc8181;
  }

  .sm\:focus\:placeholder-red-400:focus:-ms-input-placeholder {
    color: #fc8181;
  }

  .sm\:focus\:placeholder-red-400:focus::-ms-input-placeholder {
    color: #fc8181;
  }

  .sm\:focus\:placeholder-red-400:focus::placeholder {
    color: #fc8181;
  }

  .sm\:focus\:placeholder-red-500:focus::-webkit-input-placeholder {
    color: #f56565;
  }

  .sm\:focus\:placeholder-red-500:focus::-moz-placeholder {
    color: #f56565;
  }

  .sm\:focus\:placeholder-red-500:focus:-ms-input-placeholder {
    color: #f56565;
  }

  .sm\:focus\:placeholder-red-500:focus::-ms-input-placeholder {
    color: #f56565;
  }

  .sm\:focus\:placeholder-red-500:focus::placeholder {
    color: #f56565;
  }

  .sm\:focus\:placeholder-red-600:focus::-webkit-input-placeholder {
    color: #e53e3e;
  }

  .sm\:focus\:placeholder-red-600:focus::-moz-placeholder {
    color: #e53e3e;
  }

  .sm\:focus\:placeholder-red-600:focus:-ms-input-placeholder {
    color: #e53e3e;
  }

  .sm\:focus\:placeholder-red-600:focus::-ms-input-placeholder {
    color: #e53e3e;
  }

  .sm\:focus\:placeholder-red-600:focus::placeholder {
    color: #e53e3e;
  }

  .sm\:focus\:placeholder-red-700:focus::-webkit-input-placeholder {
    color: #c53030;
  }

  .sm\:focus\:placeholder-red-700:focus::-moz-placeholder {
    color: #c53030;
  }

  .sm\:focus\:placeholder-red-700:focus:-ms-input-placeholder {
    color: #c53030;
  }

  .sm\:focus\:placeholder-red-700:focus::-ms-input-placeholder {
    color: #c53030;
  }

  .sm\:focus\:placeholder-red-700:focus::placeholder {
    color: #c53030;
  }

  .sm\:focus\:placeholder-red-800:focus::-webkit-input-placeholder {
    color: #9b2c2c;
  }

  .sm\:focus\:placeholder-red-800:focus::-moz-placeholder {
    color: #9b2c2c;
  }

  .sm\:focus\:placeholder-red-800:focus:-ms-input-placeholder {
    color: #9b2c2c;
  }

  .sm\:focus\:placeholder-red-800:focus::-ms-input-placeholder {
    color: #9b2c2c;
  }

  .sm\:focus\:placeholder-red-800:focus::placeholder {
    color: #9b2c2c;
  }

  .sm\:focus\:placeholder-red-900:focus::-webkit-input-placeholder {
    color: #742a2a;
  }

  .sm\:focus\:placeholder-red-900:focus::-moz-placeholder {
    color: #742a2a;
  }

  .sm\:focus\:placeholder-red-900:focus:-ms-input-placeholder {
    color: #742a2a;
  }

  .sm\:focus\:placeholder-red-900:focus::-ms-input-placeholder {
    color: #742a2a;
  }

  .sm\:focus\:placeholder-red-900:focus::placeholder {
    color: #742a2a;
  }

  .sm\:focus\:placeholder-orange-100:focus::-webkit-input-placeholder {
    color: #fffaf0;
  }

  .sm\:focus\:placeholder-orange-100:focus::-moz-placeholder {
    color: #fffaf0;
  }

  .sm\:focus\:placeholder-orange-100:focus:-ms-input-placeholder {
    color: #fffaf0;
  }

  .sm\:focus\:placeholder-orange-100:focus::-ms-input-placeholder {
    color: #fffaf0;
  }

  .sm\:focus\:placeholder-orange-100:focus::placeholder {
    color: #fffaf0;
  }

  .sm\:focus\:placeholder-orange-200:focus::-webkit-input-placeholder {
    color: #feebc8;
  }

  .sm\:focus\:placeholder-orange-200:focus::-moz-placeholder {
    color: #feebc8;
  }

  .sm\:focus\:placeholder-orange-200:focus:-ms-input-placeholder {
    color: #feebc8;
  }

  .sm\:focus\:placeholder-orange-200:focus::-ms-input-placeholder {
    color: #feebc8;
  }

  .sm\:focus\:placeholder-orange-200:focus::placeholder {
    color: #feebc8;
  }

  .sm\:focus\:placeholder-orange-300:focus::-webkit-input-placeholder {
    color: #fbd38d;
  }

  .sm\:focus\:placeholder-orange-300:focus::-moz-placeholder {
    color: #fbd38d;
  }

  .sm\:focus\:placeholder-orange-300:focus:-ms-input-placeholder {
    color: #fbd38d;
  }

  .sm\:focus\:placeholder-orange-300:focus::-ms-input-placeholder {
    color: #fbd38d;
  }

  .sm\:focus\:placeholder-orange-300:focus::placeholder {
    color: #fbd38d;
  }

  .sm\:focus\:placeholder-orange-400:focus::-webkit-input-placeholder {
    color: #f6ad55;
  }

  .sm\:focus\:placeholder-orange-400:focus::-moz-placeholder {
    color: #f6ad55;
  }

  .sm\:focus\:placeholder-orange-400:focus:-ms-input-placeholder {
    color: #f6ad55;
  }

  .sm\:focus\:placeholder-orange-400:focus::-ms-input-placeholder {
    color: #f6ad55;
  }

  .sm\:focus\:placeholder-orange-400:focus::placeholder {
    color: #f6ad55;
  }

  .sm\:focus\:placeholder-orange-500:focus::-webkit-input-placeholder {
    color: #ed8936;
  }

  .sm\:focus\:placeholder-orange-500:focus::-moz-placeholder {
    color: #ed8936;
  }

  .sm\:focus\:placeholder-orange-500:focus:-ms-input-placeholder {
    color: #ed8936;
  }

  .sm\:focus\:placeholder-orange-500:focus::-ms-input-placeholder {
    color: #ed8936;
  }

  .sm\:focus\:placeholder-orange-500:focus::placeholder {
    color: #ed8936;
  }

  .sm\:focus\:placeholder-orange-600:focus::-webkit-input-placeholder {
    color: #dd6b20;
  }

  .sm\:focus\:placeholder-orange-600:focus::-moz-placeholder {
    color: #dd6b20;
  }

  .sm\:focus\:placeholder-orange-600:focus:-ms-input-placeholder {
    color: #dd6b20;
  }

  .sm\:focus\:placeholder-orange-600:focus::-ms-input-placeholder {
    color: #dd6b20;
  }

  .sm\:focus\:placeholder-orange-600:focus::placeholder {
    color: #dd6b20;
  }

  .sm\:focus\:placeholder-orange-700:focus::-webkit-input-placeholder {
    color: #c05621;
  }

  .sm\:focus\:placeholder-orange-700:focus::-moz-placeholder {
    color: #c05621;
  }

  .sm\:focus\:placeholder-orange-700:focus:-ms-input-placeholder {
    color: #c05621;
  }

  .sm\:focus\:placeholder-orange-700:focus::-ms-input-placeholder {
    color: #c05621;
  }

  .sm\:focus\:placeholder-orange-700:focus::placeholder {
    color: #c05621;
  }

  .sm\:focus\:placeholder-orange-800:focus::-webkit-input-placeholder {
    color: #9c4221;
  }

  .sm\:focus\:placeholder-orange-800:focus::-moz-placeholder {
    color: #9c4221;
  }

  .sm\:focus\:placeholder-orange-800:focus:-ms-input-placeholder {
    color: #9c4221;
  }

  .sm\:focus\:placeholder-orange-800:focus::-ms-input-placeholder {
    color: #9c4221;
  }

  .sm\:focus\:placeholder-orange-800:focus::placeholder {
    color: #9c4221;
  }

  .sm\:focus\:placeholder-orange-900:focus::-webkit-input-placeholder {
    color: #7b341e;
  }

  .sm\:focus\:placeholder-orange-900:focus::-moz-placeholder {
    color: #7b341e;
  }

  .sm\:focus\:placeholder-orange-900:focus:-ms-input-placeholder {
    color: #7b341e;
  }

  .sm\:focus\:placeholder-orange-900:focus::-ms-input-placeholder {
    color: #7b341e;
  }

  .sm\:focus\:placeholder-orange-900:focus::placeholder {
    color: #7b341e;
  }

  .sm\:focus\:placeholder-yellow-100:focus::-webkit-input-placeholder {
    color: #fffff0;
  }

  .sm\:focus\:placeholder-yellow-100:focus::-moz-placeholder {
    color: #fffff0;
  }

  .sm\:focus\:placeholder-yellow-100:focus:-ms-input-placeholder {
    color: #fffff0;
  }

  .sm\:focus\:placeholder-yellow-100:focus::-ms-input-placeholder {
    color: #fffff0;
  }

  .sm\:focus\:placeholder-yellow-100:focus::placeholder {
    color: #fffff0;
  }

  .sm\:focus\:placeholder-yellow-200:focus::-webkit-input-placeholder {
    color: #fefcbf;
  }

  .sm\:focus\:placeholder-yellow-200:focus::-moz-placeholder {
    color: #fefcbf;
  }

  .sm\:focus\:placeholder-yellow-200:focus:-ms-input-placeholder {
    color: #fefcbf;
  }

  .sm\:focus\:placeholder-yellow-200:focus::-ms-input-placeholder {
    color: #fefcbf;
  }

  .sm\:focus\:placeholder-yellow-200:focus::placeholder {
    color: #fefcbf;
  }

  .sm\:focus\:placeholder-yellow-300:focus::-webkit-input-placeholder {
    color: #faf089;
  }

  .sm\:focus\:placeholder-yellow-300:focus::-moz-placeholder {
    color: #faf089;
  }

  .sm\:focus\:placeholder-yellow-300:focus:-ms-input-placeholder {
    color: #faf089;
  }

  .sm\:focus\:placeholder-yellow-300:focus::-ms-input-placeholder {
    color: #faf089;
  }

  .sm\:focus\:placeholder-yellow-300:focus::placeholder {
    color: #faf089;
  }

  .sm\:focus\:placeholder-yellow-400:focus::-webkit-input-placeholder {
    color: #f6e05e;
  }

  .sm\:focus\:placeholder-yellow-400:focus::-moz-placeholder {
    color: #f6e05e;
  }

  .sm\:focus\:placeholder-yellow-400:focus:-ms-input-placeholder {
    color: #f6e05e;
  }

  .sm\:focus\:placeholder-yellow-400:focus::-ms-input-placeholder {
    color: #f6e05e;
  }

  .sm\:focus\:placeholder-yellow-400:focus::placeholder {
    color: #f6e05e;
  }

  .sm\:focus\:placeholder-yellow-500:focus::-webkit-input-placeholder {
    color: #ecc94b;
  }

  .sm\:focus\:placeholder-yellow-500:focus::-moz-placeholder {
    color: #ecc94b;
  }

  .sm\:focus\:placeholder-yellow-500:focus:-ms-input-placeholder {
    color: #ecc94b;
  }

  .sm\:focus\:placeholder-yellow-500:focus::-ms-input-placeholder {
    color: #ecc94b;
  }

  .sm\:focus\:placeholder-yellow-500:focus::placeholder {
    color: #ecc94b;
  }

  .sm\:focus\:placeholder-yellow-600:focus::-webkit-input-placeholder {
    color: #d69e2e;
  }

  .sm\:focus\:placeholder-yellow-600:focus::-moz-placeholder {
    color: #d69e2e;
  }

  .sm\:focus\:placeholder-yellow-600:focus:-ms-input-placeholder {
    color: #d69e2e;
  }

  .sm\:focus\:placeholder-yellow-600:focus::-ms-input-placeholder {
    color: #d69e2e;
  }

  .sm\:focus\:placeholder-yellow-600:focus::placeholder {
    color: #d69e2e;
  }

  .sm\:focus\:placeholder-yellow-700:focus::-webkit-input-placeholder {
    color: #b7791f;
  }

  .sm\:focus\:placeholder-yellow-700:focus::-moz-placeholder {
    color: #b7791f;
  }

  .sm\:focus\:placeholder-yellow-700:focus:-ms-input-placeholder {
    color: #b7791f;
  }

  .sm\:focus\:placeholder-yellow-700:focus::-ms-input-placeholder {
    color: #b7791f;
  }

  .sm\:focus\:placeholder-yellow-700:focus::placeholder {
    color: #b7791f;
  }

  .sm\:focus\:placeholder-yellow-800:focus::-webkit-input-placeholder {
    color: #975a16;
  }

  .sm\:focus\:placeholder-yellow-800:focus::-moz-placeholder {
    color: #975a16;
  }

  .sm\:focus\:placeholder-yellow-800:focus:-ms-input-placeholder {
    color: #975a16;
  }

  .sm\:focus\:placeholder-yellow-800:focus::-ms-input-placeholder {
    color: #975a16;
  }

  .sm\:focus\:placeholder-yellow-800:focus::placeholder {
    color: #975a16;
  }

  .sm\:focus\:placeholder-yellow-900:focus::-webkit-input-placeholder {
    color: #744210;
  }

  .sm\:focus\:placeholder-yellow-900:focus::-moz-placeholder {
    color: #744210;
  }

  .sm\:focus\:placeholder-yellow-900:focus:-ms-input-placeholder {
    color: #744210;
  }

  .sm\:focus\:placeholder-yellow-900:focus::-ms-input-placeholder {
    color: #744210;
  }

  .sm\:focus\:placeholder-yellow-900:focus::placeholder {
    color: #744210;
  }

  .sm\:focus\:placeholder-green-100:focus::-webkit-input-placeholder {
    color: #f0fff4;
  }

  .sm\:focus\:placeholder-green-100:focus::-moz-placeholder {
    color: #f0fff4;
  }

  .sm\:focus\:placeholder-green-100:focus:-ms-input-placeholder {
    color: #f0fff4;
  }

  .sm\:focus\:placeholder-green-100:focus::-ms-input-placeholder {
    color: #f0fff4;
  }

  .sm\:focus\:placeholder-green-100:focus::placeholder {
    color: #f0fff4;
  }

  .sm\:focus\:placeholder-green-200:focus::-webkit-input-placeholder {
    color: #c6f6d5;
  }

  .sm\:focus\:placeholder-green-200:focus::-moz-placeholder {
    color: #c6f6d5;
  }

  .sm\:focus\:placeholder-green-200:focus:-ms-input-placeholder {
    color: #c6f6d5;
  }

  .sm\:focus\:placeholder-green-200:focus::-ms-input-placeholder {
    color: #c6f6d5;
  }

  .sm\:focus\:placeholder-green-200:focus::placeholder {
    color: #c6f6d5;
  }

  .sm\:focus\:placeholder-green-300:focus::-webkit-input-placeholder {
    color: #9ae6b4;
  }

  .sm\:focus\:placeholder-green-300:focus::-moz-placeholder {
    color: #9ae6b4;
  }

  .sm\:focus\:placeholder-green-300:focus:-ms-input-placeholder {
    color: #9ae6b4;
  }

  .sm\:focus\:placeholder-green-300:focus::-ms-input-placeholder {
    color: #9ae6b4;
  }

  .sm\:focus\:placeholder-green-300:focus::placeholder {
    color: #9ae6b4;
  }

  .sm\:focus\:placeholder-green-400:focus::-webkit-input-placeholder {
    color: #68d391;
  }

  .sm\:focus\:placeholder-green-400:focus::-moz-placeholder {
    color: #68d391;
  }

  .sm\:focus\:placeholder-green-400:focus:-ms-input-placeholder {
    color: #68d391;
  }

  .sm\:focus\:placeholder-green-400:focus::-ms-input-placeholder {
    color: #68d391;
  }

  .sm\:focus\:placeholder-green-400:focus::placeholder {
    color: #68d391;
  }

  .sm\:focus\:placeholder-green-500:focus::-webkit-input-placeholder {
    color: #48bb78;
  }

  .sm\:focus\:placeholder-green-500:focus::-moz-placeholder {
    color: #48bb78;
  }

  .sm\:focus\:placeholder-green-500:focus:-ms-input-placeholder {
    color: #48bb78;
  }

  .sm\:focus\:placeholder-green-500:focus::-ms-input-placeholder {
    color: #48bb78;
  }

  .sm\:focus\:placeholder-green-500:focus::placeholder {
    color: #48bb78;
  }

  .sm\:focus\:placeholder-green-600:focus::-webkit-input-placeholder {
    color: #38a169;
  }

  .sm\:focus\:placeholder-green-600:focus::-moz-placeholder {
    color: #38a169;
  }

  .sm\:focus\:placeholder-green-600:focus:-ms-input-placeholder {
    color: #38a169;
  }

  .sm\:focus\:placeholder-green-600:focus::-ms-input-placeholder {
    color: #38a169;
  }

  .sm\:focus\:placeholder-green-600:focus::placeholder {
    color: #38a169;
  }

  .sm\:focus\:placeholder-green-700:focus::-webkit-input-placeholder {
    color: #2f855a;
  }

  .sm\:focus\:placeholder-green-700:focus::-moz-placeholder {
    color: #2f855a;
  }

  .sm\:focus\:placeholder-green-700:focus:-ms-input-placeholder {
    color: #2f855a;
  }

  .sm\:focus\:placeholder-green-700:focus::-ms-input-placeholder {
    color: #2f855a;
  }

  .sm\:focus\:placeholder-green-700:focus::placeholder {
    color: #2f855a;
  }

  .sm\:focus\:placeholder-green-800:focus::-webkit-input-placeholder {
    color: #276749;
  }

  .sm\:focus\:placeholder-green-800:focus::-moz-placeholder {
    color: #276749;
  }

  .sm\:focus\:placeholder-green-800:focus:-ms-input-placeholder {
    color: #276749;
  }

  .sm\:focus\:placeholder-green-800:focus::-ms-input-placeholder {
    color: #276749;
  }

  .sm\:focus\:placeholder-green-800:focus::placeholder {
    color: #276749;
  }

  .sm\:focus\:placeholder-green-900:focus::-webkit-input-placeholder {
    color: #22543d;
  }

  .sm\:focus\:placeholder-green-900:focus::-moz-placeholder {
    color: #22543d;
  }

  .sm\:focus\:placeholder-green-900:focus:-ms-input-placeholder {
    color: #22543d;
  }

  .sm\:focus\:placeholder-green-900:focus::-ms-input-placeholder {
    color: #22543d;
  }

  .sm\:focus\:placeholder-green-900:focus::placeholder {
    color: #22543d;
  }

  .sm\:focus\:placeholder-teal-100:focus::-webkit-input-placeholder {
    color: #e6fffa;
  }

  .sm\:focus\:placeholder-teal-100:focus::-moz-placeholder {
    color: #e6fffa;
  }

  .sm\:focus\:placeholder-teal-100:focus:-ms-input-placeholder {
    color: #e6fffa;
  }

  .sm\:focus\:placeholder-teal-100:focus::-ms-input-placeholder {
    color: #e6fffa;
  }

  .sm\:focus\:placeholder-teal-100:focus::placeholder {
    color: #e6fffa;
  }

  .sm\:focus\:placeholder-teal-200:focus::-webkit-input-placeholder {
    color: #b2f5ea;
  }

  .sm\:focus\:placeholder-teal-200:focus::-moz-placeholder {
    color: #b2f5ea;
  }

  .sm\:focus\:placeholder-teal-200:focus:-ms-input-placeholder {
    color: #b2f5ea;
  }

  .sm\:focus\:placeholder-teal-200:focus::-ms-input-placeholder {
    color: #b2f5ea;
  }

  .sm\:focus\:placeholder-teal-200:focus::placeholder {
    color: #b2f5ea;
  }

  .sm\:focus\:placeholder-teal-300:focus::-webkit-input-placeholder {
    color: #81e6d9;
  }

  .sm\:focus\:placeholder-teal-300:focus::-moz-placeholder {
    color: #81e6d9;
  }

  .sm\:focus\:placeholder-teal-300:focus:-ms-input-placeholder {
    color: #81e6d9;
  }

  .sm\:focus\:placeholder-teal-300:focus::-ms-input-placeholder {
    color: #81e6d9;
  }

  .sm\:focus\:placeholder-teal-300:focus::placeholder {
    color: #81e6d9;
  }

  .sm\:focus\:placeholder-teal-400:focus::-webkit-input-placeholder {
    color: #4fd1c5;
  }

  .sm\:focus\:placeholder-teal-400:focus::-moz-placeholder {
    color: #4fd1c5;
  }

  .sm\:focus\:placeholder-teal-400:focus:-ms-input-placeholder {
    color: #4fd1c5;
  }

  .sm\:focus\:placeholder-teal-400:focus::-ms-input-placeholder {
    color: #4fd1c5;
  }

  .sm\:focus\:placeholder-teal-400:focus::placeholder {
    color: #4fd1c5;
  }

  .sm\:focus\:placeholder-teal-500:focus::-webkit-input-placeholder {
    color: #38b2ac;
  }

  .sm\:focus\:placeholder-teal-500:focus::-moz-placeholder {
    color: #38b2ac;
  }

  .sm\:focus\:placeholder-teal-500:focus:-ms-input-placeholder {
    color: #38b2ac;
  }

  .sm\:focus\:placeholder-teal-500:focus::-ms-input-placeholder {
    color: #38b2ac;
  }

  .sm\:focus\:placeholder-teal-500:focus::placeholder {
    color: #38b2ac;
  }

  .sm\:focus\:placeholder-teal-600:focus::-webkit-input-placeholder {
    color: #319795;
  }

  .sm\:focus\:placeholder-teal-600:focus::-moz-placeholder {
    color: #319795;
  }

  .sm\:focus\:placeholder-teal-600:focus:-ms-input-placeholder {
    color: #319795;
  }

  .sm\:focus\:placeholder-teal-600:focus::-ms-input-placeholder {
    color: #319795;
  }

  .sm\:focus\:placeholder-teal-600:focus::placeholder {
    color: #319795;
  }

  .sm\:focus\:placeholder-teal-700:focus::-webkit-input-placeholder {
    color: #2c7a7b;
  }

  .sm\:focus\:placeholder-teal-700:focus::-moz-placeholder {
    color: #2c7a7b;
  }

  .sm\:focus\:placeholder-teal-700:focus:-ms-input-placeholder {
    color: #2c7a7b;
  }

  .sm\:focus\:placeholder-teal-700:focus::-ms-input-placeholder {
    color: #2c7a7b;
  }

  .sm\:focus\:placeholder-teal-700:focus::placeholder {
    color: #2c7a7b;
  }

  .sm\:focus\:placeholder-teal-800:focus::-webkit-input-placeholder {
    color: #285e61;
  }

  .sm\:focus\:placeholder-teal-800:focus::-moz-placeholder {
    color: #285e61;
  }

  .sm\:focus\:placeholder-teal-800:focus:-ms-input-placeholder {
    color: #285e61;
  }

  .sm\:focus\:placeholder-teal-800:focus::-ms-input-placeholder {
    color: #285e61;
  }

  .sm\:focus\:placeholder-teal-800:focus::placeholder {
    color: #285e61;
  }

  .sm\:focus\:placeholder-teal-900:focus::-webkit-input-placeholder {
    color: #234e52;
  }

  .sm\:focus\:placeholder-teal-900:focus::-moz-placeholder {
    color: #234e52;
  }

  .sm\:focus\:placeholder-teal-900:focus:-ms-input-placeholder {
    color: #234e52;
  }

  .sm\:focus\:placeholder-teal-900:focus::-ms-input-placeholder {
    color: #234e52;
  }

  .sm\:focus\:placeholder-teal-900:focus::placeholder {
    color: #234e52;
  }

  .sm\:focus\:placeholder-blue-100:focus::-webkit-input-placeholder {
    color: #ebf8ff;
  }

  .sm\:focus\:placeholder-blue-100:focus::-moz-placeholder {
    color: #ebf8ff;
  }

  .sm\:focus\:placeholder-blue-100:focus:-ms-input-placeholder {
    color: #ebf8ff;
  }

  .sm\:focus\:placeholder-blue-100:focus::-ms-input-placeholder {
    color: #ebf8ff;
  }

  .sm\:focus\:placeholder-blue-100:focus::placeholder {
    color: #ebf8ff;
  }

  .sm\:focus\:placeholder-blue-200:focus::-webkit-input-placeholder {
    color: #bee3f8;
  }

  .sm\:focus\:placeholder-blue-200:focus::-moz-placeholder {
    color: #bee3f8;
  }

  .sm\:focus\:placeholder-blue-200:focus:-ms-input-placeholder {
    color: #bee3f8;
  }

  .sm\:focus\:placeholder-blue-200:focus::-ms-input-placeholder {
    color: #bee3f8;
  }

  .sm\:focus\:placeholder-blue-200:focus::placeholder {
    color: #bee3f8;
  }

  .sm\:focus\:placeholder-blue-300:focus::-webkit-input-placeholder {
    color: #90cdf4;
  }

  .sm\:focus\:placeholder-blue-300:focus::-moz-placeholder {
    color: #90cdf4;
  }

  .sm\:focus\:placeholder-blue-300:focus:-ms-input-placeholder {
    color: #90cdf4;
  }

  .sm\:focus\:placeholder-blue-300:focus::-ms-input-placeholder {
    color: #90cdf4;
  }

  .sm\:focus\:placeholder-blue-300:focus::placeholder {
    color: #90cdf4;
  }

  .sm\:focus\:placeholder-blue-400:focus::-webkit-input-placeholder {
    color: #63b3ed;
  }

  .sm\:focus\:placeholder-blue-400:focus::-moz-placeholder {
    color: #63b3ed;
  }

  .sm\:focus\:placeholder-blue-400:focus:-ms-input-placeholder {
    color: #63b3ed;
  }

  .sm\:focus\:placeholder-blue-400:focus::-ms-input-placeholder {
    color: #63b3ed;
  }

  .sm\:focus\:placeholder-blue-400:focus::placeholder {
    color: #63b3ed;
  }

  .sm\:focus\:placeholder-blue-500:focus::-webkit-input-placeholder {
    color: #4299e1;
  }

  .sm\:focus\:placeholder-blue-500:focus::-moz-placeholder {
    color: #4299e1;
  }

  .sm\:focus\:placeholder-blue-500:focus:-ms-input-placeholder {
    color: #4299e1;
  }

  .sm\:focus\:placeholder-blue-500:focus::-ms-input-placeholder {
    color: #4299e1;
  }

  .sm\:focus\:placeholder-blue-500:focus::placeholder {
    color: #4299e1;
  }

  .sm\:focus\:placeholder-blue-600:focus::-webkit-input-placeholder {
    color: #3182ce;
  }

  .sm\:focus\:placeholder-blue-600:focus::-moz-placeholder {
    color: #3182ce;
  }

  .sm\:focus\:placeholder-blue-600:focus:-ms-input-placeholder {
    color: #3182ce;
  }

  .sm\:focus\:placeholder-blue-600:focus::-ms-input-placeholder {
    color: #3182ce;
  }

  .sm\:focus\:placeholder-blue-600:focus::placeholder {
    color: #3182ce;
  }

  .sm\:focus\:placeholder-blue-700:focus::-webkit-input-placeholder {
    color: #2b6cb0;
  }

  .sm\:focus\:placeholder-blue-700:focus::-moz-placeholder {
    color: #2b6cb0;
  }

  .sm\:focus\:placeholder-blue-700:focus:-ms-input-placeholder {
    color: #2b6cb0;
  }

  .sm\:focus\:placeholder-blue-700:focus::-ms-input-placeholder {
    color: #2b6cb0;
  }

  .sm\:focus\:placeholder-blue-700:focus::placeholder {
    color: #2b6cb0;
  }

  .sm\:focus\:placeholder-blue-800:focus::-webkit-input-placeholder {
    color: #2c5282;
  }

  .sm\:focus\:placeholder-blue-800:focus::-moz-placeholder {
    color: #2c5282;
  }

  .sm\:focus\:placeholder-blue-800:focus:-ms-input-placeholder {
    color: #2c5282;
  }

  .sm\:focus\:placeholder-blue-800:focus::-ms-input-placeholder {
    color: #2c5282;
  }

  .sm\:focus\:placeholder-blue-800:focus::placeholder {
    color: #2c5282;
  }

  .sm\:focus\:placeholder-blue-900:focus::-webkit-input-placeholder {
    color: #2a4365;
  }

  .sm\:focus\:placeholder-blue-900:focus::-moz-placeholder {
    color: #2a4365;
  }

  .sm\:focus\:placeholder-blue-900:focus:-ms-input-placeholder {
    color: #2a4365;
  }

  .sm\:focus\:placeholder-blue-900:focus::-ms-input-placeholder {
    color: #2a4365;
  }

  .sm\:focus\:placeholder-blue-900:focus::placeholder {
    color: #2a4365;
  }

  .sm\:focus\:placeholder-indigo-100:focus::-webkit-input-placeholder {
    color: #ebf4ff;
  }

  .sm\:focus\:placeholder-indigo-100:focus::-moz-placeholder {
    color: #ebf4ff;
  }

  .sm\:focus\:placeholder-indigo-100:focus:-ms-input-placeholder {
    color: #ebf4ff;
  }

  .sm\:focus\:placeholder-indigo-100:focus::-ms-input-placeholder {
    color: #ebf4ff;
  }

  .sm\:focus\:placeholder-indigo-100:focus::placeholder {
    color: #ebf4ff;
  }

  .sm\:focus\:placeholder-indigo-200:focus::-webkit-input-placeholder {
    color: #c3dafe;
  }

  .sm\:focus\:placeholder-indigo-200:focus::-moz-placeholder {
    color: #c3dafe;
  }

  .sm\:focus\:placeholder-indigo-200:focus:-ms-input-placeholder {
    color: #c3dafe;
  }

  .sm\:focus\:placeholder-indigo-200:focus::-ms-input-placeholder {
    color: #c3dafe;
  }

  .sm\:focus\:placeholder-indigo-200:focus::placeholder {
    color: #c3dafe;
  }

  .sm\:focus\:placeholder-indigo-300:focus::-webkit-input-placeholder {
    color: #a3bffa;
  }

  .sm\:focus\:placeholder-indigo-300:focus::-moz-placeholder {
    color: #a3bffa;
  }

  .sm\:focus\:placeholder-indigo-300:focus:-ms-input-placeholder {
    color: #a3bffa;
  }

  .sm\:focus\:placeholder-indigo-300:focus::-ms-input-placeholder {
    color: #a3bffa;
  }

  .sm\:focus\:placeholder-indigo-300:focus::placeholder {
    color: #a3bffa;
  }

  .sm\:focus\:placeholder-indigo-400:focus::-webkit-input-placeholder {
    color: #7f9cf5;
  }

  .sm\:focus\:placeholder-indigo-400:focus::-moz-placeholder {
    color: #7f9cf5;
  }

  .sm\:focus\:placeholder-indigo-400:focus:-ms-input-placeholder {
    color: #7f9cf5;
  }

  .sm\:focus\:placeholder-indigo-400:focus::-ms-input-placeholder {
    color: #7f9cf5;
  }

  .sm\:focus\:placeholder-indigo-400:focus::placeholder {
    color: #7f9cf5;
  }

  .sm\:focus\:placeholder-indigo-500:focus::-webkit-input-placeholder {
    color: #667eea;
  }

  .sm\:focus\:placeholder-indigo-500:focus::-moz-placeholder {
    color: #667eea;
  }

  .sm\:focus\:placeholder-indigo-500:focus:-ms-input-placeholder {
    color: #667eea;
  }

  .sm\:focus\:placeholder-indigo-500:focus::-ms-input-placeholder {
    color: #667eea;
  }

  .sm\:focus\:placeholder-indigo-500:focus::placeholder {
    color: #667eea;
  }

  .sm\:focus\:placeholder-indigo-600:focus::-webkit-input-placeholder {
    color: #5a67d8;
  }

  .sm\:focus\:placeholder-indigo-600:focus::-moz-placeholder {
    color: #5a67d8;
  }

  .sm\:focus\:placeholder-indigo-600:focus:-ms-input-placeholder {
    color: #5a67d8;
  }

  .sm\:focus\:placeholder-indigo-600:focus::-ms-input-placeholder {
    color: #5a67d8;
  }

  .sm\:focus\:placeholder-indigo-600:focus::placeholder {
    color: #5a67d8;
  }

  .sm\:focus\:placeholder-indigo-700:focus::-webkit-input-placeholder {
    color: #4c51bf;
  }

  .sm\:focus\:placeholder-indigo-700:focus::-moz-placeholder {
    color: #4c51bf;
  }

  .sm\:focus\:placeholder-indigo-700:focus:-ms-input-placeholder {
    color: #4c51bf;
  }

  .sm\:focus\:placeholder-indigo-700:focus::-ms-input-placeholder {
    color: #4c51bf;
  }

  .sm\:focus\:placeholder-indigo-700:focus::placeholder {
    color: #4c51bf;
  }

  .sm\:focus\:placeholder-indigo-800:focus::-webkit-input-placeholder {
    color: #434190;
  }

  .sm\:focus\:placeholder-indigo-800:focus::-moz-placeholder {
    color: #434190;
  }

  .sm\:focus\:placeholder-indigo-800:focus:-ms-input-placeholder {
    color: #434190;
  }

  .sm\:focus\:placeholder-indigo-800:focus::-ms-input-placeholder {
    color: #434190;
  }

  .sm\:focus\:placeholder-indigo-800:focus::placeholder {
    color: #434190;
  }

  .sm\:focus\:placeholder-indigo-900:focus::-webkit-input-placeholder {
    color: #3c366b;
  }

  .sm\:focus\:placeholder-indigo-900:focus::-moz-placeholder {
    color: #3c366b;
  }

  .sm\:focus\:placeholder-indigo-900:focus:-ms-input-placeholder {
    color: #3c366b;
  }

  .sm\:focus\:placeholder-indigo-900:focus::-ms-input-placeholder {
    color: #3c366b;
  }

  .sm\:focus\:placeholder-indigo-900:focus::placeholder {
    color: #3c366b;
  }

  .sm\:focus\:placeholder-purple-100:focus::-webkit-input-placeholder {
    color: #faf5ff;
  }

  .sm\:focus\:placeholder-purple-100:focus::-moz-placeholder {
    color: #faf5ff;
  }

  .sm\:focus\:placeholder-purple-100:focus:-ms-input-placeholder {
    color: #faf5ff;
  }

  .sm\:focus\:placeholder-purple-100:focus::-ms-input-placeholder {
    color: #faf5ff;
  }

  .sm\:focus\:placeholder-purple-100:focus::placeholder {
    color: #faf5ff;
  }

  .sm\:focus\:placeholder-purple-200:focus::-webkit-input-placeholder {
    color: #e9d8fd;
  }

  .sm\:focus\:placeholder-purple-200:focus::-moz-placeholder {
    color: #e9d8fd;
  }

  .sm\:focus\:placeholder-purple-200:focus:-ms-input-placeholder {
    color: #e9d8fd;
  }

  .sm\:focus\:placeholder-purple-200:focus::-ms-input-placeholder {
    color: #e9d8fd;
  }

  .sm\:focus\:placeholder-purple-200:focus::placeholder {
    color: #e9d8fd;
  }

  .sm\:focus\:placeholder-purple-300:focus::-webkit-input-placeholder {
    color: #d6bcfa;
  }

  .sm\:focus\:placeholder-purple-300:focus::-moz-placeholder {
    color: #d6bcfa;
  }

  .sm\:focus\:placeholder-purple-300:focus:-ms-input-placeholder {
    color: #d6bcfa;
  }

  .sm\:focus\:placeholder-purple-300:focus::-ms-input-placeholder {
    color: #d6bcfa;
  }

  .sm\:focus\:placeholder-purple-300:focus::placeholder {
    color: #d6bcfa;
  }

  .sm\:focus\:placeholder-purple-400:focus::-webkit-input-placeholder {
    color: #b794f4;
  }

  .sm\:focus\:placeholder-purple-400:focus::-moz-placeholder {
    color: #b794f4;
  }

  .sm\:focus\:placeholder-purple-400:focus:-ms-input-placeholder {
    color: #b794f4;
  }

  .sm\:focus\:placeholder-purple-400:focus::-ms-input-placeholder {
    color: #b794f4;
  }

  .sm\:focus\:placeholder-purple-400:focus::placeholder {
    color: #b794f4;
  }

  .sm\:focus\:placeholder-purple-500:focus::-webkit-input-placeholder {
    color: #9f7aea;
  }

  .sm\:focus\:placeholder-purple-500:focus::-moz-placeholder {
    color: #9f7aea;
  }

  .sm\:focus\:placeholder-purple-500:focus:-ms-input-placeholder {
    color: #9f7aea;
  }

  .sm\:focus\:placeholder-purple-500:focus::-ms-input-placeholder {
    color: #9f7aea;
  }

  .sm\:focus\:placeholder-purple-500:focus::placeholder {
    color: #9f7aea;
  }

  .sm\:focus\:placeholder-purple-600:focus::-webkit-input-placeholder {
    color: #805ad5;
  }

  .sm\:focus\:placeholder-purple-600:focus::-moz-placeholder {
    color: #805ad5;
  }

  .sm\:focus\:placeholder-purple-600:focus:-ms-input-placeholder {
    color: #805ad5;
  }

  .sm\:focus\:placeholder-purple-600:focus::-ms-input-placeholder {
    color: #805ad5;
  }

  .sm\:focus\:placeholder-purple-600:focus::placeholder {
    color: #805ad5;
  }

  .sm\:focus\:placeholder-purple-700:focus::-webkit-input-placeholder {
    color: #6b46c1;
  }

  .sm\:focus\:placeholder-purple-700:focus::-moz-placeholder {
    color: #6b46c1;
  }

  .sm\:focus\:placeholder-purple-700:focus:-ms-input-placeholder {
    color: #6b46c1;
  }

  .sm\:focus\:placeholder-purple-700:focus::-ms-input-placeholder {
    color: #6b46c1;
  }

  .sm\:focus\:placeholder-purple-700:focus::placeholder {
    color: #6b46c1;
  }

  .sm\:focus\:placeholder-purple-800:focus::-webkit-input-placeholder {
    color: #553c9a;
  }

  .sm\:focus\:placeholder-purple-800:focus::-moz-placeholder {
    color: #553c9a;
  }

  .sm\:focus\:placeholder-purple-800:focus:-ms-input-placeholder {
    color: #553c9a;
  }

  .sm\:focus\:placeholder-purple-800:focus::-ms-input-placeholder {
    color: #553c9a;
  }

  .sm\:focus\:placeholder-purple-800:focus::placeholder {
    color: #553c9a;
  }

  .sm\:focus\:placeholder-purple-900:focus::-webkit-input-placeholder {
    color: #44337a;
  }

  .sm\:focus\:placeholder-purple-900:focus::-moz-placeholder {
    color: #44337a;
  }

  .sm\:focus\:placeholder-purple-900:focus:-ms-input-placeholder {
    color: #44337a;
  }

  .sm\:focus\:placeholder-purple-900:focus::-ms-input-placeholder {
    color: #44337a;
  }

  .sm\:focus\:placeholder-purple-900:focus::placeholder {
    color: #44337a;
  }

  .sm\:focus\:placeholder-pink-100:focus::-webkit-input-placeholder {
    color: #fff5f7;
  }

  .sm\:focus\:placeholder-pink-100:focus::-moz-placeholder {
    color: #fff5f7;
  }

  .sm\:focus\:placeholder-pink-100:focus:-ms-input-placeholder {
    color: #fff5f7;
  }

  .sm\:focus\:placeholder-pink-100:focus::-ms-input-placeholder {
    color: #fff5f7;
  }

  .sm\:focus\:placeholder-pink-100:focus::placeholder {
    color: #fff5f7;
  }

  .sm\:focus\:placeholder-pink-200:focus::-webkit-input-placeholder {
    color: #fed7e2;
  }

  .sm\:focus\:placeholder-pink-200:focus::-moz-placeholder {
    color: #fed7e2;
  }

  .sm\:focus\:placeholder-pink-200:focus:-ms-input-placeholder {
    color: #fed7e2;
  }

  .sm\:focus\:placeholder-pink-200:focus::-ms-input-placeholder {
    color: #fed7e2;
  }

  .sm\:focus\:placeholder-pink-200:focus::placeholder {
    color: #fed7e2;
  }

  .sm\:focus\:placeholder-pink-300:focus::-webkit-input-placeholder {
    color: #fbb6ce;
  }

  .sm\:focus\:placeholder-pink-300:focus::-moz-placeholder {
    color: #fbb6ce;
  }

  .sm\:focus\:placeholder-pink-300:focus:-ms-input-placeholder {
    color: #fbb6ce;
  }

  .sm\:focus\:placeholder-pink-300:focus::-ms-input-placeholder {
    color: #fbb6ce;
  }

  .sm\:focus\:placeholder-pink-300:focus::placeholder {
    color: #fbb6ce;
  }

  .sm\:focus\:placeholder-pink-400:focus::-webkit-input-placeholder {
    color: #f687b3;
  }

  .sm\:focus\:placeholder-pink-400:focus::-moz-placeholder {
    color: #f687b3;
  }

  .sm\:focus\:placeholder-pink-400:focus:-ms-input-placeholder {
    color: #f687b3;
  }

  .sm\:focus\:placeholder-pink-400:focus::-ms-input-placeholder {
    color: #f687b3;
  }

  .sm\:focus\:placeholder-pink-400:focus::placeholder {
    color: #f687b3;
  }

  .sm\:focus\:placeholder-pink-500:focus::-webkit-input-placeholder {
    color: #ed64a6;
  }

  .sm\:focus\:placeholder-pink-500:focus::-moz-placeholder {
    color: #ed64a6;
  }

  .sm\:focus\:placeholder-pink-500:focus:-ms-input-placeholder {
    color: #ed64a6;
  }

  .sm\:focus\:placeholder-pink-500:focus::-ms-input-placeholder {
    color: #ed64a6;
  }

  .sm\:focus\:placeholder-pink-500:focus::placeholder {
    color: #ed64a6;
  }

  .sm\:focus\:placeholder-pink-600:focus::-webkit-input-placeholder {
    color: #d53f8c;
  }

  .sm\:focus\:placeholder-pink-600:focus::-moz-placeholder {
    color: #d53f8c;
  }

  .sm\:focus\:placeholder-pink-600:focus:-ms-input-placeholder {
    color: #d53f8c;
  }

  .sm\:focus\:placeholder-pink-600:focus::-ms-input-placeholder {
    color: #d53f8c;
  }

  .sm\:focus\:placeholder-pink-600:focus::placeholder {
    color: #d53f8c;
  }

  .sm\:focus\:placeholder-pink-700:focus::-webkit-input-placeholder {
    color: #b83280;
  }

  .sm\:focus\:placeholder-pink-700:focus::-moz-placeholder {
    color: #b83280;
  }

  .sm\:focus\:placeholder-pink-700:focus:-ms-input-placeholder {
    color: #b83280;
  }

  .sm\:focus\:placeholder-pink-700:focus::-ms-input-placeholder {
    color: #b83280;
  }

  .sm\:focus\:placeholder-pink-700:focus::placeholder {
    color: #b83280;
  }

  .sm\:focus\:placeholder-pink-800:focus::-webkit-input-placeholder {
    color: #97266d;
  }

  .sm\:focus\:placeholder-pink-800:focus::-moz-placeholder {
    color: #97266d;
  }

  .sm\:focus\:placeholder-pink-800:focus:-ms-input-placeholder {
    color: #97266d;
  }

  .sm\:focus\:placeholder-pink-800:focus::-ms-input-placeholder {
    color: #97266d;
  }

  .sm\:focus\:placeholder-pink-800:focus::placeholder {
    color: #97266d;
  }

  .sm\:focus\:placeholder-pink-900:focus::-webkit-input-placeholder {
    color: #702459;
  }

  .sm\:focus\:placeholder-pink-900:focus::-moz-placeholder {
    color: #702459;
  }

  .sm\:focus\:placeholder-pink-900:focus:-ms-input-placeholder {
    color: #702459;
  }

  .sm\:focus\:placeholder-pink-900:focus::-ms-input-placeholder {
    color: #702459;
  }

  .sm\:focus\:placeholder-pink-900:focus::placeholder {
    color: #702459;
  }

  .sm\:pointer-events-none {
    pointer-events: none;
  }

  .sm\:pointer-events-auto {
    pointer-events: auto;
  }

  .sm\:static {
    position: static;
  }

  .sm\:fixed {
    position: fixed;
  }

  .sm\:absolute {
    position: absolute;
  }

  .sm\:relative {
    position: relative;
  }

  .sm\:sticky {
    position: -webkit-sticky;
    position: sticky;
  }

  .sm\:inset-0 {
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
  }

  .sm\:inset-auto {
    top: auto;
    right: auto;
    bottom: auto;
    left: auto;
  }

  .sm\:inset-y-0 {
    top: 0;
    bottom: 0;
  }

  .sm\:inset-x-0 {
    right: 0;
    left: 0;
  }

  .sm\:inset-y-auto {
    top: auto;
    bottom: auto;
  }

  .sm\:inset-x-auto {
    right: auto;
    left: auto;
  }

  .sm\:top-0 {
    top: 0;
  }

  .sm\:right-0 {
    right: 0;
  }

  .sm\:bottom-0 {
    bottom: 0;
  }

  .sm\:left-0 {
    left: 0;
  }

  .sm\:top-auto {
    top: auto;
  }

  .sm\:right-auto {
    right: auto;
  }

  .sm\:bottom-auto {
    bottom: auto;
  }

  .sm\:left-auto {
    left: auto;
  }

  .sm\:resize-none {
    resize: none;
  }

  .sm\:resize-y {
    resize: vertical;
  }

  .sm\:resize-x {
    resize: horizontal;
  }

  .sm\:resize {
    resize: both;
  }

  .sm\:shadow {
    box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06);
  }

  .sm\:shadow-md {
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
  }

  .sm\:shadow-lg {
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
  }

  .sm\:shadow-xl {
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
  }

  .sm\:shadow-2xl {
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
  }

  .sm\:shadow-inner {
    box-shadow: inset 0 2px 4px 0 rgba(0, 0, 0, 0.06);
  }

  .sm\:shadow-outline {
    box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.5);
  }

  .sm\:shadow-none {
    box-shadow: none;
  }

  .sm\:hover\:shadow:hover {
    box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06);
  }

  .sm\:hover\:shadow-md:hover {
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
  }

  .sm\:hover\:shadow-lg:hover {
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
  }

  .sm\:hover\:shadow-xl:hover {
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
  }

  .sm\:hover\:shadow-2xl:hover {
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
  }

  .sm\:hover\:shadow-inner:hover {
    box-shadow: inset 0 2px 4px 0 rgba(0, 0, 0, 0.06);
  }

  .sm\:hover\:shadow-outline:hover {
    box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.5);
  }

  .sm\:hover\:shadow-none:hover {
    box-shadow: none;
  }

  .sm\:focus\:shadow:focus {
    box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06);
  }

  .sm\:focus\:shadow-md:focus {
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
  }

  .sm\:focus\:shadow-lg:focus {
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
  }

  .sm\:focus\:shadow-xl:focus {
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
  }

  .sm\:focus\:shadow-2xl:focus {
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
  }

  .sm\:focus\:shadow-inner:focus {
    box-shadow: inset 0 2px 4px 0 rgba(0, 0, 0, 0.06);
  }

  .sm\:focus\:shadow-outline:focus {
    box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.5);
  }

  .sm\:focus\:shadow-none:focus {
    box-shadow: none;
  }

  .sm\:fill-current {
    fill: currentColor;
  }

  .sm\:stroke-current {
    stroke: currentColor;
  }

  .sm\:table-auto {
    table-layout: auto;
  }

  .sm\:table-fixed {
    table-layout: fixed;
  }

  .sm\:text-left {
    text-align: left;
  }

  .sm\:text-center {
    text-align: center;
  }

  .sm\:text-right {
    text-align: right;
  }

  .sm\:text-justify {
    text-align: justify;
  }

  .sm\:text-transparent {
    color: transparent;
  }

  .sm\:text-black {
    color: #000;
  }

  .sm\:text-white {
    color: #fff;
  }

  .sm\:text-gray-100 {
    color: #f7fafc;
  }

  .sm\:text-gray-200 {
    color: #edf2f7;
  }

  .sm\:text-gray-300 {
    color: #e2e8f0;
  }

  .sm\:text-gray-400 {
    color: #cbd5e0;
  }

  .sm\:text-gray-500 {
    color: #a0aec0;
  }

  .sm\:text-gray-600 {
    color: #718096;
  }

  .sm\:text-gray-700 {
    color: #4a5568;
  }

  .sm\:text-gray-800 {
    color: #2d3748;
  }

  .sm\:text-gray-900 {
    color: #1a202c;
  }

  .sm\:text-red-100 {
    color: #fff5f5;
  }

  .sm\:text-red-200 {
    color: #fed7d7;
  }

  .sm\:text-red-300 {
    color: #feb2b2;
  }

  .sm\:text-red-400 {
    color: #fc8181;
  }

  .sm\:text-red-500 {
    color: #f56565;
  }

  .sm\:text-red-600 {
    color: #e53e3e;
  }

  .sm\:text-red-700 {
    color: #c53030;
  }

  .sm\:text-red-800 {
    color: #9b2c2c;
  }

  .sm\:text-red-900 {
    color: #742a2a;
  }

  .sm\:text-orange-100 {
    color: #fffaf0;
  }

  .sm\:text-orange-200 {
    color: #feebc8;
  }

  .sm\:text-orange-300 {
    color: #fbd38d;
  }

  .sm\:text-orange-400 {
    color: #f6ad55;
  }

  .sm\:text-orange-500 {
    color: #ed8936;
  }

  .sm\:text-orange-600 {
    color: #dd6b20;
  }

  .sm\:text-orange-700 {
    color: #c05621;
  }

  .sm\:text-orange-800 {
    color: #9c4221;
  }

  .sm\:text-orange-900 {
    color: #7b341e;
  }

  .sm\:text-yellow-100 {
    color: #fffff0;
  }

  .sm\:text-yellow-200 {
    color: #fefcbf;
  }

  .sm\:text-yellow-300 {
    color: #faf089;
  }

  .sm\:text-yellow-400 {
    color: #f6e05e;
  }

  .sm\:text-yellow-500 {
    color: #ecc94b;
  }

  .sm\:text-yellow-600 {
    color: #d69e2e;
  }

  .sm\:text-yellow-700 {
    color: #b7791f;
  }

  .sm\:text-yellow-800 {
    color: #975a16;
  }

  .sm\:text-yellow-900 {
    color: #744210;
  }

  .sm\:text-green-100 {
    color: #f0fff4;
  }

  .sm\:text-green-200 {
    color: #c6f6d5;
  }

  .sm\:text-green-300 {
    color: #9ae6b4;
  }

  .sm\:text-green-400 {
    color: #68d391;
  }

  .sm\:text-green-500 {
    color: #48bb78;
  }

  .sm\:text-green-600 {
    color: #38a169;
  }

  .sm\:text-green-700 {
    color: #2f855a;
  }

  .sm\:text-green-800 {
    color: #276749;
  }

  .sm\:text-green-900 {
    color: #22543d;
  }

  .sm\:text-teal-100 {
    color: #e6fffa;
  }

  .sm\:text-teal-200 {
    color: #b2f5ea;
  }

  .sm\:text-teal-300 {
    color: #81e6d9;
  }

  .sm\:text-teal-400 {
    color: #4fd1c5;
  }

  .sm\:text-teal-500 {
    color: #38b2ac;
  }

  .sm\:text-teal-600 {
    color: #319795;
  }

  .sm\:text-teal-700 {
    color: #2c7a7b;
  }

  .sm\:text-teal-800 {
    color: #285e61;
  }

  .sm\:text-teal-900 {
    color: #234e52;
  }

  .sm\:text-blue-100 {
    color: #ebf8ff;
  }

  .sm\:text-blue-200 {
    color: #bee3f8;
  }

  .sm\:text-blue-300 {
    color: #90cdf4;
  }

  .sm\:text-blue-400 {
    color: #63b3ed;
  }

  .sm\:text-blue-500 {
    color: #4299e1;
  }

  .sm\:text-blue-600 {
    color: #3182ce;
  }

  .sm\:text-blue-700 {
    color: #2b6cb0;
  }

  .sm\:text-blue-800 {
    color: #2c5282;
  }

  .sm\:text-blue-900 {
    color: #2a4365;
  }

  .sm\:text-indigo-100 {
    color: #ebf4ff;
  }

  .sm\:text-indigo-200 {
    color: #c3dafe;
  }

  .sm\:text-indigo-300 {
    color: #a3bffa;
  }

  .sm\:text-indigo-400 {
    color: #7f9cf5;
  }

  .sm\:text-indigo-500 {
    color: #667eea;
  }

  .sm\:text-indigo-600 {
    color: #5a67d8;
  }

  .sm\:text-indigo-700 {
    color: #4c51bf;
  }

  .sm\:text-indigo-800 {
    color: #434190;
  }

  .sm\:text-indigo-900 {
    color: #3c366b;
  }

  .sm\:text-purple-100 {
    color: #faf5ff;
  }

  .sm\:text-purple-200 {
    color: #e9d8fd;
  }

  .sm\:text-purple-300 {
    color: #d6bcfa;
  }

  .sm\:text-purple-400 {
    color: #b794f4;
  }

  .sm\:text-purple-500 {
    color: #9f7aea;
  }

  .sm\:text-purple-600 {
    color: #805ad5;
  }

  .sm\:text-purple-700 {
    color: #6b46c1;
  }

  .sm\:text-purple-800 {
    color: #553c9a;
  }

  .sm\:text-purple-900 {
    color: #44337a;
  }

  .sm\:text-pink-100 {
    color: #fff5f7;
  }

  .sm\:text-pink-200 {
    color: #fed7e2;
  }

  .sm\:text-pink-300 {
    color: #fbb6ce;
  }

  .sm\:text-pink-400 {
    color: #f687b3;
  }

  .sm\:text-pink-500 {
    color: #ed64a6;
  }

  .sm\:text-pink-600 {
    color: #d53f8c;
  }

  .sm\:text-pink-700 {
    color: #b83280;
  }

  .sm\:text-pink-800 {
    color: #97266d;
  }

  .sm\:text-pink-900 {
    color: #702459;
  }

  .sm\:hover\:text-transparent:hover {
    color: transparent;
  }

  .sm\:hover\:text-black:hover {
    color: #000;
  }

  .sm\:hover\:text-white:hover {
    color: #fff;
  }

  .sm\:hover\:text-gray-100:hover {
    color: #f7fafc;
  }

  .sm\:hover\:text-gray-200:hover {
    color: #edf2f7;
  }

  .sm\:hover\:text-gray-300:hover {
    color: #e2e8f0;
  }

  .sm\:hover\:text-gray-400:hover {
    color: #cbd5e0;
  }

  .sm\:hover\:text-gray-500:hover {
    color: #a0aec0;
  }

  .sm\:hover\:text-gray-600:hover {
    color: #718096;
  }

  .sm\:hover\:text-gray-700:hover {
    color: #4a5568;
  }

  .sm\:hover\:text-gray-800:hover {
    color: #2d3748;
  }

  .sm\:hover\:text-gray-900:hover {
    color: #1a202c;
  }

  .sm\:hover\:text-red-100:hover {
    color: #fff5f5;
  }

  .sm\:hover\:text-red-200:hover {
    color: #fed7d7;
  }

  .sm\:hover\:text-red-300:hover {
    color: #feb2b2;
  }

  .sm\:hover\:text-red-400:hover {
    color: #fc8181;
  }

  .sm\:hover\:text-red-500:hover {
    color: #f56565;
  }

  .sm\:hover\:text-red-600:hover {
    color: #e53e3e;
  }

  .sm\:hover\:text-red-700:hover {
    color: #c53030;
  }

  .sm\:hover\:text-red-800:hover {
    color: #9b2c2c;
  }

  .sm\:hover\:text-red-900:hover {
    color: #742a2a;
  }

  .sm\:hover\:text-orange-100:hover {
    color: #fffaf0;
  }

  .sm\:hover\:text-orange-200:hover {
    color: #feebc8;
  }

  .sm\:hover\:text-orange-300:hover {
    color: #fbd38d;
  }

  .sm\:hover\:text-orange-400:hover {
    color: #f6ad55;
  }

  .sm\:hover\:text-orange-500:hover {
    color: #ed8936;
  }

  .sm\:hover\:text-orange-600:hover {
    color: #dd6b20;
  }

  .sm\:hover\:text-orange-700:hover {
    color: #c05621;
  }

  .sm\:hover\:text-orange-800:hover {
    color: #9c4221;
  }

  .sm\:hover\:text-orange-900:hover {
    color: #7b341e;
  }

  .sm\:hover\:text-yellow-100:hover {
    color: #fffff0;
  }

  .sm\:hover\:text-yellow-200:hover {
    color: #fefcbf;
  }

  .sm\:hover\:text-yellow-300:hover {
    color: #faf089;
  }

  .sm\:hover\:text-yellow-400:hover {
    color: #f6e05e;
  }

  .sm\:hover\:text-yellow-500:hover {
    color: #ecc94b;
  }

  .sm\:hover\:text-yellow-600:hover {
    color: #d69e2e;
  }

  .sm\:hover\:text-yellow-700:hover {
    color: #b7791f;
  }

  .sm\:hover\:text-yellow-800:hover {
    color: #975a16;
  }

  .sm\:hover\:text-yellow-900:hover {
    color: #744210;
  }

  .sm\:hover\:text-green-100:hover {
    color: #f0fff4;
  }

  .sm\:hover\:text-green-200:hover {
    color: #c6f6d5;
  }

  .sm\:hover\:text-green-300:hover {
    color: #9ae6b4;
  }

  .sm\:hover\:text-green-400:hover {
    color: #68d391;
  }

  .sm\:hover\:text-green-500:hover {
    color: #48bb78;
  }

  .sm\:hover\:text-green-600:hover {
    color: #38a169;
  }

  .sm\:hover\:text-green-700:hover {
    color: #2f855a;
  }

  .sm\:hover\:text-green-800:hover {
    color: #276749;
  }

  .sm\:hover\:text-green-900:hover {
    color: #22543d;
  }

  .sm\:hover\:text-teal-100:hover {
    color: #e6fffa;
  }

  .sm\:hover\:text-teal-200:hover {
    color: #b2f5ea;
  }

  .sm\:hover\:text-teal-300:hover {
    color: #81e6d9;
  }

  .sm\:hover\:text-teal-400:hover {
    color: #4fd1c5;
  }

  .sm\:hover\:text-teal-500:hover {
    color: #38b2ac;
  }

  .sm\:hover\:text-teal-600:hover {
    color: #319795;
  }

  .sm\:hover\:text-teal-700:hover {
    color: #2c7a7b;
  }

  .sm\:hover\:text-teal-800:hover {
    color: #285e61;
  }

  .sm\:hover\:text-teal-900:hover {
    color: #234e52;
  }

  .sm\:hover\:text-blue-100:hover {
    color: #ebf8ff;
  }

  .sm\:hover\:text-blue-200:hover {
    color: #bee3f8;
  }

  .sm\:hover\:text-blue-300:hover {
    color: #90cdf4;
  }

  .sm\:hover\:text-blue-400:hover {
    color: #63b3ed;
  }

  .sm\:hover\:text-blue-500:hover {
    color: #4299e1;
  }

  .sm\:hover\:text-blue-600:hover {
    color: #3182ce;
  }

  .sm\:hover\:text-blue-700:hover {
    color: #2b6cb0;
  }

  .sm\:hover\:text-blue-800:hover {
    color: #2c5282;
  }

  .sm\:hover\:text-blue-900:hover {
    color: #2a4365;
  }

  .sm\:hover\:text-indigo-100:hover {
    color: #ebf4ff;
  }

  .sm\:hover\:text-indigo-200:hover {
    color: #c3dafe;
  }

  .sm\:hover\:text-indigo-300:hover {
    color: #a3bffa;
  }

  .sm\:hover\:text-indigo-400:hover {
    color: #7f9cf5;
  }

  .sm\:hover\:text-indigo-500:hover {
    color: #667eea;
  }

  .sm\:hover\:text-indigo-600:hover {
    color: #5a67d8;
  }

  .sm\:hover\:text-indigo-700:hover {
    color: #4c51bf;
  }

  .sm\:hover\:text-indigo-800:hover {
    color: #434190;
  }

  .sm\:hover\:text-indigo-900:hover {
    color: #3c366b;
  }

  .sm\:hover\:text-purple-100:hover {
    color: #faf5ff;
  }

  .sm\:hover\:text-purple-200:hover {
    color: #e9d8fd;
  }

  .sm\:hover\:text-purple-300:hover {
    color: #d6bcfa;
  }

  .sm\:hover\:text-purple-400:hover {
    color: #b794f4;
  }

  .sm\:hover\:text-purple-500:hover {
    color: #9f7aea;
  }

  .sm\:hover\:text-purple-600:hover {
    color: #805ad5;
  }

  .sm\:hover\:text-purple-700:hover {
    color: #6b46c1;
  }

  .sm\:hover\:text-purple-800:hover {
    color: #553c9a;
  }

  .sm\:hover\:text-purple-900:hover {
    color: #44337a;
  }

  .sm\:hover\:text-pink-100:hover {
    color: #fff5f7;
  }

  .sm\:hover\:text-pink-200:hover {
    color: #fed7e2;
  }

  .sm\:hover\:text-pink-300:hover {
    color: #fbb6ce;
  }

  .sm\:hover\:text-pink-400:hover {
    color: #f687b3;
  }

  .sm\:hover\:text-pink-500:hover {
    color: #ed64a6;
  }

  .sm\:hover\:text-pink-600:hover {
    color: #d53f8c;
  }

  .sm\:hover\:text-pink-700:hover {
    color: #b83280;
  }

  .sm\:hover\:text-pink-800:hover {
    color: #97266d;
  }

  .sm\:hover\:text-pink-900:hover {
    color: #702459;
  }

  .sm\:focus\:text-transparent:focus {
    color: transparent;
  }

  .sm\:focus\:text-black:focus {
    color: #000;
  }

  .sm\:focus\:text-white:focus {
    color: #fff;
  }

  .sm\:focus\:text-gray-100:focus {
    color: #f7fafc;
  }

  .sm\:focus\:text-gray-200:focus {
    color: #edf2f7;
  }

  .sm\:focus\:text-gray-300:focus {
    color: #e2e8f0;
  }

  .sm\:focus\:text-gray-400:focus {
    color: #cbd5e0;
  }

  .sm\:focus\:text-gray-500:focus {
    color: #a0aec0;
  }

  .sm\:focus\:text-gray-600:focus {
    color: #718096;
  }

  .sm\:focus\:text-gray-700:focus {
    color: #4a5568;
  }

  .sm\:focus\:text-gray-800:focus {
    color: #2d3748;
  }

  .sm\:focus\:text-gray-900:focus {
    color: #1a202c;
  }

  .sm\:focus\:text-red-100:focus {
    color: #fff5f5;
  }

  .sm\:focus\:text-red-200:focus {
    color: #fed7d7;
  }

  .sm\:focus\:text-red-300:focus {
    color: #feb2b2;
  }

  .sm\:focus\:text-red-400:focus {
    color: #fc8181;
  }

  .sm\:focus\:text-red-500:focus {
    color: #f56565;
  }

  .sm\:focus\:text-red-600:focus {
    color: #e53e3e;
  }

  .sm\:focus\:text-red-700:focus {
    color: #c53030;
  }

  .sm\:focus\:text-red-800:focus {
    color: #9b2c2c;
  }

  .sm\:focus\:text-red-900:focus {
    color: #742a2a;
  }

  .sm\:focus\:text-orange-100:focus {
    color: #fffaf0;
  }

  .sm\:focus\:text-orange-200:focus {
    color: #feebc8;
  }

  .sm\:focus\:text-orange-300:focus {
    color: #fbd38d;
  }

  .sm\:focus\:text-orange-400:focus {
    color: #f6ad55;
  }

  .sm\:focus\:text-orange-500:focus {
    color: #ed8936;
  }

  .sm\:focus\:text-orange-600:focus {
    color: #dd6b20;
  }

  .sm\:focus\:text-orange-700:focus {
    color: #c05621;
  }

  .sm\:focus\:text-orange-800:focus {
    color: #9c4221;
  }

  .sm\:focus\:text-orange-900:focus {
    color: #7b341e;
  }

  .sm\:focus\:text-yellow-100:focus {
    color: #fffff0;
  }

  .sm\:focus\:text-yellow-200:focus {
    color: #fefcbf;
  }

  .sm\:focus\:text-yellow-300:focus {
    color: #faf089;
  }

  .sm\:focus\:text-yellow-400:focus {
    color: #f6e05e;
  }

  .sm\:focus\:text-yellow-500:focus {
    color: #ecc94b;
  }

  .sm\:focus\:text-yellow-600:focus {
    color: #d69e2e;
  }

  .sm\:focus\:text-yellow-700:focus {
    color: #b7791f;
  }

  .sm\:focus\:text-yellow-800:focus {
    color: #975a16;
  }

  .sm\:focus\:text-yellow-900:focus {
    color: #744210;
  }

  .sm\:focus\:text-green-100:focus {
    color: #f0fff4;
  }

  .sm\:focus\:text-green-200:focus {
    color: #c6f6d5;
  }

  .sm\:focus\:text-green-300:focus {
    color: #9ae6b4;
  }

  .sm\:focus\:text-green-400:focus {
    color: #68d391;
  }

  .sm\:focus\:text-green-500:focus {
    color: #48bb78;
  }

  .sm\:focus\:text-green-600:focus {
    color: #38a169;
  }

  .sm\:focus\:text-green-700:focus {
    color: #2f855a;
  }

  .sm\:focus\:text-green-800:focus {
    color: #276749;
  }

  .sm\:focus\:text-green-900:focus {
    color: #22543d;
  }

  .sm\:focus\:text-teal-100:focus {
    color: #e6fffa;
  }

  .sm\:focus\:text-teal-200:focus {
    color: #b2f5ea;
  }

  .sm\:focus\:text-teal-300:focus {
    color: #81e6d9;
  }

  .sm\:focus\:text-teal-400:focus {
    color: #4fd1c5;
  }

  .sm\:focus\:text-teal-500:focus {
    color: #38b2ac;
  }

  .sm\:focus\:text-teal-600:focus {
    color: #319795;
  }

  .sm\:focus\:text-teal-700:focus {
    color: #2c7a7b;
  }

  .sm\:focus\:text-teal-800:focus {
    color: #285e61;
  }

  .sm\:focus\:text-teal-900:focus {
    color: #234e52;
  }

  .sm\:focus\:text-blue-100:focus {
    color: #ebf8ff;
  }

  .sm\:focus\:text-blue-200:focus {
    color: #bee3f8;
  }

  .sm\:focus\:text-blue-300:focus {
    color: #90cdf4;
  }

  .sm\:focus\:text-blue-400:focus {
    color: #63b3ed;
  }

  .sm\:focus\:text-blue-500:focus {
    color: #4299e1;
  }

  .sm\:focus\:text-blue-600:focus {
    color: #3182ce;
  }

  .sm\:focus\:text-blue-700:focus {
    color: #2b6cb0;
  }

  .sm\:focus\:text-blue-800:focus {
    color: #2c5282;
  }

  .sm\:focus\:text-blue-900:focus {
    color: #2a4365;
  }

  .sm\:focus\:text-indigo-100:focus {
    color: #ebf4ff;
  }

  .sm\:focus\:text-indigo-200:focus {
    color: #c3dafe;
  }

  .sm\:focus\:text-indigo-300:focus {
    color: #a3bffa;
  }

  .sm\:focus\:text-indigo-400:focus {
    color: #7f9cf5;
  }

  .sm\:focus\:text-indigo-500:focus {
    color: #667eea;
  }

  .sm\:focus\:text-indigo-600:focus {
    color: #5a67d8;
  }

  .sm\:focus\:text-indigo-700:focus {
    color: #4c51bf;
  }

  .sm\:focus\:text-indigo-800:focus {
    color: #434190;
  }

  .sm\:focus\:text-indigo-900:focus {
    color: #3c366b;
  }

  .sm\:focus\:text-purple-100:focus {
    color: #faf5ff;
  }

  .sm\:focus\:text-purple-200:focus {
    color: #e9d8fd;
  }

  .sm\:focus\:text-purple-300:focus {
    color: #d6bcfa;
  }

  .sm\:focus\:text-purple-400:focus {
    color: #b794f4;
  }

  .sm\:focus\:text-purple-500:focus {
    color: #9f7aea;
  }

  .sm\:focus\:text-purple-600:focus {
    color: #805ad5;
  }

  .sm\:focus\:text-purple-700:focus {
    color: #6b46c1;
  }

  .sm\:focus\:text-purple-800:focus {
    color: #553c9a;
  }

  .sm\:focus\:text-purple-900:focus {
    color: #44337a;
  }

  .sm\:focus\:text-pink-100:focus {
    color: #fff5f7;
  }

  .sm\:focus\:text-pink-200:focus {
    color: #fed7e2;
  }

  .sm\:focus\:text-pink-300:focus {
    color: #fbb6ce;
  }

  .sm\:focus\:text-pink-400:focus {
    color: #f687b3;
  }

  .sm\:focus\:text-pink-500:focus {
    color: #ed64a6;
  }

  .sm\:focus\:text-pink-600:focus {
    color: #d53f8c;
  }

  .sm\:focus\:text-pink-700:focus {
    color: #b83280;
  }

  .sm\:focus\:text-pink-800:focus {
    color: #97266d;
  }

  .sm\:focus\:text-pink-900:focus {
    color: #702459;
  }

  .sm\:text-xs {
    font-size: 0.75rem;
  }

  .sm\:text-sm {
    font-size: 0.875rem;
  }

  .sm\:text-base {
    font-size: 1rem;
  }

  .sm\:text-lg {
    font-size: 1.125rem;
  }

  .sm\:text-xl {
    font-size: 1.25rem;
  }

  .sm\:text-2xl {
    font-size: 1.5rem;
  }

  .sm\:text-3xl {
    font-size: 1.875rem;
  }

  .sm\:text-4xl {
    font-size: 2.25rem;
  }

  .sm\:text-5xl {
    font-size: 3rem;
  }

  .sm\:text-6xl {
    font-size: 4rem;
  }

  .sm\:italic {
    font-style: italic;
  }

  .sm\:not-italic {
    font-style: normal;
  }

  .sm\:uppercase {
    text-transform: uppercase;
  }

  .sm\:lowercase {
    text-transform: lowercase;
  }

  .sm\:capitalize {
    text-transform: capitalize;
  }

  .sm\:normal-case {
    text-transform: none;
  }

  .sm\:underline {
    text-decoration: underline;
  }

  .sm\:line-through {
    text-decoration: line-through;
  }

  .sm\:no-underline {
    text-decoration: none;
  }

  .sm\:hover\:underline:hover {
    text-decoration: underline;
  }

  .sm\:hover\:line-through:hover {
    text-decoration: line-through;
  }

  .sm\:hover\:no-underline:hover {
    text-decoration: none;
  }

  .sm\:focus\:underline:focus {
    text-decoration: underline;
  }

  .sm\:focus\:line-through:focus {
    text-decoration: line-through;
  }

  .sm\:focus\:no-underline:focus {
    text-decoration: none;
  }

  .sm\:antialiased {
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
  }

  .sm\:subpixel-antialiased {
    -webkit-font-smoothing: auto;
    -moz-osx-font-smoothing: auto;
  }

  .sm\:tracking-tighter {
    letter-spacing: -0.05em;
  }

  .sm\:tracking-tight {
    letter-spacing: -0.025em;
  }

  .sm\:tracking-normal {
    letter-spacing: 0;
  }

  .sm\:tracking-wide {
    letter-spacing: 0.025em;
  }

  .sm\:tracking-wider {
    letter-spacing: 0.05em;
  }

  .sm\:tracking-widest {
    letter-spacing: 0.1em;
  }

  .sm\:select-none {
    -webkit-user-select: none;
       -moz-user-select: none;
        -ms-user-select: none;
            user-select: none;
  }

  .sm\:select-text {
    -webkit-user-select: text;
       -moz-user-select: text;
        -ms-user-select: text;
            user-select: text;
  }

  .sm\:select-all {
    -webkit-user-select: all;
       -moz-user-select: all;
        -ms-user-select: all;
            user-select: all;
  }

  .sm\:select-auto {
    -webkit-user-select: auto;
       -moz-user-select: auto;
        -ms-user-select: auto;
            user-select: auto;
  }

  .sm\:align-baseline {
    vertical-align: baseline;
  }

  .sm\:align-top {
    vertical-align: top;
  }

  .sm\:align-middle {
    vertical-align: middle;
  }

  .sm\:align-bottom {
    vertical-align: bottom;
  }

  .sm\:align-text-top {
    vertical-align: text-top;
  }

  .sm\:align-text-bottom {
    vertical-align: text-bottom;
  }

  .sm\:visible {
    visibility: visible;
  }

  .sm\:invisible {
    visibility: hidden;
  }

  .sm\:whitespace-normal {
    white-space: normal;
  }

  .sm\:whitespace-no-wrap {
    white-space: nowrap;
  }

  .sm\:whitespace-pre {
    white-space: pre;
  }

  .sm\:whitespace-pre-line {
    white-space: pre-line;
  }

  .sm\:whitespace-pre-wrap {
    white-space: pre-wrap;
  }

  .sm\:break-normal {
    overflow-wrap: normal;
    word-break: normal;
  }

  .sm\:break-words {
    overflow-wrap: break-word;
  }

  .sm\:break-all {
    word-break: break-all;
  }

  .sm\:truncate {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .sm\:w-0 {
    width: 0;
  }

  .sm\:w-1 {
    width: 0.25rem;
  }

  .sm\:w-2 {
    width: 0.5rem;
  }

  .sm\:w-3 {
    width: 0.75rem;
  }

  .sm\:w-4 {
    width: 1rem;
  }

  .sm\:w-5 {
    width: 1.25rem;
  }

  .sm\:w-6 {
    width: 1.5rem;
  }

  .sm\:w-8 {
    width: 2rem;
  }

  .sm\:w-10 {
    width: 2.5rem;
  }

  .sm\:w-12 {
    width: 3rem;
  }

  .sm\:w-16 {
    width: 4rem;
  }

  .sm\:w-20 {
    width: 5rem;
  }

  .sm\:w-24 {
    width: 6rem;
  }

  .sm\:w-32 {
    width: 8rem;
  }

  .sm\:w-40 {
    width: 10rem;
  }

  .sm\:w-48 {
    width: 12rem;
  }

  .sm\:w-56 {
    width: 14rem;
  }

  .sm\:w-64 {
    width: 16rem;
  }

  .sm\:w-auto {
    width: auto;
  }

  .sm\:w-px {
    width: 1px;
  }

  .sm\:w-1\/2 {
    width: 50%;
  }

  .sm\:w-1\/3 {
    width: 33.333333%;
  }

  .sm\:w-2\/3 {
    width: 66.666667%;
  }

  .sm\:w-1\/4 {
    width: 25%;
  }

  .sm\:w-2\/4 {
    width: 50%;
  }

  .sm\:w-3\/4 {
    width: 75%;
  }

  .sm\:w-1\/5 {
    width: 20%;
  }

  .sm\:w-2\/5 {
    width: 40%;
  }

  .sm\:w-3\/5 {
    width: 60%;
  }

  .sm\:w-4\/5 {
    width: 80%;
  }

  .sm\:w-1\/6 {
    width: 16.666667%;
  }

  .sm\:w-2\/6 {
    width: 33.333333%;
  }

  .sm\:w-3\/6 {
    width: 50%;
  }

  .sm\:w-4\/6 {
    width: 66.666667%;
  }

  .sm\:w-5\/6 {
    width: 83.333333%;
  }

  .sm\:w-1\/12 {
    width: 8.333333%;
  }

  .sm\:w-2\/12 {
    width: 16.666667%;
  }

  .sm\:w-3\/12 {
    width: 25%;
  }

  .sm\:w-4\/12 {
    width: 33.333333%;
  }

  .sm\:w-5\/12 {
    width: 41.666667%;
  }

  .sm\:w-6\/12 {
    width: 50%;
  }

  .sm\:w-7\/12 {
    width: 58.333333%;
  }

  .sm\:w-8\/12 {
    width: 66.666667%;
  }

  .sm\:w-9\/12 {
    width: 75%;
  }

  .sm\:w-10\/12 {
    width: 83.333333%;
  }

  .sm\:w-11\/12 {
    width: 91.666667%;
  }

  .sm\:w-full {
    width: 100%;
  }

  .sm\:w-screen {
    width: 100vw;
  }

  .sm\:z-0 {
    z-index: 0;
  }

  .sm\:z-10 {
    z-index: 10;
  }

  .sm\:z-20 {
    z-index: 20;
  }

  .sm\:z-30 {
    z-index: 30;
  }

  .sm\:z-40 {
    z-index: 40;
  }

  .sm\:z-50 {
    z-index: 50;
  }

  .sm\:z-auto {
    z-index: auto;
  }
}

@media (min-width: 768px) {
  .md\:sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border-width: 0;
  }

  .md\:not-sr-only {
    position: static;
    width: auto;
    height: auto;
    padding: 0;
    margin: 0;
    overflow: visible;
    clip: auto;
    white-space: normal;
  }

  .md\:focus\:sr-only:focus {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border-width: 0;
  }

  .md\:focus\:not-sr-only:focus {
    position: static;
    width: auto;
    height: auto;
    padding: 0;
    margin: 0;
    overflow: visible;
    clip: auto;
    white-space: normal;
  }

  .md\:appearance-none {
    -webkit-appearance: none;
       -moz-appearance: none;
            appearance: none;
  }

  .md\:bg-fixed {
    background-attachment: fixed;
  }

  .md\:bg-local {
    background-attachment: local;
  }

  .md\:bg-scroll {
    background-attachment: scroll;
  }

  .md\:bg-transparent {
    background-color: transparent;
  }

  .md\:bg-black {
    background-color: #000;
  }

  .md\:bg-white {
    background-color: #fff;
  }

  .md\:bg-gray-100 {
    background-color: #f7fafc;
  }

  .md\:bg-gray-200 {
    background-color: #edf2f7;
  }

  .md\:bg-gray-300 {
    background-color: #e2e8f0;
  }

  .md\:bg-gray-400 {
    background-color: #cbd5e0;
  }

  .md\:bg-gray-500 {
    background-color: #a0aec0;
  }

  .md\:bg-gray-600 {
    background-color: #718096;
  }

  .md\:bg-gray-700 {
    background-color: #4a5568;
  }

  .md\:bg-gray-800 {
    background-color: #2d3748;
  }

  .md\:bg-gray-900 {
    background-color: #1a202c;
  }

  .md\:bg-red-100 {
    background-color: #fff5f5;
  }

  .md\:bg-red-200 {
    background-color: #fed7d7;
  }

  .md\:bg-red-300 {
    background-color: #feb2b2;
  }

  .md\:bg-red-400 {
    background-color: #fc8181;
  }

  .md\:bg-red-500 {
    background-color: #f56565;
  }

  .md\:bg-red-600 {
    background-color: #e53e3e;
  }

  .md\:bg-red-700 {
    background-color: #c53030;
  }

  .md\:bg-red-800 {
    background-color: #9b2c2c;
  }

  .md\:bg-red-900 {
    background-color: #742a2a;
  }

  .md\:bg-orange-100 {
    background-color: #fffaf0;
  }

  .md\:bg-orange-200 {
    background-color: #feebc8;
  }

  .md\:bg-orange-300 {
    background-color: #fbd38d;
  }

  .md\:bg-orange-400 {
    background-color: #f6ad55;
  }

  .md\:bg-orange-500 {
    background-color: #ed8936;
  }

  .md\:bg-orange-600 {
    background-color: #dd6b20;
  }

  .md\:bg-orange-700 {
    background-color: #c05621;
  }

  .md\:bg-orange-800 {
    background-color: #9c4221;
  }

  .md\:bg-orange-900 {
    background-color: #7b341e;
  }

  .md\:bg-yellow-100 {
    background-color: #fffff0;
  }

  .md\:bg-yellow-200 {
    background-color: #fefcbf;
  }

  .md\:bg-yellow-300 {
    background-color: #faf089;
  }

  .md\:bg-yellow-400 {
    background-color: #f6e05e;
  }

  .md\:bg-yellow-500 {
    background-color: #ecc94b;
  }

  .md\:bg-yellow-600 {
    background-color: #d69e2e;
  }

  .md\:bg-yellow-700 {
    background-color: #b7791f;
  }

  .md\:bg-yellow-800 {
    background-color: #975a16;
  }

  .md\:bg-yellow-900 {
    background-color: #744210;
  }

  .md\:bg-green-100 {
    background-color: #f0fff4;
  }

  .md\:bg-green-200 {
    background-color: #c6f6d5;
  }

  .md\:bg-green-300 {
    background-color: #9ae6b4;
  }

  .md\:bg-green-400 {
    background-color: #68d391;
  }

  .md\:bg-green-500 {
    background-color: #48bb78;
  }

  .md\:bg-green-600 {
    background-color: #38a169;
  }

  .md\:bg-green-700 {
    background-color: #2f855a;
  }

  .md\:bg-green-800 {
    background-color: #276749;
  }

  .md\:bg-green-900 {
    background-color: #22543d;
  }

  .md\:bg-teal-100 {
    background-color: #e6fffa;
  }

  .md\:bg-teal-200 {
    background-color: #b2f5ea;
  }

  .md\:bg-teal-300 {
    background-color: #81e6d9;
  }

  .md\:bg-teal-400 {
    background-color: #4fd1c5;
  }

  .md\:bg-teal-500 {
    background-color: #38b2ac;
  }

  .md\:bg-teal-600 {
    background-color: #319795;
  }

  .md\:bg-teal-700 {
    background-color: #2c7a7b;
  }

  .md\:bg-teal-800 {
    background-color: #285e61;
  }

  .md\:bg-teal-900 {
    background-color: #234e52;
  }

  .md\:bg-blue-100 {
    background-color: #ebf8ff;
  }

  .md\:bg-blue-200 {
    background-color: #bee3f8;
  }

  .md\:bg-blue-300 {
    background-color: #90cdf4;
  }

  .md\:bg-blue-400 {
    background-color: #63b3ed;
  }

  .md\:bg-blue-500 {
    background-color: #4299e1;
  }

  .md\:bg-blue-600 {
    background-color: #3182ce;
  }

  .md\:bg-blue-700 {
    background-color: #2b6cb0;
  }

  .md\:bg-blue-800 {
    background-color: #2c5282;
  }

  .md\:bg-blue-900 {
    background-color: #2a4365;
  }

  .md\:bg-indigo-100 {
    background-color: #ebf4ff;
  }

  .md\:bg-indigo-200 {
    background-color: #c3dafe;
  }

  .md\:bg-indigo-300 {
    background-color: #a3bffa;
  }

  .md\:bg-indigo-400 {
    background-color: #7f9cf5;
  }

  .md\:bg-indigo-500 {
    background-color: #667eea;
  }

  .md\:bg-indigo-600 {
    background-color: #5a67d8;
  }

  .md\:bg-indigo-700 {
    background-color: #4c51bf;
  }

  .md\:bg-indigo-800 {
    background-color: #434190;
  }

  .md\:bg-indigo-900 {
    background-color: #3c366b;
  }

  .md\:bg-purple-100 {
    background-color: #faf5ff;
  }

  .md\:bg-purple-200 {
    background-color: #e9d8fd;
  }

  .md\:bg-purple-300 {
    background-color: #d6bcfa;
  }

  .md\:bg-purple-400 {
    background-color: #b794f4;
  }

  .md\:bg-purple-500 {
    background-color: #9f7aea;
  }

  .md\:bg-purple-600 {
    background-color: #805ad5;
  }

  .md\:bg-purple-700 {
    background-color: #6b46c1;
  }

  .md\:bg-purple-800 {
    background-color: #553c9a;
  }

  .md\:bg-purple-900 {
    background-color: #44337a;
  }

  .md\:bg-pink-100 {
    background-color: #fff5f7;
  }

  .md\:bg-pink-200 {
    background-color: #fed7e2;
  }

  .md\:bg-pink-300 {
    background-color: #fbb6ce;
  }

  .md\:bg-pink-400 {
    background-color: #f687b3;
  }

  .md\:bg-pink-500 {
    background-color: #ed64a6;
  }

  .md\:bg-pink-600 {
    background-color: #d53f8c;
  }

  .md\:bg-pink-700 {
    background-color: #b83280;
  }

  .md\:bg-pink-800 {
    background-color: #97266d;
  }

  .md\:bg-pink-900 {
    background-color: #702459;
  }

  .md\:hover\:bg-transparent:hover {
    background-color: transparent;
  }

  .md\:hover\:bg-black:hover {
    background-color: #000;
  }

  .md\:hover\:bg-white:hover {
    background-color: #fff;
  }

  .md\:hover\:bg-gray-100:hover {
    background-color: #f7fafc;
  }

  .md\:hover\:bg-gray-200:hover {
    background-color: #edf2f7;
  }

  .md\:hover\:bg-gray-300:hover {
    background-color: #e2e8f0;
  }

  .md\:hover\:bg-gray-400:hover {
    background-color: #cbd5e0;
  }

  .md\:hover\:bg-gray-500:hover {
    background-color: #a0aec0;
  }

  .md\:hover\:bg-gray-600:hover {
    background-color: #718096;
  }

  .md\:hover\:bg-gray-700:hover {
    background-color: #4a5568;
  }

  .md\:hover\:bg-gray-800:hover {
    background-color: #2d3748;
  }

  .md\:hover\:bg-gray-900:hover {
    background-color: #1a202c;
  }

  .md\:hover\:bg-red-100:hover {
    background-color: #fff5f5;
  }

  .md\:hover\:bg-red-200:hover {
    background-color: #fed7d7;
  }

  .md\:hover\:bg-red-300:hover {
    background-color: #feb2b2;
  }

  .md\:hover\:bg-red-400:hover {
    background-color: #fc8181;
  }

  .md\:hover\:bg-red-500:hover {
    background-color: #f56565;
  }

  .md\:hover\:bg-red-600:hover {
    background-color: #e53e3e;
  }

  .md\:hover\:bg-red-700:hover {
    background-color: #c53030;
  }

  .md\:hover\:bg-red-800:hover {
    background-color: #9b2c2c;
  }

  .md\:hover\:bg-red-900:hover {
    background-color: #742a2a;
  }

  .md\:hover\:bg-orange-100:hover {
    background-color: #fffaf0;
  }

  .md\:hover\:bg-orange-200:hover {
    background-color: #feebc8;
  }

  .md\:hover\:bg-orange-300:hover {
    background-color: #fbd38d;
  }

  .md\:hover\:bg-orange-400:hover {
    background-color: #f6ad55;
  }

  .md\:hover\:bg-orange-500:hover {
    background-color: #ed8936;
  }

  .md\:hover\:bg-orange-600:hover {
    background-color: #dd6b20;
  }

  .md\:hover\:bg-orange-700:hover {
    background-color: #c05621;
  }

  .md\:hover\:bg-orange-800:hover {
    background-color: #9c4221;
  }

  .md\:hover\:bg-orange-900:hover {
    background-color: #7b341e;
  }

  .md\:hover\:bg-yellow-100:hover {
    background-color: #fffff0;
  }

  .md\:hover\:bg-yellow-200:hover {
    background-color: #fefcbf;
  }

  .md\:hover\:bg-yellow-300:hover {
    background-color: #faf089;
  }

  .md\:hover\:bg-yellow-400:hover {
    background-color: #f6e05e;
  }

  .md\:hover\:bg-yellow-500:hover {
    background-color: #ecc94b;
  }

  .md\:hover\:bg-yellow-600:hover {
    background-color: #d69e2e;
  }

  .md\:hover\:bg-yellow-700:hover {
    background-color: #b7791f;
  }

  .md\:hover\:bg-yellow-800:hover {
    background-color: #975a16;
  }

  .md\:hover\:bg-yellow-900:hover {
    background-color: #744210;
  }

  .md\:hover\:bg-green-100:hover {
    background-color: #f0fff4;
  }

  .md\:hover\:bg-green-200:hover {
    background-color: #c6f6d5;
  }

  .md\:hover\:bg-green-300:hover {
    background-color: #9ae6b4;
  }

  .md\:hover\:bg-green-400:hover {
    background-color: #68d391;
  }

  .md\:hover\:bg-green-500:hover {
    background-color: #48bb78;
  }

  .md\:hover\:bg-green-600:hover {
    background-color: #38a169;
  }

  .md\:hover\:bg-green-700:hover {
    background-color: #2f855a;
  }

  .md\:hover\:bg-green-800:hover {
    background-color: #276749;
  }

  .md\:hover\:bg-green-900:hover {
    background-color: #22543d;
  }

  .md\:hover\:bg-teal-100:hover {
    background-color: #e6fffa;
  }

  .md\:hover\:bg-teal-200:hover {
    background-color: #b2f5ea;
  }

  .md\:hover\:bg-teal-300:hover {
    background-color: #81e6d9;
  }

  .md\:hover\:bg-teal-400:hover {
    background-color: #4fd1c5;
  }

  .md\:hover\:bg-teal-500:hover {
    background-color: #38b2ac;
  }

  .md\:hover\:bg-teal-600:hover {
    background-color: #319795;
  }

  .md\:hover\:bg-teal-700:hover {
    background-color: #2c7a7b;
  }

  .md\:hover\:bg-teal-800:hover {
    background-color: #285e61;
  }

  .md\:hover\:bg-teal-900:hover {
    background-color: #234e52;
  }

  .md\:hover\:bg-blue-100:hover {
    background-color: #ebf8ff;
  }

  .md\:hover\:bg-blue-200:hover {
    background-color: #bee3f8;
  }

  .md\:hover\:bg-blue-300:hover {
    background-color: #90cdf4;
  }

  .md\:hover\:bg-blue-400:hover {
    background-color: #63b3ed;
  }

  .md\:hover\:bg-blue-500:hover {
    background-color: #4299e1;
  }

  .md\:hover\:bg-blue-600:hover {
    background-color: #3182ce;
  }

  .md\:hover\:bg-blue-700:hover {
    background-color: #2b6cb0;
  }

  .md\:hover\:bg-blue-800:hover {
    background-color: #2c5282;
  }

  .md\:hover\:bg-blue-900:hover {
    background-color: #2a4365;
  }

  .md\:hover\:bg-indigo-100:hover {
    background-color: #ebf4ff;
  }

  .md\:hover\:bg-indigo-200:hover {
    background-color: #c3dafe;
  }

  .md\:hover\:bg-indigo-300:hover {
    background-color: #a3bffa;
  }

  .md\:hover\:bg-indigo-400:hover {
    background-color: #7f9cf5;
  }

  .md\:hover\:bg-indigo-500:hover {
    background-color: #667eea;
  }

  .md\:hover\:bg-indigo-600:hover {
    background-color: #5a67d8;
  }

  .md\:hover\:bg-indigo-700:hover {
    background-color: #4c51bf;
  }

  .md\:hover\:bg-indigo-800:hover {
    background-color: #434190;
  }

  .md\:hover\:bg-indigo-900:hover {
    background-color: #3c366b;
  }

  .md\:hover\:bg-purple-100:hover {
    background-color: #faf5ff;
  }

  .md\:hover\:bg-purple-200:hover {
    background-color: #e9d8fd;
  }

  .md\:hover\:bg-purple-300:hover {
    background-color: #d6bcfa;
  }

  .md\:hover\:bg-purple-400:hover {
    background-color: #b794f4;
  }

  .md\:hover\:bg-purple-500:hover {
    background-color: #9f7aea;
  }

  .md\:hover\:bg-purple-600:hover {
    background-color: #805ad5;
  }

  .md\:hover\:bg-purple-700:hover {
    background-color: #6b46c1;
  }

  .md\:hover\:bg-purple-800:hover {
    background-color: #553c9a;
  }

  .md\:hover\:bg-purple-900:hover {
    background-color: #44337a;
  }

  .md\:hover\:bg-pink-100:hover {
    background-color: #fff5f7;
  }

  .md\:hover\:bg-pink-200:hover {
    background-color: #fed7e2;
  }

  .md\:hover\:bg-pink-300:hover {
    background-color: #fbb6ce;
  }

  .md\:hover\:bg-pink-400:hover {
    background-color: #f687b3;
  }

  .md\:hover\:bg-pink-500:hover {
    background-color: #ed64a6;
  }

  .md\:hover\:bg-pink-600:hover {
    background-color: #d53f8c;
  }

  .md\:hover\:bg-pink-700:hover {
    background-color: #b83280;
  }

  .md\:hover\:bg-pink-800:hover {
    background-color: #97266d;
  }

  .md\:hover\:bg-pink-900:hover {
    background-color: #702459;
  }

  .md\:focus\:bg-transparent:focus {
    background-color: transparent;
  }

  .md\:focus\:bg-black:focus {
    background-color: #000;
  }

  .md\:focus\:bg-white:focus {
    background-color: #fff;
  }

  .md\:focus\:bg-gray-100:focus {
    background-color: #f7fafc;
  }

  .md\:focus\:bg-gray-200:focus {
    background-color: #edf2f7;
  }

  .md\:focus\:bg-gray-300:focus {
    background-color: #e2e8f0;
  }

  .md\:focus\:bg-gray-400:focus {
    background-color: #cbd5e0;
  }

  .md\:focus\:bg-gray-500:focus {
    background-color: #a0aec0;
  }

  .md\:focus\:bg-gray-600:focus {
    background-color: #718096;
  }

  .md\:focus\:bg-gray-700:focus {
    background-color: #4a5568;
  }

  .md\:focus\:bg-gray-800:focus {
    background-color: #2d3748;
  }

  .md\:focus\:bg-gray-900:focus {
    background-color: #1a202c;
  }

  .md\:focus\:bg-red-100:focus {
    background-color: #fff5f5;
  }

  .md\:focus\:bg-red-200:focus {
    background-color: #fed7d7;
  }

  .md\:focus\:bg-red-300:focus {
    background-color: #feb2b2;
  }

  .md\:focus\:bg-red-400:focus {
    background-color: #fc8181;
  }

  .md\:focus\:bg-red-500:focus {
    background-color: #f56565;
  }

  .md\:focus\:bg-red-600:focus {
    background-color: #e53e3e;
  }

  .md\:focus\:bg-red-700:focus {
    background-color: #c53030;
  }

  .md\:focus\:bg-red-800:focus {
    background-color: #9b2c2c;
  }

  .md\:focus\:bg-red-900:focus {
    background-color: #742a2a;
  }

  .md\:focus\:bg-orange-100:focus {
    background-color: #fffaf0;
  }

  .md\:focus\:bg-orange-200:focus {
    background-color: #feebc8;
  }

  .md\:focus\:bg-orange-300:focus {
    background-color: #fbd38d;
  }

  .md\:focus\:bg-orange-400:focus {
    background-color: #f6ad55;
  }

  .md\:focus\:bg-orange-500:focus {
    background-color: #ed8936;
  }

  .md\:focus\:bg-orange-600:focus {
    background-color: #dd6b20;
  }

  .md\:focus\:bg-orange-700:focus {
    background-color: #c05621;
  }

  .md\:focus\:bg-orange-800:focus {
    background-color: #9c4221;
  }

  .md\:focus\:bg-orange-900:focus {
    background-color: #7b341e;
  }

  .md\:focus\:bg-yellow-100:focus {
    background-color: #fffff0;
  }

  .md\:focus\:bg-yellow-200:focus {
    background-color: #fefcbf;
  }

  .md\:focus\:bg-yellow-300:focus {
    background-color: #faf089;
  }

  .md\:focus\:bg-yellow-400:focus {
    background-color: #f6e05e;
  }

  .md\:focus\:bg-yellow-500:focus {
    background-color: #ecc94b;
  }

  .md\:focus\:bg-yellow-600:focus {
    background-color: #d69e2e;
  }

  .md\:focus\:bg-yellow-700:focus {
    background-color: #b7791f;
  }

  .md\:focus\:bg-yellow-800:focus {
    background-color: #975a16;
  }

  .md\:focus\:bg-yellow-900:focus {
    background-color: #744210;
  }

  .md\:focus\:bg-green-100:focus {
    background-color: #f0fff4;
  }

  .md\:focus\:bg-green-200:focus {
    background-color: #c6f6d5;
  }

  .md\:focus\:bg-green-300:focus {
    background-color: #9ae6b4;
  }

  .md\:focus\:bg-green-400:focus {
    background-color: #68d391;
  }

  .md\:focus\:bg-green-500:focus {
    background-color: #48bb78;
  }

  .md\:focus\:bg-green-600:focus {
    background-color: #38a169;
  }

  .md\:focus\:bg-green-700:focus {
    background-color: #2f855a;
  }

  .md\:focus\:bg-green-800:focus {
    background-color: #276749;
  }

  .md\:focus\:bg-green-900:focus {
    background-color: #22543d;
  }

  .md\:focus\:bg-teal-100:focus {
    background-color: #e6fffa;
  }

  .md\:focus\:bg-teal-200:focus {
    background-color: #b2f5ea;
  }

  .md\:focus\:bg-teal-300:focus {
    background-color: #81e6d9;
  }

  .md\:focus\:bg-teal-400:focus {
    background-color: #4fd1c5;
  }

  .md\:focus\:bg-teal-500:focus {
    background-color: #38b2ac;
  }

  .md\:focus\:bg-teal-600:focus {
    background-color: #319795;
  }

  .md\:focus\:bg-teal-700:focus {
    background-color: #2c7a7b;
  }

  .md\:focus\:bg-teal-800:focus {
    background-color: #285e61;
  }

  .md\:focus\:bg-teal-900:focus {
    background-color: #234e52;
  }

  .md\:focus\:bg-blue-100:focus {
    background-color: #ebf8ff;
  }

  .md\:focus\:bg-blue-200:focus {
    background-color: #bee3f8;
  }

  .md\:focus\:bg-blue-300:focus {
    background-color: #90cdf4;
  }

  .md\:focus\:bg-blue-400:focus {
    background-color: #63b3ed;
  }

  .md\:focus\:bg-blue-500:focus {
    background-color: #4299e1;
  }

  .md\:focus\:bg-blue-600:focus {
    background-color: #3182ce;
  }

  .md\:focus\:bg-blue-700:focus {
    background-color: #2b6cb0;
  }

  .md\:focus\:bg-blue-800:focus {
    background-color: #2c5282;
  }

  .md\:focus\:bg-blue-900:focus {
    background-color: #2a4365;
  }

  .md\:focus\:bg-indigo-100:focus {
    background-color: #ebf4ff;
  }

  .md\:focus\:bg-indigo-200:focus {
    background-color: #c3dafe;
  }

  .md\:focus\:bg-indigo-300:focus {
    background-color: #a3bffa;
  }

  .md\:focus\:bg-indigo-400:focus {
    background-color: #7f9cf5;
  }

  .md\:focus\:bg-indigo-500:focus {
    background-color: #667eea;
  }

  .md\:focus\:bg-indigo-600:focus {
    background-color: #5a67d8;
  }

  .md\:focus\:bg-indigo-700:focus {
    background-color: #4c51bf;
  }

  .md\:focus\:bg-indigo-800:focus {
    background-color: #434190;
  }

  .md\:focus\:bg-indigo-900:focus {
    background-color: #3c366b;
  }

  .md\:focus\:bg-purple-100:focus {
    background-color: #faf5ff;
  }

  .md\:focus\:bg-purple-200:focus {
    background-color: #e9d8fd;
  }

  .md\:focus\:bg-purple-300:focus {
    background-color: #d6bcfa;
  }

  .md\:focus\:bg-purple-400:focus {
    background-color: #b794f4;
  }

  .md\:focus\:bg-purple-500:focus {
    background-color: #9f7aea;
  }

  .md\:focus\:bg-purple-600:focus {
    background-color: #805ad5;
  }

  .md\:focus\:bg-purple-700:focus {
    background-color: #6b46c1;
  }

  .md\:focus\:bg-purple-800:focus {
    background-color: #553c9a;
  }

  .md\:focus\:bg-purple-900:focus {
    background-color: #44337a;
  }

  .md\:focus\:bg-pink-100:focus {
    background-color: #fff5f7;
  }

  .md\:focus\:bg-pink-200:focus {
    background-color: #fed7e2;
  }

  .md\:focus\:bg-pink-300:focus {
    background-color: #fbb6ce;
  }

  .md\:focus\:bg-pink-400:focus {
    background-color: #f687b3;
  }

  .md\:focus\:bg-pink-500:focus {
    background-color: #ed64a6;
  }

  .md\:focus\:bg-pink-600:focus {
    background-color: #d53f8c;
  }

  .md\:focus\:bg-pink-700:focus {
    background-color: #b83280;
  }

  .md\:focus\:bg-pink-800:focus {
    background-color: #97266d;
  }

  .md\:focus\:bg-pink-900:focus {
    background-color: #702459;
  }

  .md\:bg-bottom {
    background-position: bottom;
  }

  .md\:bg-center {
    background-position: center;
  }

  .md\:bg-left {
    background-position: left;
  }

  .md\:bg-left-bottom {
    background-position: left bottom;
  }

  .md\:bg-left-top {
    background-position: left top;
  }

  .md\:bg-right {
    background-position: right;
  }

  .md\:bg-right-bottom {
    background-position: right bottom;
  }

  .md\:bg-right-top {
    background-position: right top;
  }

  .md\:bg-top {
    background-position: top;
  }

  .md\:bg-repeat {
    background-repeat: repeat;
  }

  .md\:bg-no-repeat {
    background-repeat: no-repeat;
  }

  .md\:bg-repeat-x {
    background-repeat: repeat-x;
  }

  .md\:bg-repeat-y {
    background-repeat: repeat-y;
  }

  .md\:bg-repeat-round {
    background-repeat: round;
  }

  .md\:bg-repeat-space {
    background-repeat: space;
  }

  .md\:bg-auto {
    background-size: auto;
  }

  .md\:bg-cover {
    background-size: cover;
  }

  .md\:bg-contain {
    background-size: contain;
  }

  .md\:border-collapse {
    border-collapse: collapse;
  }

  .md\:border-separate {
    border-collapse: separate;
  }

  .md\:border-transparent {
    border-color: transparent;
  }

  .md\:border-black {
    border-color: #000;
  }

  .md\:border-white {
    border-color: #fff;
  }

  .md\:border-gray-100 {
    border-color: #f7fafc;
  }

  .md\:border-gray-200 {
    border-color: #edf2f7;
  }

  .md\:border-gray-300 {
    border-color: #e2e8f0;
  }

  .md\:border-gray-400 {
    border-color: #cbd5e0;
  }

  .md\:border-gray-500 {
    border-color: #a0aec0;
  }

  .md\:border-gray-600 {
    border-color: #718096;
  }

  .md\:border-gray-700 {
    border-color: #4a5568;
  }

  .md\:border-gray-800 {
    border-color: #2d3748;
  }

  .md\:border-gray-900 {
    border-color: #1a202c;
  }

  .md\:border-red-100 {
    border-color: #fff5f5;
  }

  .md\:border-red-200 {
    border-color: #fed7d7;
  }

  .md\:border-red-300 {
    border-color: #feb2b2;
  }

  .md\:border-red-400 {
    border-color: #fc8181;
  }

  .md\:border-red-500 {
    border-color: #f56565;
  }

  .md\:border-red-600 {
    border-color: #e53e3e;
  }

  .md\:border-red-700 {
    border-color: #c53030;
  }

  .md\:border-red-800 {
    border-color: #9b2c2c;
  }

  .md\:border-red-900 {
    border-color: #742a2a;
  }

  .md\:border-orange-100 {
    border-color: #fffaf0;
  }

  .md\:border-orange-200 {
    border-color: #feebc8;
  }

  .md\:border-orange-300 {
    border-color: #fbd38d;
  }

  .md\:border-orange-400 {
    border-color: #f6ad55;
  }

  .md\:border-orange-500 {
    border-color: #ed8936;
  }

  .md\:border-orange-600 {
    border-color: #dd6b20;
  }

  .md\:border-orange-700 {
    border-color: #c05621;
  }

  .md\:border-orange-800 {
    border-color: #9c4221;
  }

  .md\:border-orange-900 {
    border-color: #7b341e;
  }

  .md\:border-yellow-100 {
    border-color: #fffff0;
  }

  .md\:border-yellow-200 {
    border-color: #fefcbf;
  }

  .md\:border-yellow-300 {
    border-color: #faf089;
  }

  .md\:border-yellow-400 {
    border-color: #f6e05e;
  }

  .md\:border-yellow-500 {
    border-color: #ecc94b;
  }

  .md\:border-yellow-600 {
    border-color: #d69e2e;
  }

  .md\:border-yellow-700 {
    border-color: #b7791f;
  }

  .md\:border-yellow-800 {
    border-color: #975a16;
  }

  .md\:border-yellow-900 {
    border-color: #744210;
  }

  .md\:border-green-100 {
    border-color: #f0fff4;
  }

  .md\:border-green-200 {
    border-color: #c6f6d5;
  }

  .md\:border-green-300 {
    border-color: #9ae6b4;
  }

  .md\:border-green-400 {
    border-color: #68d391;
  }

  .md\:border-green-500 {
    border-color: #48bb78;
  }

  .md\:border-green-600 {
    border-color: #38a169;
  }

  .md\:border-green-700 {
    border-color: #2f855a;
  }

  .md\:border-green-800 {
    border-color: #276749;
  }

  .md\:border-green-900 {
    border-color: #22543d;
  }

  .md\:border-teal-100 {
    border-color: #e6fffa;
  }

  .md\:border-teal-200 {
    border-color: #b2f5ea;
  }

  .md\:border-teal-300 {
    border-color: #81e6d9;
  }

  .md\:border-teal-400 {
    border-color: #4fd1c5;
  }

  .md\:border-teal-500 {
    border-color: #38b2ac;
  }

  .md\:border-teal-600 {
    border-color: #319795;
  }

  .md\:border-teal-700 {
    border-color: #2c7a7b;
  }

  .md\:border-teal-800 {
    border-color: #285e61;
  }

  .md\:border-teal-900 {
    border-color: #234e52;
  }

  .md\:border-blue-100 {
    border-color: #ebf8ff;
  }

  .md\:border-blue-200 {
    border-color: #bee3f8;
  }

  .md\:border-blue-300 {
    border-color: #90cdf4;
  }

  .md\:border-blue-400 {
    border-color: #63b3ed;
  }

  .md\:border-blue-500 {
    border-color: #4299e1;
  }

  .md\:border-blue-600 {
    border-color: #3182ce;
  }

  .md\:border-blue-700 {
    border-color: #2b6cb0;
  }

  .md\:border-blue-800 {
    border-color: #2c5282;
  }

  .md\:border-blue-900 {
    border-color: #2a4365;
  }

  .md\:border-indigo-100 {
    border-color: #ebf4ff;
  }

  .md\:border-indigo-200 {
    border-color: #c3dafe;
  }

  .md\:border-indigo-300 {
    border-color: #a3bffa;
  }

  .md\:border-indigo-400 {
    border-color: #7f9cf5;
  }

  .md\:border-indigo-500 {
    border-color: #667eea;
  }

  .md\:border-indigo-600 {
    border-color: #5a67d8;
  }

  .md\:border-indigo-700 {
    border-color: #4c51bf;
  }

  .md\:border-indigo-800 {
    border-color: #434190;
  }

  .md\:border-indigo-900 {
    border-color: #3c366b;
  }

  .md\:border-purple-100 {
    border-color: #faf5ff;
  }

  .md\:border-purple-200 {
    border-color: #e9d8fd;
  }

  .md\:border-purple-300 {
    border-color: #d6bcfa;
  }

  .md\:border-purple-400 {
    border-color: #b794f4;
  }

  .md\:border-purple-500 {
    border-color: #9f7aea;
  }

  .md\:border-purple-600 {
    border-color: #805ad5;
  }

  .md\:border-purple-700 {
    border-color: #6b46c1;
  }

  .md\:border-purple-800 {
    border-color: #553c9a;
  }

  .md\:border-purple-900 {
    border-color: #44337a;
  }

  .md\:border-pink-100 {
    border-color: #fff5f7;
  }

  .md\:border-pink-200 {
    border-color: #fed7e2;
  }

  .md\:border-pink-300 {
    border-color: #fbb6ce;
  }

  .md\:border-pink-400 {
    border-color: #f687b3;
  }

  .md\:border-pink-500 {
    border-color: #ed64a6;
  }

  .md\:border-pink-600 {
    border-color: #d53f8c;
  }

  .md\:border-pink-700 {
    border-color: #b83280;
  }

  .md\:border-pink-800 {
    border-color: #97266d;
  }

  .md\:border-pink-900 {
    border-color: #702459;
  }

  .md\:hover\:border-transparent:hover {
    border-color: transparent;
  }

  .md\:hover\:border-black:hover {
    border-color: #000;
  }

  .md\:hover\:border-white:hover {
    border-color: #fff;
  }

  .md\:hover\:border-gray-100:hover {
    border-color: #f7fafc;
  }

  .md\:hover\:border-gray-200:hover {
    border-color: #edf2f7;
  }

  .md\:hover\:border-gray-300:hover {
    border-color: #e2e8f0;
  }

  .md\:hover\:border-gray-400:hover {
    border-color: #cbd5e0;
  }

  .md\:hover\:border-gray-500:hover {
    border-color: #a0aec0;
  }

  .md\:hover\:border-gray-600:hover {
    border-color: #718096;
  }

  .md\:hover\:border-gray-700:hover {
    border-color: #4a5568;
  }

  .md\:hover\:border-gray-800:hover {
    border-color: #2d3748;
  }

  .md\:hover\:border-gray-900:hover {
    border-color: #1a202c;
  }

  .md\:hover\:border-red-100:hover {
    border-color: #fff5f5;
  }

  .md\:hover\:border-red-200:hover {
    border-color: #fed7d7;
  }

  .md\:hover\:border-red-300:hover {
    border-color: #feb2b2;
  }

  .md\:hover\:border-red-400:hover {
    border-color: #fc8181;
  }

  .md\:hover\:border-red-500:hover {
    border-color: #f56565;
  }

  .md\:hover\:border-red-600:hover {
    border-color: #e53e3e;
  }

  .md\:hover\:border-red-700:hover {
    border-color: #c53030;
  }

  .md\:hover\:border-red-800:hover {
    border-color: #9b2c2c;
  }

  .md\:hover\:border-red-900:hover {
    border-color: #742a2a;
  }

  .md\:hover\:border-orange-100:hover {
    border-color: #fffaf0;
  }

  .md\:hover\:border-orange-200:hover {
    border-color: #feebc8;
  }

  .md\:hover\:border-orange-300:hover {
    border-color: #fbd38d;
  }

  .md\:hover\:border-orange-400:hover {
    border-color: #f6ad55;
  }

  .md\:hover\:border-orange-500:hover {
    border-color: #ed8936;
  }

  .md\:hover\:border-orange-600:hover {
    border-color: #dd6b20;
  }

  .md\:hover\:border-orange-700:hover {
    border-color: #c05621;
  }

  .md\:hover\:border-orange-800:hover {
    border-color: #9c4221;
  }

  .md\:hover\:border-orange-900:hover {
    border-color: #7b341e;
  }

  .md\:hover\:border-yellow-100:hover {
    border-color: #fffff0;
  }

  .md\:hover\:border-yellow-200:hover {
    border-color: #fefcbf;
  }

  .md\:hover\:border-yellow-300:hover {
    border-color: #faf089;
  }

  .md\:hover\:border-yellow-400:hover {
    border-color: #f6e05e;
  }

  .md\:hover\:border-yellow-500:hover {
    border-color: #ecc94b;
  }

  .md\:hover\:border-yellow-600:hover {
    border-color: #d69e2e;
  }

  .md\:hover\:border-yellow-700:hover {
    border-color: #b7791f;
  }

  .md\:hover\:border-yellow-800:hover {
    border-color: #975a16;
  }

  .md\:hover\:border-yellow-900:hover {
    border-color: #744210;
  }

  .md\:hover\:border-green-100:hover {
    border-color: #f0fff4;
  }

  .md\:hover\:border-green-200:hover {
    border-color: #c6f6d5;
  }

  .md\:hover\:border-green-300:hover {
    border-color: #9ae6b4;
  }

  .md\:hover\:border-green-400:hover {
    border-color: #68d391;
  }

  .md\:hover\:border-green-500:hover {
    border-color: #48bb78;
  }

  .md\:hover\:border-green-600:hover {
    border-color: #38a169;
  }

  .md\:hover\:border-green-700:hover {
    border-color: #2f855a;
  }

  .md\:hover\:border-green-800:hover {
    border-color: #276749;
  }

  .md\:hover\:border-green-900:hover {
    border-color: #22543d;
  }

  .md\:hover\:border-teal-100:hover {
    border-color: #e6fffa;
  }

  .md\:hover\:border-teal-200:hover {
    border-color: #b2f5ea;
  }

  .md\:hover\:border-teal-300:hover {
    border-color: #81e6d9;
  }

  .md\:hover\:border-teal-400:hover {
    border-color: #4fd1c5;
  }

  .md\:hover\:border-teal-500:hover {
    border-color: #38b2ac;
  }

  .md\:hover\:border-teal-600:hover {
    border-color: #319795;
  }

  .md\:hover\:border-teal-700:hover {
    border-color: #2c7a7b;
  }

  .md\:hover\:border-teal-800:hover {
    border-color: #285e61;
  }

  .md\:hover\:border-teal-900:hover {
    border-color: #234e52;
  }

  .md\:hover\:border-blue-100:hover {
    border-color: #ebf8ff;
  }

  .md\:hover\:border-blue-200:hover {
    border-color: #bee3f8;
  }

  .md\:hover\:border-blue-300:hover {
    border-color: #90cdf4;
  }

  .md\:hover\:border-blue-400:hover {
    border-color: #63b3ed;
  }

  .md\:hover\:border-blue-500:hover {
    border-color: #4299e1;
  }

  .md\:hover\:border-blue-600:hover {
    border-color: #3182ce;
  }

  .md\:hover\:border-blue-700:hover {
    border-color: #2b6cb0;
  }

  .md\:hover\:border-blue-800:hover {
    border-color: #2c5282;
  }

  .md\:hover\:border-blue-900:hover {
    border-color: #2a4365;
  }

  .md\:hover\:border-indigo-100:hover {
    border-color: #ebf4ff;
  }

  .md\:hover\:border-indigo-200:hover {
    border-color: #c3dafe;
  }

  .md\:hover\:border-indigo-300:hover {
    border-color: #a3bffa;
  }

  .md\:hover\:border-indigo-400:hover {
    border-color: #7f9cf5;
  }

  .md\:hover\:border-indigo-500:hover {
    border-color: #667eea;
  }

  .md\:hover\:border-indigo-600:hover {
    border-color: #5a67d8;
  }

  .md\:hover\:border-indigo-700:hover {
    border-color: #4c51bf;
  }

  .md\:hover\:border-indigo-800:hover {
    border-color: #434190;
  }

  .md\:hover\:border-indigo-900:hover {
    border-color: #3c366b;
  }

  .md\:hover\:border-purple-100:hover {
    border-color: #faf5ff;
  }

  .md\:hover\:border-purple-200:hover {
    border-color: #e9d8fd;
  }

  .md\:hover\:border-purple-300:hover {
    border-color: #d6bcfa;
  }

  .md\:hover\:border-purple-400:hover {
    border-color: #b794f4;
  }

  .md\:hover\:border-purple-500:hover {
    border-color: #9f7aea;
  }

  .md\:hover\:border-purple-600:hover {
    border-color: #805ad5;
  }

  .md\:hover\:border-purple-700:hover {
    border-color: #6b46c1;
  }

  .md\:hover\:border-purple-800:hover {
    border-color: #553c9a;
  }

  .md\:hover\:border-purple-900:hover {
    border-color: #44337a;
  }

  .md\:hover\:border-pink-100:hover {
    border-color: #fff5f7;
  }

  .md\:hover\:border-pink-200:hover {
    border-color: #fed7e2;
  }

  .md\:hover\:border-pink-300:hover {
    border-color: #fbb6ce;
  }

  .md\:hover\:border-pink-400:hover {
    border-color: #f687b3;
  }

  .md\:hover\:border-pink-500:hover {
    border-color: #ed64a6;
  }

  .md\:hover\:border-pink-600:hover {
    border-color: #d53f8c;
  }

  .md\:hover\:border-pink-700:hover {
    border-color: #b83280;
  }

  .md\:hover\:border-pink-800:hover {
    border-color: #97266d;
  }

  .md\:hover\:border-pink-900:hover {
    border-color: #702459;
  }

  .md\:focus\:border-transparent:focus {
    border-color: transparent;
  }

  .md\:focus\:border-black:focus {
    border-color: #000;
  }

  .md\:focus\:border-white:focus {
    border-color: #fff;
  }

  .md\:focus\:border-gray-100:focus {
    border-color: #f7fafc;
  }

  .md\:focus\:border-gray-200:focus {
    border-color: #edf2f7;
  }

  .md\:focus\:border-gray-300:focus {
    border-color: #e2e8f0;
  }

  .md\:focus\:border-gray-400:focus {
    border-color: #cbd5e0;
  }

  .md\:focus\:border-gray-500:focus {
    border-color: #a0aec0;
  }

  .md\:focus\:border-gray-600:focus {
    border-color: #718096;
  }

  .md\:focus\:border-gray-700:focus {
    border-color: #4a5568;
  }

  .md\:focus\:border-gray-800:focus {
    border-color: #2d3748;
  }

  .md\:focus\:border-gray-900:focus {
    border-color: #1a202c;
  }

  .md\:focus\:border-red-100:focus {
    border-color: #fff5f5;
  }

  .md\:focus\:border-red-200:focus {
    border-color: #fed7d7;
  }

  .md\:focus\:border-red-300:focus {
    border-color: #feb2b2;
  }

  .md\:focus\:border-red-400:focus {
    border-color: #fc8181;
  }

  .md\:focus\:border-red-500:focus {
    border-color: #f56565;
  }

  .md\:focus\:border-red-600:focus {
    border-color: #e53e3e;
  }

  .md\:focus\:border-red-700:focus {
    border-color: #c53030;
  }

  .md\:focus\:border-red-800:focus {
    border-color: #9b2c2c;
  }

  .md\:focus\:border-red-900:focus {
    border-color: #742a2a;
  }

  .md\:focus\:border-orange-100:focus {
    border-color: #fffaf0;
  }

  .md\:focus\:border-orange-200:focus {
    border-color: #feebc8;
  }

  .md\:focus\:border-orange-300:focus {
    border-color: #fbd38d;
  }

  .md\:focus\:border-orange-400:focus {
    border-color: #f6ad55;
  }

  .md\:focus\:border-orange-500:focus {
    border-color: #ed8936;
  }

  .md\:focus\:border-orange-600:focus {
    border-color: #dd6b20;
  }

  .md\:focus\:border-orange-700:focus {
    border-color: #c05621;
  }

  .md\:focus\:border-orange-800:focus {
    border-color: #9c4221;
  }

  .md\:focus\:border-orange-900:focus {
    border-color: #7b341e;
  }

  .md\:focus\:border-yellow-100:focus {
    border-color: #fffff0;
  }

  .md\:focus\:border-yellow-200:focus {
    border-color: #fefcbf;
  }

  .md\:focus\:border-yellow-300:focus {
    border-color: #faf089;
  }

  .md\:focus\:border-yellow-400:focus {
    border-color: #f6e05e;
  }

  .md\:focus\:border-yellow-500:focus {
    border-color: #ecc94b;
  }

  .md\:focus\:border-yellow-600:focus {
    border-color: #d69e2e;
  }

  .md\:focus\:border-yellow-700:focus {
    border-color: #b7791f;
  }

  .md\:focus\:border-yellow-800:focus {
    border-color: #975a16;
  }

  .md\:focus\:border-yellow-900:focus {
    border-color: #744210;
  }

  .md\:focus\:border-green-100:focus {
    border-color: #f0fff4;
  }

  .md\:focus\:border-green-200:focus {
    border-color: #c6f6d5;
  }

  .md\:focus\:border-green-300:focus {
    border-color: #9ae6b4;
  }

  .md\:focus\:border-green-400:focus {
    border-color: #68d391;
  }

  .md\:focus\:border-green-500:focus {
    border-color: #48bb78;
  }

  .md\:focus\:border-green-600:focus {
    border-color: #38a169;
  }

  .md\:focus\:border-green-700:focus {
    border-color: #2f855a;
  }

  .md\:focus\:border-green-800:focus {
    border-color: #276749;
  }

  .md\:focus\:border-green-900:focus {
    border-color: #22543d;
  }

  .md\:focus\:border-teal-100:focus {
    border-color: #e6fffa;
  }

  .md\:focus\:border-teal-200:focus {
    border-color: #b2f5ea;
  }

  .md\:focus\:border-teal-300:focus {
    border-color: #81e6d9;
  }

  .md\:focus\:border-teal-400:focus {
    border-color: #4fd1c5;
  }

  .md\:focus\:border-teal-500:focus {
    border-color: #38b2ac;
  }

  .md\:focus\:border-teal-600:focus {
    border-color: #319795;
  }

  .md\:focus\:border-teal-700:focus {
    border-color: #2c7a7b;
  }

  .md\:focus\:border-teal-800:focus {
    border-color: #285e61;
  }

  .md\:focus\:border-teal-900:focus {
    border-color: #234e52;
  }

  .md\:focus\:border-blue-100:focus {
    border-color: #ebf8ff;
  }

  .md\:focus\:border-blue-200:focus {
    border-color: #bee3f8;
  }

  .md\:focus\:border-blue-300:focus {
    border-color: #90cdf4;
  }

  .md\:focus\:border-blue-400:focus {
    border-color: #63b3ed;
  }

  .md\:focus\:border-blue-500:focus {
    border-color: #4299e1;
  }

  .md\:focus\:border-blue-600:focus {
    border-color: #3182ce;
  }

  .md\:focus\:border-blue-700:focus {
    border-color: #2b6cb0;
  }

  .md\:focus\:border-blue-800:focus {
    border-color: #2c5282;
  }

  .md\:focus\:border-blue-900:focus {
    border-color: #2a4365;
  }

  .md\:focus\:border-indigo-100:focus {
    border-color: #ebf4ff;
  }

  .md\:focus\:border-indigo-200:focus {
    border-color: #c3dafe;
  }

  .md\:focus\:border-indigo-300:focus {
    border-color: #a3bffa;
  }

  .md\:focus\:border-indigo-400:focus {
    border-color: #7f9cf5;
  }

  .md\:focus\:border-indigo-500:focus {
    border-color: #667eea;
  }

  .md\:focus\:border-indigo-600:focus {
    border-color: #5a67d8;
  }

  .md\:focus\:border-indigo-700:focus {
    border-color: #4c51bf;
  }

  .md\:focus\:border-indigo-800:focus {
    border-color: #434190;
  }

  .md\:focus\:border-indigo-900:focus {
    border-color: #3c366b;
  }

  .md\:focus\:border-purple-100:focus {
    border-color: #faf5ff;
  }

  .md\:focus\:border-purple-200:focus {
    border-color: #e9d8fd;
  }

  .md\:focus\:border-purple-300:focus {
    border-color: #d6bcfa;
  }

  .md\:focus\:border-purple-400:focus {
    border-color: #b794f4;
  }

  .md\:focus\:border-purple-500:focus {
    border-color: #9f7aea;
  }

  .md\:focus\:border-purple-600:focus {
    border-color: #805ad5;
  }

  .md\:focus\:border-purple-700:focus {
    border-color: #6b46c1;
  }

  .md\:focus\:border-purple-800:focus {
    border-color: #553c9a;
  }

  .md\:focus\:border-purple-900:focus {
    border-color: #44337a;
  }

  .md\:focus\:border-pink-100:focus {
    border-color: #fff5f7;
  }

  .md\:focus\:border-pink-200:focus {
    border-color: #fed7e2;
  }

  .md\:focus\:border-pink-300:focus {
    border-color: #fbb6ce;
  }

  .md\:focus\:border-pink-400:focus {
    border-color: #f687b3;
  }

  .md\:focus\:border-pink-500:focus {
    border-color: #ed64a6;
  }

  .md\:focus\:border-pink-600:focus {
    border-color: #d53f8c;
  }

  .md\:focus\:border-pink-700:focus {
    border-color: #b83280;
  }

  .md\:focus\:border-pink-800:focus {
    border-color: #97266d;
  }

  .md\:focus\:border-pink-900:focus {
    border-color: #702459;
  }

  .md\:rounded-none {
    border-radius: 0;
  }

  .md\:rounded-sm {
    border-radius: 0.125rem;
  }

  .md\:rounded {
    border-radius: 0.25rem;
  }

  .md\:rounded-lg {
    border-radius: 0.5rem;
  }

  .md\:rounded-full {
    border-radius: 9999px;
  }

  .md\:rounded-t-none {
    border-top-left-radius: 0;
    border-top-right-radius: 0;
  }

  .md\:rounded-r-none {
    border-top-right-radius: 0;
    border-bottom-right-radius: 0;
  }

  .md\:rounded-b-none {
    border-bottom-right-radius: 0;
    border-bottom-left-radius: 0;
  }

  .md\:rounded-l-none {
    border-top-left-radius: 0;
    border-bottom-left-radius: 0;
  }

  .md\:rounded-t-sm {
    border-top-left-radius: 0.125rem;
    border-top-right-radius: 0.125rem;
  }

  .md\:rounded-r-sm {
    border-top-right-radius: 0.125rem;
    border-bottom-right-radius: 0.125rem;
  }

  .md\:rounded-b-sm {
    border-bottom-right-radius: 0.125rem;
    border-bottom-left-radius: 0.125rem;
  }

  .md\:rounded-l-sm {
    border-top-left-radius: 0.125rem;
    border-bottom-left-radius: 0.125rem;
  }

  .md\:rounded-t {
    border-top-left-radius: 0.25rem;
    border-top-right-radius: 0.25rem;
  }

  .md\:rounded-r {
    border-top-right-radius: 0.25rem;
    border-bottom-right-radius: 0.25rem;
  }

  .md\:rounded-b {
    border-bottom-right-radius: 0.25rem;
    border-bottom-left-radius: 0.25rem;
  }

  .md\:rounded-l {
    border-top-left-radius: 0.25rem;
    border-bottom-left-radius: 0.25rem;
  }

  .md\:rounded-t-lg {
    border-top-left-radius: 0.5rem;
    border-top-right-radius: 0.5rem;
  }

  .md\:rounded-r-lg {
    border-top-right-radius: 0.5rem;
    border-bottom-right-radius: 0.5rem;
  }

  .md\:rounded-b-lg {
    border-bottom-right-radius: 0.5rem;
    border-bottom-left-radius: 0.5rem;
  }

  .md\:rounded-l-lg {
    border-top-left-radius: 0.5rem;
    border-bottom-left-radius: 0.5rem;
  }

  .md\:rounded-t-full {
    border-top-left-radius: 9999px;
    border-top-right-radius: 9999px;
  }

  .md\:rounded-r-full {
    border-top-right-radius: 9999px;
    border-bottom-right-radius: 9999px;
  }

  .md\:rounded-b-full {
    border-bottom-right-radius: 9999px;
    border-bottom-left-radius: 9999px;
  }

  .md\:rounded-l-full {
    border-top-left-radius: 9999px;
    border-bottom-left-radius: 9999px;
  }

  .md\:rounded-tl-none {
    border-top-left-radius: 0;
  }

  .md\:rounded-tr-none {
    border-top-right-radius: 0;
  }

  .md\:rounded-br-none {
    border-bottom-right-radius: 0;
  }

  .md\:rounded-bl-none {
    border-bottom-left-radius: 0;
  }

  .md\:rounded-tl-sm {
    border-top-left-radius: 0.125rem;
  }

  .md\:rounded-tr-sm {
    border-top-right-radius: 0.125rem;
  }

  .md\:rounded-br-sm {
    border-bottom-right-radius: 0.125rem;
  }

  .md\:rounded-bl-sm {
    border-bottom-left-radius: 0.125rem;
  }

  .md\:rounded-tl {
    border-top-left-radius: 0.25rem;
  }

  .md\:rounded-tr {
    border-top-right-radius: 0.25rem;
  }

  .md\:rounded-br {
    border-bottom-right-radius: 0.25rem;
  }

  .md\:rounded-bl {
    border-bottom-left-radius: 0.25rem;
  }

  .md\:rounded-tl-lg {
    border-top-left-radius: 0.5rem;
  }

  .md\:rounded-tr-lg {
    border-top-right-radius: 0.5rem;
  }

  .md\:rounded-br-lg {
    border-bottom-right-radius: 0.5rem;
  }

  .md\:rounded-bl-lg {
    border-bottom-left-radius: 0.5rem;
  }

  .md\:rounded-tl-full {
    border-top-left-radius: 9999px;
  }

  .md\:rounded-tr-full {
    border-top-right-radius: 9999px;
  }

  .md\:rounded-br-full {
    border-bottom-right-radius: 9999px;
  }

  .md\:rounded-bl-full {
    border-bottom-left-radius: 9999px;
  }

  .md\:border-solid {
    border-style: solid;
  }

  .md\:border-dashed {
    border-style: dashed;
  }

  .md\:border-dotted {
    border-style: dotted;
  }

  .md\:border-double {
    border-style: double;
  }

  .md\:border-none {
    border-style: none;
  }

  .md\:border-0 {
    border-width: 0;
  }

  .md\:border-2 {
    border-width: 2px;
  }

  .md\:border-4 {
    border-width: 4px;
  }

  .md\:border-8 {
    border-width: 8px;
  }

  .md\:border {
    border-width: 1px;
  }

  .md\:border-t-0 {
    border-top-width: 0;
  }

  .md\:border-r-0 {
    border-right-width: 0;
  }

  .md\:border-b-0 {
    border-bottom-width: 0;
  }

  .md\:border-l-0 {
    border-left-width: 0;
  }

  .md\:border-t-2 {
    border-top-width: 2px;
  }

  .md\:border-r-2 {
    border-right-width: 2px;
  }

  .md\:border-b-2 {
    border-bottom-width: 2px;
  }

  .md\:border-l-2 {
    border-left-width: 2px;
  }

  .md\:border-t-4 {
    border-top-width: 4px;
  }

  .md\:border-r-4 {
    border-right-width: 4px;
  }

  .md\:border-b-4 {
    border-bottom-width: 4px;
  }

  .md\:border-l-4 {
    border-left-width: 4px;
  }

  .md\:border-t-8 {
    border-top-width: 8px;
  }

  .md\:border-r-8 {
    border-right-width: 8px;
  }

  .md\:border-b-8 {
    border-bottom-width: 8px;
  }

  .md\:border-l-8 {
    border-left-width: 8px;
  }

  .md\:border-t {
    border-top-width: 1px;
  }

  .md\:border-r {
    border-right-width: 1px;
  }

  .md\:border-b {
    border-bottom-width: 1px;
  }

  .md\:border-l {
    border-left-width: 1px;
  }

  .md\:cursor-auto {
    cursor: auto;
  }

  .md\:cursor-default {
    cursor: default;
  }

  .md\:cursor-pointer {
    cursor: pointer;
  }

  .md\:cursor-wait {
    cursor: wait;
  }

  .md\:cursor-text {
    cursor: text;
  }

  .md\:cursor-move {
    cursor: move;
  }

  .md\:cursor-not-allowed {
    cursor: not-allowed;
  }

  .md\:block {
    display: block;
  }

  .md\:inline-block {
    display: inline-block;
  }

  .md\:inline {
    display: inline;
  }

  .md\:flex {
    display: -webkit-box;
    display: flex;
  }

  .md\:inline-flex {
    display: -webkit-inline-box;
    display: inline-flex;
  }

  .md\:table {
    display: table;
  }

  .md\:table-row {
    display: table-row;
  }

  .md\:table-cell {
    display: table-cell;
  }

  .md\:hidden {
    display: none;
  }

  .md\:flex-row {
    -webkit-box-orient: horizontal;
    -webkit-box-direction: normal;
            flex-direction: row;
  }

  .md\:flex-row-reverse {
    -webkit-box-orient: horizontal;
    -webkit-box-direction: reverse;
            flex-direction: row-reverse;
  }

  .md\:flex-col {
    -webkit-box-orient: vertical;
    -webkit-box-direction: normal;
            flex-direction: column;
  }

  .md\:flex-col-reverse {
    -webkit-box-orient: vertical;
    -webkit-box-direction: reverse;
            flex-direction: column-reverse;
  }

  .md\:flex-wrap {
    flex-wrap: wrap;
  }

  .md\:flex-wrap-reverse {
    flex-wrap: wrap-reverse;
  }

  .md\:flex-no-wrap {
    flex-wrap: nowrap;
  }

  .md\:items-start {
    -webkit-box-align: start;
            align-items: flex-start;
  }

  .md\:items-end {
    -webkit-box-align: end;
            align-items: flex-end;
  }

  .md\:items-center {
    -webkit-box-align: center;
            align-items: center;
  }

  .md\:items-baseline {
    -webkit-box-align: baseline;
            align-items: baseline;
  }

  .md\:items-stretch {
    -webkit-box-align: stretch;
            align-items: stretch;
  }

  .md\:self-auto {
    align-self: auto;
  }

  .md\:self-start {
    align-self: flex-start;
  }

  .md\:self-end {
    align-self: flex-end;
  }

  .md\:self-center {
    align-self: center;
  }

  .md\:self-stretch {
    align-self: stretch;
  }

  .md\:justify-start {
    -webkit-box-pack: start;
            justify-content: flex-start;
  }

  .md\:justify-end {
    -webkit-box-pack: end;
            justify-content: flex-end;
  }

  .md\:justify-center {
    -webkit-box-pack: center;
            justify-content: center;
  }

  .md\:justify-between {
    -webkit-box-pack: justify;
            justify-content: space-between;
  }

  .md\:justify-around {
    justify-content: space-around;
  }

  .md\:content-center {
    align-content: center;
  }

  .md\:content-start {
    align-content: flex-start;
  }

  .md\:content-end {
    align-content: flex-end;
  }

  .md\:content-between {
    align-content: space-between;
  }

  .md\:content-around {
    align-content: space-around;
  }

  .md\:flex-1 {
    -webkit-box-flex: 1;
            flex: 1 1 0%;
  }

  .md\:flex-auto {
    -webkit-box-flex: 1;
            flex: 1 1 auto;
  }

  .md\:flex-initial {
    -webkit-box-flex: 0;
            flex: 0 1 auto;
  }

  .md\:flex-none {
    -webkit-box-flex: 0;
            flex: none;
  }

  .md\:flex-grow-0 {
    -webkit-box-flex: 0;
            flex-grow: 0;
  }

  .md\:flex-grow {
    -webkit-box-flex: 1;
            flex-grow: 1;
  }

  .md\:flex-shrink-0 {
    flex-shrink: 0;
  }

  .md\:flex-shrink {
    flex-shrink: 1;
  }

  .md\:order-1 {
    -webkit-box-ordinal-group: 2;
            order: 1;
  }

  .md\:order-2 {
    -webkit-box-ordinal-group: 3;
            order: 2;
  }

  .md\:order-3 {
    -webkit-box-ordinal-group: 4;
            order: 3;
  }

  .md\:order-4 {
    -webkit-box-ordinal-group: 5;
            order: 4;
  }

  .md\:order-5 {
    -webkit-box-ordinal-group: 6;
            order: 5;
  }

  .md\:order-6 {
    -webkit-box-ordinal-group: 7;
            order: 6;
  }

  .md\:order-7 {
    -webkit-box-ordinal-group: 8;
            order: 7;
  }

  .md\:order-8 {
    -webkit-box-ordinal-group: 9;
            order: 8;
  }

  .md\:order-9 {
    -webkit-box-ordinal-group: 10;
            order: 9;
  }

  .md\:order-10 {
    -webkit-box-ordinal-group: 11;
            order: 10;
  }

  .md\:order-11 {
    -webkit-box-ordinal-group: 12;
            order: 11;
  }

  .md\:order-12 {
    -webkit-box-ordinal-group: 13;
            order: 12;
  }

  .md\:order-first {
    -webkit-box-ordinal-group: -9998;
            order: -9999;
  }

  .md\:order-last {
    -webkit-box-ordinal-group: 10000;
            order: 9999;
  }

  .md\:order-none {
    -webkit-box-ordinal-group: 1;
            order: 0;
  }

  .md\:float-right {
    float: right;
  }

  .md\:float-left {
    float: left;
  }

  .md\:float-none {
    float: none;
  }

  .md\:clearfix:after {
    content: "";
    display: table;
    clear: both;
  }

  .md\:font-sans {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji";
  }

  .md\:font-serif {
    font-family: Georgia, Cambria, "Times New Roman", Times, serif;
  }

  .md\:font-mono {
    font-family: Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  }

  .md\:font-hairline {
    font-weight: 100;
  }

  .md\:font-thin {
    font-weight: 200;
  }

  .md\:font-light {
    font-weight: 300;
  }

  .md\:font-normal {
    font-weight: 400;
  }

  .md\:font-medium {
    font-weight: 500;
  }

  .md\:font-semibold {
    font-weight: 600;
  }

  .md\:font-bold {
    font-weight: 700;
  }

  .md\:font-extrabold {
    font-weight: 800;
  }

  .md\:font-black {
    font-weight: 900;
  }

  .md\:hover\:font-hairline:hover {
    font-weight: 100;
  }

  .md\:hover\:font-thin:hover {
    font-weight: 200;
  }

  .md\:hover\:font-light:hover {
    font-weight: 300;
  }

  .md\:hover\:font-normal:hover {
    font-weight: 400;
  }

  .md\:hover\:font-medium:hover {
    font-weight: 500;
  }

  .md\:hover\:font-semibold:hover {
    font-weight: 600;
  }

  .md\:hover\:font-bold:hover {
    font-weight: 700;
  }

  .md\:hover\:font-extrabold:hover {
    font-weight: 800;
  }

  .md\:hover\:font-black:hover {
    font-weight: 900;
  }

  .md\:focus\:font-hairline:focus {
    font-weight: 100;
  }

  .md\:focus\:font-thin:focus {
    font-weight: 200;
  }

  .md\:focus\:font-light:focus {
    font-weight: 300;
  }

  .md\:focus\:font-normal:focus {
    font-weight: 400;
  }

  .md\:focus\:font-medium:focus {
    font-weight: 500;
  }

  .md\:focus\:font-semibold:focus {
    font-weight: 600;
  }

  .md\:focus\:font-bold:focus {
    font-weight: 700;
  }

  .md\:focus\:font-extrabold:focus {
    font-weight: 800;
  }

  .md\:focus\:font-black:focus {
    font-weight: 900;
  }

  .md\:h-0 {
    height: 0;
  }

  .md\:h-1 {
    height: 0.25rem;
  }

  .md\:h-2 {
    height: 0.5rem;
  }

  .md\:h-3 {
    height: 0.75rem;
  }

  .md\:h-4 {
    height: 1rem;
  }

  .md\:h-5 {
    height: 1.25rem;
  }

  .md\:h-6 {
    height: 1.5rem;
  }

  .md\:h-8 {
    height: 2rem;
  }

  .md\:h-10 {
    height: 2.5rem;
  }

  .md\:h-12 {
    height: 3rem;
  }

  .md\:h-16 {
    height: 4rem;
  }

  .md\:h-20 {
    height: 5rem;
  }

  .md\:h-24 {
    height: 6rem;
  }

  .md\:h-32 {
    height: 8rem;
  }

  .md\:h-40 {
    height: 10rem;
  }

  .md\:h-48 {
    height: 12rem;
  }

  .md\:h-56 {
    height: 14rem;
  }

  .md\:h-64 {
    height: 16rem;
  }

  .md\:h-auto {
    height: auto;
  }

  .md\:h-px {
    height: 1px;
  }

  .md\:h-full {
    height: 100%;
  }

  .md\:h-screen {
    height: 100vh;
  }

  .md\:leading-none {
    line-height: 1;
  }

  .md\:leading-tight {
    line-height: 1.25;
  }

  .md\:leading-snug {
    line-height: 1.375;
  }

  .md\:leading-normal {
    line-height: 1.5;
  }

  .md\:leading-relaxed {
    line-height: 1.625;
  }

  .md\:leading-loose {
    line-height: 2;
  }

  .md\:list-inside {
    list-style-position: inside;
  }

  .md\:list-outside {
    list-style-position: outside;
  }

  .md\:list-none {
    list-style-type: none;
  }

  .md\:list-disc {
    list-style-type: disc;
  }

  .md\:list-decimal {
    list-style-type: decimal;
  }

  .md\:m-0 {
    margin: 0;
  }

  .md\:m-1 {
    margin: 0.25rem;
  }

  .md\:m-2 {
    margin: 0.5rem;
  }

  .md\:m-3 {
    margin: 0.75rem;
  }

  .md\:m-4 {
    margin: 1rem;
  }

  .md\:m-5 {
    margin: 1.25rem;
  }

  .md\:m-6 {
    margin: 1.5rem;
  }

  .md\:m-8 {
    margin: 2rem;
  }

  .md\:m-10 {
    margin: 2.5rem;
  }

  .md\:m-12 {
    margin: 3rem;
  }

  .md\:m-16 {
    margin: 4rem;
  }

  .md\:m-20 {
    margin: 5rem;
  }

  .md\:m-24 {
    margin: 6rem;
  }

  .md\:m-32 {
    margin: 8rem;
  }

  .md\:m-40 {
    margin: 10rem;
  }

  .md\:m-48 {
    margin: 12rem;
  }

  .md\:m-56 {
    margin: 14rem;
  }

  .md\:m-64 {
    margin: 16rem;
  }

  .md\:m-auto {
    margin: auto;
  }

  .md\:m-px {
    margin: 1px;
  }

  .md\:-m-1 {
    margin: -0.25rem;
  }

  .md\:-m-2 {
    margin: -0.5rem;
  }

  .md\:-m-3 {
    margin: -0.75rem;
  }

  .md\:-m-4 {
    margin: -1rem;
  }

  .md\:-m-5 {
    margin: -1.25rem;
  }

  .md\:-m-6 {
    margin: -1.5rem;
  }

  .md\:-m-8 {
    margin: -2rem;
  }

  .md\:-m-10 {
    margin: -2.5rem;
  }

  .md\:-m-12 {
    margin: -3rem;
  }

  .md\:-m-16 {
    margin: -4rem;
  }

  .md\:-m-20 {
    margin: -5rem;
  }

  .md\:-m-24 {
    margin: -6rem;
  }

  .md\:-m-32 {
    margin: -8rem;
  }

  .md\:-m-40 {
    margin: -10rem;
  }

  .md\:-m-48 {
    margin: -12rem;
  }

  .md\:-m-56 {
    margin: -14rem;
  }

  .md\:-m-64 {
    margin: -16rem;
  }

  .md\:-m-px {
    margin: -1px;
  }

  .md\:my-0 {
    margin-top: 0;
    margin-bottom: 0;
  }

  .md\:mx-0 {
    margin-left: 0;
    margin-right: 0;
  }

  .md\:my-1 {
    margin-top: 0.25rem;
    margin-bottom: 0.25rem;
  }

  .md\:mx-1 {
    margin-left: 0.25rem;
    margin-right: 0.25rem;
  }

  .md\:my-2 {
    margin-top: 0.5rem;
    margin-bottom: 0.5rem;
  }

  .md\:mx-2 {
    margin-left: 0.5rem;
    margin-right: 0.5rem;
  }

  .md\:my-3 {
    margin-top: 0.75rem;
    margin-bottom: 0.75rem;
  }

  .md\:mx-3 {
    margin-left: 0.75rem;
    margin-right: 0.75rem;
  }

  .md\:my-4 {
    margin-top: 1rem;
    margin-bottom: 1rem;
  }

  .md\:mx-4 {
    margin-left: 1rem;
    margin-right: 1rem;
  }

  .md\:my-5 {
    margin-top: 1.25rem;
    margin-bottom: 1.25rem;
  }

  .md\:mx-5 {
    margin-left: 1.25rem;
    margin-right: 1.25rem;
  }

  .md\:my-6 {
    margin-top: 1.5rem;
    margin-bottom: 1.5rem;
  }

  .md\:mx-6 {
    margin-left: 1.5rem;
    margin-right: 1.5rem;
  }

  .md\:my-8 {
    margin-top: 2rem;
    margin-bottom: 2rem;
  }

  .md\:mx-8 {
    margin-left: 2rem;
    margin-right: 2rem;
  }

  .md\:my-10 {
    margin-top: 2.5rem;
    margin-bottom: 2.5rem;
  }

  .md\:mx-10 {
    margin-left: 2.5rem;
    margin-right: 2.5rem;
  }

  .md\:my-12 {
    margin-top: 3rem;
    margin-bottom: 3rem;
  }

  .md\:mx-12 {
    margin-left: 3rem;
    margin-right: 3rem;
  }

  .md\:my-16 {
    margin-top: 4rem;
    margin-bottom: 4rem;
  }

  .md\:mx-16 {
    margin-left: 4rem;
    margin-right: 4rem;
  }

  .md\:my-20 {
    margin-top: 5rem;
    margin-bottom: 5rem;
  }

  .md\:mx-20 {
    margin-left: 5rem;
    margin-right: 5rem;
  }

  .md\:my-24 {
    margin-top: 6rem;
    margin-bottom: 6rem;
  }

  .md\:mx-24 {
    margin-left: 6rem;
    margin-right: 6rem;
  }

  .md\:my-32 {
    margin-top: 8rem;
    margin-bottom: 8rem;
  }

  .md\:mx-32 {
    margin-left: 8rem;
    margin-right: 8rem;
  }

  .md\:my-40 {
    margin-top: 10rem;
    margin-bottom: 10rem;
  }

  .md\:mx-40 {
    margin-left: 10rem;
    margin-right: 10rem;
  }

  .md\:my-48 {
    margin-top: 12rem;
    margin-bottom: 12rem;
  }

  .md\:mx-48 {
    margin-left: 12rem;
    margin-right: 12rem;
  }

  .md\:my-56 {
    margin-top: 14rem;
    margin-bottom: 14rem;
  }

  .md\:mx-56 {
    margin-left: 14rem;
    margin-right: 14rem;
  }

  .md\:my-64 {
    margin-top: 16rem;
    margin-bottom: 16rem;
  }

  .md\:mx-64 {
    margin-left: 16rem;
    margin-right: 16rem;
  }

  .md\:my-auto {
    margin-top: auto;
    margin-bottom: auto;
  }

  .md\:mx-auto {
    margin-left: auto;
    margin-right: auto;
  }

  .md\:my-px {
    margin-top: 1px;
    margin-bottom: 1px;
  }

  .md\:mx-px {
    margin-left: 1px;
    margin-right: 1px;
  }

  .md\:-my-1 {
    margin-top: -0.25rem;
    margin-bottom: -0.25rem;
  }

  .md\:-mx-1 {
    margin-left: -0.25rem;
    margin-right: -0.25rem;
  }

  .md\:-my-2 {
    margin-top: -0.5rem;
    margin-bottom: -0.5rem;
  }

  .md\:-mx-2 {
    margin-left: -0.5rem;
    margin-right: -0.5rem;
  }

  .md\:-my-3 {
    margin-top: -0.75rem;
    margin-bottom: -0.75rem;
  }

  .md\:-mx-3 {
    margin-left: -0.75rem;
    margin-right: -0.75rem;
  }

  .md\:-my-4 {
    margin-top: -1rem;
    margin-bottom: -1rem;
  }

  .md\:-mx-4 {
    margin-left: -1rem;
    margin-right: -1rem;
  }

  .md\:-my-5 {
    margin-top: -1.25rem;
    margin-bottom: -1.25rem;
  }

  .md\:-mx-5 {
    margin-left: -1.25rem;
    margin-right: -1.25rem;
  }

  .md\:-my-6 {
    margin-top: -1.5rem;
    margin-bottom: -1.5rem;
  }

  .md\:-mx-6 {
    margin-left: -1.5rem;
    margin-right: -1.5rem;
  }

  .md\:-my-8 {
    margin-top: -2rem;
    margin-bottom: -2rem;
  }

  .md\:-mx-8 {
    margin-left: -2rem;
    margin-right: -2rem;
  }

  .md\:-my-10 {
    margin-top: -2.5rem;
    margin-bottom: -2.5rem;
  }

  .md\:-mx-10 {
    margin-left: -2.5rem;
    margin-right: -2.5rem;
  }

  .md\:-my-12 {
    margin-top: -3rem;
    margin-bottom: -3rem;
  }

  .md\:-mx-12 {
    margin-left: -3rem;
    margin-right: -3rem;
  }

  .md\:-my-16 {
    margin-top: -4rem;
    margin-bottom: -4rem;
  }

  .md\:-mx-16 {
    margin-left: -4rem;
    margin-right: -4rem;
  }

  .md\:-my-20 {
    margin-top: -5rem;
    margin-bottom: -5rem;
  }

  .md\:-mx-20 {
    margin-left: -5rem;
    margin-right: -5rem;
  }

  .md\:-my-24 {
    margin-top: -6rem;
    margin-bottom: -6rem;
  }

  .md\:-mx-24 {
    margin-left: -6rem;
    margin-right: -6rem;
  }

  .md\:-my-32 {
    margin-top: -8rem;
    margin-bottom: -8rem;
  }

  .md\:-mx-32 {
    margin-left: -8rem;
    margin-right: -8rem;
  }

  .md\:-my-40 {
    margin-top: -10rem;
    margin-bottom: -10rem;
  }

  .md\:-mx-40 {
    margin-left: -10rem;
    margin-right: -10rem;
  }

  .md\:-my-48 {
    margin-top: -12rem;
    margin-bottom: -12rem;
  }

  .md\:-mx-48 {
    margin-left: -12rem;
    margin-right: -12rem;
  }

  .md\:-my-56 {
    margin-top: -14rem;
    margin-bottom: -14rem;
  }

  .md\:-mx-56 {
    margin-left: -14rem;
    margin-right: -14rem;
  }

  .md\:-my-64 {
    margin-top: -16rem;
    margin-bottom: -16rem;
  }

  .md\:-mx-64 {
    margin-left: -16rem;
    margin-right: -16rem;
  }

  .md\:-my-px {
    margin-top: -1px;
    margin-bottom: -1px;
  }

  .md\:-mx-px {
    margin-left: -1px;
    margin-right: -1px;
  }

  .md\:mt-0 {
    margin-top: 0;
  }

  .md\:mr-0 {
    margin-right: 0;
  }

  .md\:mb-0 {
    margin-bottom: 0;
  }

  .md\:ml-0 {
    margin-left: 0;
  }

  .md\:mt-1 {
    margin-top: 0.25rem;
  }

  .md\:mr-1 {
    margin-right: 0.25rem;
  }

  .md\:mb-1 {
    margin-bottom: 0.25rem;
  }

  .md\:ml-1 {
    margin-left: 0.25rem;
  }

  .md\:mt-2 {
    margin-top: 0.5rem;
  }

  .md\:mr-2 {
    margin-right: 0.5rem;
  }

  .md\:mb-2 {
    margin-bottom: 0.5rem;
  }

  .md\:ml-2 {
    margin-left: 0.5rem;
  }

  .md\:mt-3 {
    margin-top: 0.75rem;
  }

  .md\:mr-3 {
    margin-right: 0.75rem;
  }

  .md\:mb-3 {
    margin-bottom: 0.75rem;
  }

  .md\:ml-3 {
    margin-left: 0.75rem;
  }

  .md\:mt-4 {
    margin-top: 1rem;
  }

  .md\:mr-4 {
    margin-right: 1rem;
  }

  .md\:mb-4 {
    margin-bottom: 1rem;
  }

  .md\:ml-4 {
    margin-left: 1rem;
  }

  .md\:mt-5 {
    margin-top: 1.25rem;
  }

  .md\:mr-5 {
    margin-right: 1.25rem;
  }

  .md\:mb-5 {
    margin-bottom: 1.25rem;
  }

  .md\:ml-5 {
    margin-left: 1.25rem;
  }

  .md\:mt-6 {
    margin-top: 1.5rem;
  }

  .md\:mr-6 {
    margin-right: 1.5rem;
  }

  .md\:mb-6 {
    margin-bottom: 1.5rem;
  }

  .md\:ml-6 {
    margin-left: 1.5rem;
  }

  .md\:mt-8 {
    margin-top: 2rem;
  }

  .md\:mr-8 {
    margin-right: 2rem;
  }

  .md\:mb-8 {
    margin-bottom: 2rem;
  }

  .md\:ml-8 {
    margin-left: 2rem;
  }

  .md\:mt-10 {
    margin-top: 2.5rem;
  }

  .md\:mr-10 {
    margin-right: 2.5rem;
  }

  .md\:mb-10 {
    margin-bottom: 2.5rem;
  }

  .md\:ml-10 {
    margin-left: 2.5rem;
  }

  .md\:mt-12 {
    margin-top: 3rem;
  }

  .md\:mr-12 {
    margin-right: 3rem;
  }

  .md\:mb-12 {
    margin-bottom: 3rem;
  }

  .md\:ml-12 {
    margin-left: 3rem;
  }

  .md\:mt-16 {
    margin-top: 4rem;
  }

  .md\:mr-16 {
    margin-right: 4rem;
  }

  .md\:mb-16 {
    margin-bottom: 4rem;
  }

  .md\:ml-16 {
    margin-left: 4rem;
  }

  .md\:mt-20 {
    margin-top: 5rem;
  }

  .md\:mr-20 {
    margin-right: 5rem;
  }

  .md\:mb-20 {
    margin-bottom: 5rem;
  }

  .md\:ml-20 {
    margin-left: 5rem;
  }

  .md\:mt-24 {
    margin-top: 6rem;
  }

  .md\:mr-24 {
    margin-right: 6rem;
  }

  .md\:mb-24 {
    margin-bottom: 6rem;
  }

  .md\:ml-24 {
    margin-left: 6rem;
  }

  .md\:mt-32 {
    margin-top: 8rem;
  }

  .md\:mr-32 {
    margin-right: 8rem;
  }

  .md\:mb-32 {
    margin-bottom: 8rem;
  }

  .md\:ml-32 {
    margin-left: 8rem;
  }

  .md\:mt-40 {
    margin-top: 10rem;
  }

  .md\:mr-40 {
    margin-right: 10rem;
  }

  .md\:mb-40 {
    margin-bottom: 10rem;
  }

  .md\:ml-40 {
    margin-left: 10rem;
  }

  .md\:mt-48 {
    margin-top: 12rem;
  }

  .md\:mr-48 {
    margin-right: 12rem;
  }

  .md\:mb-48 {
    margin-bottom: 12rem;
  }

  .md\:ml-48 {
    margin-left: 12rem;
  }

  .md\:mt-56 {
    margin-top: 14rem;
  }

  .md\:mr-56 {
    margin-right: 14rem;
  }

  .md\:mb-56 {
    margin-bottom: 14rem;
  }

  .md\:ml-56 {
    margin-left: 14rem;
  }

  .md\:mt-64 {
    margin-top: 16rem;
  }

  .md\:mr-64 {
    margin-right: 16rem;
  }

  .md\:mb-64 {
    margin-bottom: 16rem;
  }

  .md\:ml-64 {
    margin-left: 16rem;
  }

  .md\:mt-auto {
    margin-top: auto;
  }

  .md\:mr-auto {
    margin-right: auto;
  }

  .md\:mb-auto {
    margin-bottom: auto;
  }

  .md\:ml-auto {
    margin-left: auto;
  }

  .md\:mt-px {
    margin-top: 1px;
  }

  .md\:mr-px {
    margin-right: 1px;
  }

  .md\:mb-px {
    margin-bottom: 1px;
  }

  .md\:ml-px {
    margin-left: 1px;
  }

  .md\:-mt-1 {
    margin-top: -0.25rem;
  }

  .md\:-mr-1 {
    margin-right: -0.25rem;
  }

  .md\:-mb-1 {
    margin-bottom: -0.25rem;
  }

  .md\:-ml-1 {
    margin-left: -0.25rem;
  }

  .md\:-mt-2 {
    margin-top: -0.5rem;
  }

  .md\:-mr-2 {
    margin-right: -0.5rem;
  }

  .md\:-mb-2 {
    margin-bottom: -0.5rem;
  }

  .md\:-ml-2 {
    margin-left: -0.5rem;
  }

  .md\:-mt-3 {
    margin-top: -0.75rem;
  }

  .md\:-mr-3 {
    margin-right: -0.75rem;
  }

  .md\:-mb-3 {
    margin-bottom: -0.75rem;
  }

  .md\:-ml-3 {
    margin-left: -0.75rem;
  }

  .md\:-mt-4 {
    margin-top: -1rem;
  }

  .md\:-mr-4 {
    margin-right: -1rem;
  }

  .md\:-mb-4 {
    margin-bottom: -1rem;
  }

  .md\:-ml-4 {
    margin-left: -1rem;
  }

  .md\:-mt-5 {
    margin-top: -1.25rem;
  }

  .md\:-mr-5 {
    margin-right: -1.25rem;
  }

  .md\:-mb-5 {
    margin-bottom: -1.25rem;
  }

  .md\:-ml-5 {
    margin-left: -1.25rem;
  }

  .md\:-mt-6 {
    margin-top: -1.5rem;
  }

  .md\:-mr-6 {
    margin-right: -1.5rem;
  }

  .md\:-mb-6 {
    margin-bottom: -1.5rem;
  }

  .md\:-ml-6 {
    margin-left: -1.5rem;
  }

  .md\:-mt-8 {
    margin-top: -2rem;
  }

  .md\:-mr-8 {
    margin-right: -2rem;
  }

  .md\:-mb-8 {
    margin-bottom: -2rem;
  }

  .md\:-ml-8 {
    margin-left: -2rem;
  }

  .md\:-mt-10 {
    margin-top: -2.5rem;
  }

  .md\:-mr-10 {
    margin-right: -2.5rem;
  }

  .md\:-mb-10 {
    margin-bottom: -2.5rem;
  }

  .md\:-ml-10 {
    margin-left: -2.5rem;
  }

  .md\:-mt-12 {
    margin-top: -3rem;
  }

  .md\:-mr-12 {
    margin-right: -3rem;
  }

  .md\:-mb-12 {
    margin-bottom: -3rem;
  }

  .md\:-ml-12 {
    margin-left: -3rem;
  }

  .md\:-mt-16 {
    margin-top: -4rem;
  }

  .md\:-mr-16 {
    margin-right: -4rem;
  }

  .md\:-mb-16 {
    margin-bottom: -4rem;
  }

  .md\:-ml-16 {
    margin-left: -4rem;
  }

  .md\:-mt-20 {
    margin-top: -5rem;
  }

  .md\:-mr-20 {
    margin-right: -5rem;
  }

  .md\:-mb-20 {
    margin-bottom: -5rem;
  }

  .md\:-ml-20 {
    margin-left: -5rem;
  }

  .md\:-mt-24 {
    margin-top: -6rem;
  }

  .md\:-mr-24 {
    margin-right: -6rem;
  }

  .md\:-mb-24 {
    margin-bottom: -6rem;
  }

  .md\:-ml-24 {
    margin-left: -6rem;
  }

  .md\:-mt-32 {
    margin-top: -8rem;
  }

  .md\:-mr-32 {
    margin-right: -8rem;
  }

  .md\:-mb-32 {
    margin-bottom: -8rem;
  }

  .md\:-ml-32 {
    margin-left: -8rem;
  }

  .md\:-mt-40 {
    margin-top: -10rem;
  }

  .md\:-mr-40 {
    margin-right: -10rem;
  }

  .md\:-mb-40 {
    margin-bottom: -10rem;
  }

  .md\:-ml-40 {
    margin-left: -10rem;
  }

  .md\:-mt-48 {
    margin-top: -12rem;
  }

  .md\:-mr-48 {
    margin-right: -12rem;
  }

  .md\:-mb-48 {
    margin-bottom: -12rem;
  }

  .md\:-ml-48 {
    margin-left: -12rem;
  }

  .md\:-mt-56 {
    margin-top: -14rem;
  }

  .md\:-mr-56 {
    margin-right: -14rem;
  }

  .md\:-mb-56 {
    margin-bottom: -14rem;
  }

  .md\:-ml-56 {
    margin-left: -14rem;
  }

  .md\:-mt-64 {
    margin-top: -16rem;
  }

  .md\:-mr-64 {
    margin-right: -16rem;
  }

  .md\:-mb-64 {
    margin-bottom: -16rem;
  }

  .md\:-ml-64 {
    margin-left: -16rem;
  }

  .md\:-mt-px {
    margin-top: -1px;
  }

  .md\:-mr-px {
    margin-right: -1px;
  }

  .md\:-mb-px {
    margin-bottom: -1px;
  }

  .md\:-ml-px {
    margin-left: -1px;
  }

  .md\:max-h-full {
    max-height: 100%;
  }

  .md\:max-h-screen {
    max-height: 100vh;
  }

  .md\:max-w-xs {
    max-width: 20rem;
  }

  .md\:max-w-sm {
    max-width: 24rem;
  }

  .md\:max-w-md {
    max-width: 28rem;
  }

  .md\:max-w-lg {
    max-width: 32rem;
  }

  .md\:max-w-xl {
    max-width: 36rem;
  }

  .md\:max-w-2xl {
    max-width: 42rem;
  }

  .md\:max-w-3xl {
    max-width: 48rem;
  }

  .md\:max-w-4xl {
    max-width: 56rem;
  }

  .md\:max-w-5xl {
    max-width: 64rem;
  }

  .md\:max-w-6xl {
    max-width: 72rem;
  }

  .md\:max-w-full {
    max-width: 100%;
  }

  .md\:min-h-0 {
    min-height: 0;
  }

  .md\:min-h-full {
    min-height: 100%;
  }

  .md\:min-h-screen {
    min-height: 100vh;
  }

  .md\:min-w-0 {
    min-width: 0;
  }

  .md\:min-w-full {
    min-width: 100%;
  }

  .md\:object-contain {
    -o-object-fit: contain;
       object-fit: contain;
  }

  .md\:object-cover {
    -o-object-fit: cover;
       object-fit: cover;
  }

  .md\:object-fill {
    -o-object-fit: fill;
       object-fit: fill;
  }

  .md\:object-none {
    -o-object-fit: none;
       object-fit: none;
  }

  .md\:object-scale-down {
    -o-object-fit: scale-down;
       object-fit: scale-down;
  }

  .md\:object-bottom {
    -o-object-position: bottom;
       object-position: bottom;
  }

  .md\:object-center {
    -o-object-position: center;
       object-position: center;
  }

  .md\:object-left {
    -o-object-position: left;
       object-position: left;
  }

  .md\:object-left-bottom {
    -o-object-position: left bottom;
       object-position: left bottom;
  }

  .md\:object-left-top {
    -o-object-position: left top;
       object-position: left top;
  }

  .md\:object-right {
    -o-object-position: right;
       object-position: right;
  }

  .md\:object-right-bottom {
    -o-object-position: right bottom;
       object-position: right bottom;
  }

  .md\:object-right-top {
    -o-object-position: right top;
       object-position: right top;
  }

  .md\:object-top {
    -o-object-position: top;
       object-position: top;
  }

  .md\:opacity-0 {
    opacity: 0;
  }

  .md\:opacity-25 {
    opacity: 0.25;
  }

  .md\:opacity-50 {
    opacity: 0.5;
  }

  .md\:opacity-75 {
    opacity: 0.75;
  }

  .md\:opacity-100 {
    opacity: 1;
  }

  .md\:hover\:opacity-0:hover {
    opacity: 0;
  }

  .md\:hover\:opacity-25:hover {
    opacity: 0.25;
  }

  .md\:hover\:opacity-50:hover {
    opacity: 0.5;
  }

  .md\:hover\:opacity-75:hover {
    opacity: 0.75;
  }

  .md\:hover\:opacity-100:hover {
    opacity: 1;
  }

  .md\:focus\:opacity-0:focus {
    opacity: 0;
  }

  .md\:focus\:opacity-25:focus {
    opacity: 0.25;
  }

  .md\:focus\:opacity-50:focus {
    opacity: 0.5;
  }

  .md\:focus\:opacity-75:focus {
    opacity: 0.75;
  }

  .md\:focus\:opacity-100:focus {
    opacity: 1;
  }

  .md\:outline-none {
    outline: 0;
  }

  .md\:focus\:outline-none:focus {
    outline: 0;
  }

  .md\:overflow-auto {
    overflow: auto;
  }

  .md\:overflow-hidden {
    overflow: hidden;
  }

  .md\:overflow-visible {
    overflow: visible;
  }

  .md\:overflow-scroll {
    overflow: scroll;
  }

  .md\:overflow-x-auto {
    overflow-x: auto;
  }

  .md\:overflow-y-auto {
    overflow-y: auto;
  }

  .md\:overflow-x-hidden {
    overflow-x: hidden;
  }

  .md\:overflow-y-hidden {
    overflow-y: hidden;
  }

  .md\:overflow-x-visible {
    overflow-x: visible;
  }

  .md\:overflow-y-visible {
    overflow-y: visible;
  }

  .md\:overflow-x-scroll {
    overflow-x: scroll;
  }

  .md\:overflow-y-scroll {
    overflow-y: scroll;
  }

  .md\:scrolling-touch {
    -webkit-overflow-scrolling: touch;
  }

  .md\:scrolling-auto {
    -webkit-overflow-scrolling: auto;
  }

  .md\:p-0 {
    padding: 0;
  }

  .md\:p-1 {
    padding: 0.25rem;
  }

  .md\:p-2 {
    padding: 0.5rem;
  }

  .md\:p-3 {
    padding: 0.75rem;
  }

  .md\:p-4 {
    padding: 1rem;
  }

  .md\:p-5 {
    padding: 1.25rem;
  }

  .md\:p-6 {
    padding: 1.5rem;
  }

  .md\:p-8 {
    padding: 2rem;
  }

  .md\:p-10 {
    padding: 2.5rem;
  }

  .md\:p-12 {
    padding: 3rem;
  }

  .md\:p-16 {
    padding: 4rem;
  }

  .md\:p-20 {
    padding: 5rem;
  }

  .md\:p-24 {
    padding: 6rem;
  }

  .md\:p-32 {
    padding: 8rem;
  }

  .md\:p-40 {
    padding: 10rem;
  }

  .md\:p-48 {
    padding: 12rem;
  }

  .md\:p-56 {
    padding: 14rem;
  }

  .md\:p-64 {
    padding: 16rem;
  }

  .md\:p-px {
    padding: 1px;
  }

  .md\:py-0 {
    padding-top: 0;
    padding-bottom: 0;
  }

  .md\:px-0 {
    padding-left: 0;
    padding-right: 0;
  }

  .md\:py-1 {
    padding-top: 0.25rem;
    padding-bottom: 0.25rem;
  }

  .md\:px-1 {
    padding-left: 0.25rem;
    padding-right: 0.25rem;
  }

  .md\:py-2 {
    padding-top: 0.5rem;
    padding-bottom: 0.5rem;
  }

  .md\:px-2 {
    padding-left: 0.5rem;
    padding-right: 0.5rem;
  }

  .md\:py-3 {
    padding-top: 0.75rem;
    padding-bottom: 0.75rem;
  }

  .md\:px-3 {
    padding-left: 0.75rem;
    padding-right: 0.75rem;
  }

  .md\:py-4 {
    padding-top: 1rem;
    padding-bottom: 1rem;
  }

  .md\:px-4 {
    padding-left: 1rem;
    padding-right: 1rem;
  }

  .md\:py-5 {
    padding-top: 1.25rem;
    padding-bottom: 1.25rem;
  }

  .md\:px-5 {
    padding-left: 1.25rem;
    padding-right: 1.25rem;
  }

  .md\:py-6 {
    padding-top: 1.5rem;
    padding-bottom: 1.5rem;
  }

  .md\:px-6 {
    padding-left: 1.5rem;
    padding-right: 1.5rem;
  }

  .md\:py-8 {
    padding-top: 2rem;
    padding-bottom: 2rem;
  }

  .md\:px-8 {
    padding-left: 2rem;
    padding-right: 2rem;
  }

  .md\:py-10 {
    padding-top: 2.5rem;
    padding-bottom: 2.5rem;
  }

  .md\:px-10 {
    padding-left: 2.5rem;
    padding-right: 2.5rem;
  }

  .md\:py-12 {
    padding-top: 3rem;
    padding-bottom: 3rem;
  }

  .md\:px-12 {
    padding-left: 3rem;
    padding-right: 3rem;
  }

  .md\:py-16 {
    padding-top: 4rem;
    padding-bottom: 4rem;
  }

  .md\:px-16 {
    padding-left: 4rem;
    padding-right: 4rem;
  }

  .md\:py-20 {
    padding-top: 5rem;
    padding-bottom: 5rem;
  }

  .md\:px-20 {
    padding-left: 5rem;
    padding-right: 5rem;
  }

  .md\:py-24 {
    padding-top: 6rem;
    padding-bottom: 6rem;
  }

  .md\:px-24 {
    padding-left: 6rem;
    padding-right: 6rem;
  }

  .md\:py-32 {
    padding-top: 8rem;
    padding-bottom: 8rem;
  }

  .md\:px-32 {
    padding-left: 8rem;
    padding-right: 8rem;
  }

  .md\:py-40 {
    padding-top: 10rem;
    padding-bottom: 10rem;
  }

  .md\:px-40 {
    padding-left: 10rem;
    padding-right: 10rem;
  }

  .md\:py-48 {
    padding-top: 12rem;
    padding-bottom: 12rem;
  }

  .md\:px-48 {
    padding-left: 12rem;
    padding-right: 12rem;
  }

  .md\:py-56 {
    padding-top: 14rem;
    padding-bottom: 14rem;
  }

  .md\:px-56 {
    padding-left: 14rem;
    padding-right: 14rem;
  }

  .md\:py-64 {
    padding-top: 16rem;
    padding-bottom: 16rem;
  }

  .md\:px-64 {
    padding-left: 16rem;
    padding-right: 16rem;
  }

  .md\:py-px {
    padding-top: 1px;
    padding-bottom: 1px;
  }

  .md\:px-px {
    padding-left: 1px;
    padding-right: 1px;
  }

  .md\:pt-0 {
    padding-top: 0;
  }

  .md\:pr-0 {
    padding-right: 0;
  }

  .md\:pb-0 {
    padding-bottom: 0;
  }

  .md\:pl-0 {
    padding-left: 0;
  }

  .md\:pt-1 {
    padding-top: 0.25rem;
  }

  .md\:pr-1 {
    padding-right: 0.25rem;
  }

  .md\:pb-1 {
    padding-bottom: 0.25rem;
  }

  .md\:pl-1 {
    padding-left: 0.25rem;
  }

  .md\:pt-2 {
    padding-top: 0.5rem;
  }

  .md\:pr-2 {
    padding-right: 0.5rem;
  }

  .md\:pb-2 {
    padding-bottom: 0.5rem;
  }

  .md\:pl-2 {
    padding-left: 0.5rem;
  }

  .md\:pt-3 {
    padding-top: 0.75rem;
  }

  .md\:pr-3 {
    padding-right: 0.75rem;
  }

  .md\:pb-3 {
    padding-bottom: 0.75rem;
  }

  .md\:pl-3 {
    padding-left: 0.75rem;
  }

  .md\:pt-4 {
    padding-top: 1rem;
  }

  .md\:pr-4 {
    padding-right: 1rem;
  }

  .md\:pb-4 {
    padding-bottom: 1rem;
  }

  .md\:pl-4 {
    padding-left: 1rem;
  }

  .md\:pt-5 {
    padding-top: 1.25rem;
  }

  .md\:pr-5 {
    padding-right: 1.25rem;
  }

  .md\:pb-5 {
    padding-bottom: 1.25rem;
  }

  .md\:pl-5 {
    padding-left: 1.25rem;
  }

  .md\:pt-6 {
    padding-top: 1.5rem;
  }

  .md\:pr-6 {
    padding-right: 1.5rem;
  }

  .md\:pb-6 {
    padding-bottom: 1.5rem;
  }

  .md\:pl-6 {
    padding-left: 1.5rem;
  }

  .md\:pt-8 {
    padding-top: 2rem;
  }

  .md\:pr-8 {
    padding-right: 2rem;
  }

  .md\:pb-8 {
    padding-bottom: 2rem;
  }

  .md\:pl-8 {
    padding-left: 2rem;
  }

  .md\:pt-10 {
    padding-top: 2.5rem;
  }

  .md\:pr-10 {
    padding-right: 2.5rem;
  }

  .md\:pb-10 {
    padding-bottom: 2.5rem;
  }

  .md\:pl-10 {
    padding-left: 2.5rem;
  }

  .md\:pt-12 {
    padding-top: 3rem;
  }

  .md\:pr-12 {
    padding-right: 3rem;
  }

  .md\:pb-12 {
    padding-bottom: 3rem;
  }

  .md\:pl-12 {
    padding-left: 3rem;
  }

  .md\:pt-16 {
    padding-top: 4rem;
  }

  .md\:pr-16 {
    padding-right: 4rem;
  }

  .md\:pb-16 {
    padding-bottom: 4rem;
  }

  .md\:pl-16 {
    padding-left: 4rem;
  }

  .md\:pt-20 {
    padding-top: 5rem;
  }

  .md\:pr-20 {
    padding-right: 5rem;
  }

  .md\:pb-20 {
    padding-bottom: 5rem;
  }

  .md\:pl-20 {
    padding-left: 5rem;
  }

  .md\:pt-24 {
    padding-top: 6rem;
  }

  .md\:pr-24 {
    padding-right: 6rem;
  }

  .md\:pb-24 {
    padding-bottom: 6rem;
  }

  .md\:pl-24 {
    padding-left: 6rem;
  }

  .md\:pt-32 {
    padding-top: 8rem;
  }

  .md\:pr-32 {
    padding-right: 8rem;
  }

  .md\:pb-32 {
    padding-bottom: 8rem;
  }

  .md\:pl-32 {
    padding-left: 8rem;
  }

  .md\:pt-40 {
    padding-top: 10rem;
  }

  .md\:pr-40 {
    padding-right: 10rem;
  }

  .md\:pb-40 {
    padding-bottom: 10rem;
  }

  .md\:pl-40 {
    padding-left: 10rem;
  }

  .md\:pt-48 {
    padding-top: 12rem;
  }

  .md\:pr-48 {
    padding-right: 12rem;
  }

  .md\:pb-48 {
    padding-bottom: 12rem;
  }

  .md\:pl-48 {
    padding-left: 12rem;
  }

  .md\:pt-56 {
    padding-top: 14rem;
  }

  .md\:pr-56 {
    padding-right: 14rem;
  }

  .md\:pb-56 {
    padding-bottom: 14rem;
  }

  .md\:pl-56 {
    padding-left: 14rem;
  }

  .md\:pt-64 {
    padding-top: 16rem;
  }

  .md\:pr-64 {
    padding-right: 16rem;
  }

  .md\:pb-64 {
    padding-bottom: 16rem;
  }

  .md\:pl-64 {
    padding-left: 16rem;
  }

  .md\:pt-px {
    padding-top: 1px;
  }

  .md\:pr-px {
    padding-right: 1px;
  }

  .md\:pb-px {
    padding-bottom: 1px;
  }

  .md\:pl-px {
    padding-left: 1px;
  }

  .md\:placeholder-transparent::-webkit-input-placeholder {
    color: transparent;
  }

  .md\:placeholder-transparent::-moz-placeholder {
    color: transparent;
  }

  .md\:placeholder-transparent:-ms-input-placeholder {
    color: transparent;
  }

  .md\:placeholder-transparent::-ms-input-placeholder {
    color: transparent;
  }

  .md\:placeholder-transparent::placeholder {
    color: transparent;
  }

  .md\:placeholder-black::-webkit-input-placeholder {
    color: #000;
  }

  .md\:placeholder-black::-moz-placeholder {
    color: #000;
  }

  .md\:placeholder-black:-ms-input-placeholder {
    color: #000;
  }

  .md\:placeholder-black::-ms-input-placeholder {
    color: #000;
  }

  .md\:placeholder-black::placeholder {
    color: #000;
  }

  .md\:placeholder-white::-webkit-input-placeholder {
    color: #fff;
  }

  .md\:placeholder-white::-moz-placeholder {
    color: #fff;
  }

  .md\:placeholder-white:-ms-input-placeholder {
    color: #fff;
  }

  .md\:placeholder-white::-ms-input-placeholder {
    color: #fff;
  }

  .md\:placeholder-white::placeholder {
    color: #fff;
  }

  .md\:placeholder-gray-100::-webkit-input-placeholder {
    color: #f7fafc;
  }

  .md\:placeholder-gray-100::-moz-placeholder {
    color: #f7fafc;
  }

  .md\:placeholder-gray-100:-ms-input-placeholder {
    color: #f7fafc;
  }

  .md\:placeholder-gray-100::-ms-input-placeholder {
    color: #f7fafc;
  }

  .md\:placeholder-gray-100::placeholder {
    color: #f7fafc;
  }

  .md\:placeholder-gray-200::-webkit-input-placeholder {
    color: #edf2f7;
  }

  .md\:placeholder-gray-200::-moz-placeholder {
    color: #edf2f7;
  }

  .md\:placeholder-gray-200:-ms-input-placeholder {
    color: #edf2f7;
  }

  .md\:placeholder-gray-200::-ms-input-placeholder {
    color: #edf2f7;
  }

  .md\:placeholder-gray-200::placeholder {
    color: #edf2f7;
  }

  .md\:placeholder-gray-300::-webkit-input-placeholder {
    color: #e2e8f0;
  }

  .md\:placeholder-gray-300::-moz-placeholder {
    color: #e2e8f0;
  }

  .md\:placeholder-gray-300:-ms-input-placeholder {
    color: #e2e8f0;
  }

  .md\:placeholder-gray-300::-ms-input-placeholder {
    color: #e2e8f0;
  }

  .md\:placeholder-gray-300::placeholder {
    color: #e2e8f0;
  }

  .md\:placeholder-gray-400::-webkit-input-placeholder {
    color: #cbd5e0;
  }

  .md\:placeholder-gray-400::-moz-placeholder {
    color: #cbd5e0;
  }

  .md\:placeholder-gray-400:-ms-input-placeholder {
    color: #cbd5e0;
  }

  .md\:placeholder-gray-400::-ms-input-placeholder {
    color: #cbd5e0;
  }

  .md\:placeholder-gray-400::placeholder {
    color: #cbd5e0;
  }

  .md\:placeholder-gray-500::-webkit-input-placeholder {
    color: #a0aec0;
  }

  .md\:placeholder-gray-500::-moz-placeholder {
    color: #a0aec0;
  }

  .md\:placeholder-gray-500:-ms-input-placeholder {
    color: #a0aec0;
  }

  .md\:placeholder-gray-500::-ms-input-placeholder {
    color: #a0aec0;
  }

  .md\:placeholder-gray-500::placeholder {
    color: #a0aec0;
  }

  .md\:placeholder-gray-600::-webkit-input-placeholder {
    color: #718096;
  }

  .md\:placeholder-gray-600::-moz-placeholder {
    color: #718096;
  }

  .md\:placeholder-gray-600:-ms-input-placeholder {
    color: #718096;
  }

  .md\:placeholder-gray-600::-ms-input-placeholder {
    color: #718096;
  }

  .md\:placeholder-gray-600::placeholder {
    color: #718096;
  }

  .md\:placeholder-gray-700::-webkit-input-placeholder {
    color: #4a5568;
  }

  .md\:placeholder-gray-700::-moz-placeholder {
    color: #4a5568;
  }

  .md\:placeholder-gray-700:-ms-input-placeholder {
    color: #4a5568;
  }

  .md\:placeholder-gray-700::-ms-input-placeholder {
    color: #4a5568;
  }

  .md\:placeholder-gray-700::placeholder {
    color: #4a5568;
  }

  .md\:placeholder-gray-800::-webkit-input-placeholder {
    color: #2d3748;
  }

  .md\:placeholder-gray-800::-moz-placeholder {
    color: #2d3748;
  }

  .md\:placeholder-gray-800:-ms-input-placeholder {
    color: #2d3748;
  }

  .md\:placeholder-gray-800::-ms-input-placeholder {
    color: #2d3748;
  }

  .md\:placeholder-gray-800::placeholder {
    color: #2d3748;
  }

  .md\:placeholder-gray-900::-webkit-input-placeholder {
    color: #1a202c;
  }

  .md\:placeholder-gray-900::-moz-placeholder {
    color: #1a202c;
  }

  .md\:placeholder-gray-900:-ms-input-placeholder {
    color: #1a202c;
  }

  .md\:placeholder-gray-900::-ms-input-placeholder {
    color: #1a202c;
  }

  .md\:placeholder-gray-900::placeholder {
    color: #1a202c;
  }

  .md\:placeholder-red-100::-webkit-input-placeholder {
    color: #fff5f5;
  }

  .md\:placeholder-red-100::-moz-placeholder {
    color: #fff5f5;
  }

  .md\:placeholder-red-100:-ms-input-placeholder {
    color: #fff5f5;
  }

  .md\:placeholder-red-100::-ms-input-placeholder {
    color: #fff5f5;
  }

  .md\:placeholder-red-100::placeholder {
    color: #fff5f5;
  }

  .md\:placeholder-red-200::-webkit-input-placeholder {
    color: #fed7d7;
  }

  .md\:placeholder-red-200::-moz-placeholder {
    color: #fed7d7;
  }

  .md\:placeholder-red-200:-ms-input-placeholder {
    color: #fed7d7;
  }

  .md\:placeholder-red-200::-ms-input-placeholder {
    color: #fed7d7;
  }

  .md\:placeholder-red-200::placeholder {
    color: #fed7d7;
  }

  .md\:placeholder-red-300::-webkit-input-placeholder {
    color: #feb2b2;
  }

  .md\:placeholder-red-300::-moz-placeholder {
    color: #feb2b2;
  }

  .md\:placeholder-red-300:-ms-input-placeholder {
    color: #feb2b2;
  }

  .md\:placeholder-red-300::-ms-input-placeholder {
    color: #feb2b2;
  }

  .md\:placeholder-red-300::placeholder {
    color: #feb2b2;
  }

  .md\:placeholder-red-400::-webkit-input-placeholder {
    color: #fc8181;
  }

  .md\:placeholder-red-400::-moz-placeholder {
    color: #fc8181;
  }

  .md\:placeholder-red-400:-ms-input-placeholder {
    color: #fc8181;
  }

  .md\:placeholder-red-400::-ms-input-placeholder {
    color: #fc8181;
  }

  .md\:placeholder-red-400::placeholder {
    color: #fc8181;
  }

  .md\:placeholder-red-500::-webkit-input-placeholder {
    color: #f56565;
  }

  .md\:placeholder-red-500::-moz-placeholder {
    color: #f56565;
  }

  .md\:placeholder-red-500:-ms-input-placeholder {
    color: #f56565;
  }

  .md\:placeholder-red-500::-ms-input-placeholder {
    color: #f56565;
  }

  .md\:placeholder-red-500::placeholder {
    color: #f56565;
  }

  .md\:placeholder-red-600::-webkit-input-placeholder {
    color: #e53e3e;
  }

  .md\:placeholder-red-600::-moz-placeholder {
    color: #e53e3e;
  }

  .md\:placeholder-red-600:-ms-input-placeholder {
    color: #e53e3e;
  }

  .md\:placeholder-red-600::-ms-input-placeholder {
    color: #e53e3e;
  }

  .md\:placeholder-red-600::placeholder {
    color: #e53e3e;
  }

  .md\:placeholder-red-700::-webkit-input-placeholder {
    color: #c53030;
  }

  .md\:placeholder-red-700::-moz-placeholder {
    color: #c53030;
  }

  .md\:placeholder-red-700:-ms-input-placeholder {
    color: #c53030;
  }

  .md\:placeholder-red-700::-ms-input-placeholder {
    color: #c53030;
  }

  .md\:placeholder-red-700::placeholder {
    color: #c53030;
  }

  .md\:placeholder-red-800::-webkit-input-placeholder {
    color: #9b2c2c;
  }

  .md\:placeholder-red-800::-moz-placeholder {
    color: #9b2c2c;
  }

  .md\:placeholder-red-800:-ms-input-placeholder {
    color: #9b2c2c;
  }

  .md\:placeholder-red-800::-ms-input-placeholder {
    color: #9b2c2c;
  }

  .md\:placeholder-red-800::placeholder {
    color: #9b2c2c;
  }

  .md\:placeholder-red-900::-webkit-input-placeholder {
    color: #742a2a;
  }

  .md\:placeholder-red-900::-moz-placeholder {
    color: #742a2a;
  }

  .md\:placeholder-red-900:-ms-input-placeholder {
    color: #742a2a;
  }

  .md\:placeholder-red-900::-ms-input-placeholder {
    color: #742a2a;
  }

  .md\:placeholder-red-900::placeholder {
    color: #742a2a;
  }

  .md\:placeholder-orange-100::-webkit-input-placeholder {
    color: #fffaf0;
  }

  .md\:placeholder-orange-100::-moz-placeholder {
    color: #fffaf0;
  }

  .md\:placeholder-orange-100:-ms-input-placeholder {
    color: #fffaf0;
  }

  .md\:placeholder-orange-100::-ms-input-placeholder {
    color: #fffaf0;
  }

  .md\:placeholder-orange-100::placeholder {
    color: #fffaf0;
  }

  .md\:placeholder-orange-200::-webkit-input-placeholder {
    color: #feebc8;
  }

  .md\:placeholder-orange-200::-moz-placeholder {
    color: #feebc8;
  }

  .md\:placeholder-orange-200:-ms-input-placeholder {
    color: #feebc8;
  }

  .md\:placeholder-orange-200::-ms-input-placeholder {
    color: #feebc8;
  }

  .md\:placeholder-orange-200::placeholder {
    color: #feebc8;
  }

  .md\:placeholder-orange-300::-webkit-input-placeholder {
    color: #fbd38d;
  }

  .md\:placeholder-orange-300::-moz-placeholder {
    color: #fbd38d;
  }

  .md\:placeholder-orange-300:-ms-input-placeholder {
    color: #fbd38d;
  }

  .md\:placeholder-orange-300::-ms-input-placeholder {
    color: #fbd38d;
  }

  .md\:placeholder-orange-300::placeholder {
    color: #fbd38d;
  }

  .md\:placeholder-orange-400::-webkit-input-placeholder {
    color: #f6ad55;
  }

  .md\:placeholder-orange-400::-moz-placeholder {
    color: #f6ad55;
  }

  .md\:placeholder-orange-400:-ms-input-placeholder {
    color: #f6ad55;
  }

  .md\:placeholder-orange-400::-ms-input-placeholder {
    color: #f6ad55;
  }

  .md\:placeholder-orange-400::placeholder {
    color: #f6ad55;
  }

  .md\:placeholder-orange-500::-webkit-input-placeholder {
    color: #ed8936;
  }

  .md\:placeholder-orange-500::-moz-placeholder {
    color: #ed8936;
  }

  .md\:placeholder-orange-500:-ms-input-placeholder {
    color: #ed8936;
  }

  .md\:placeholder-orange-500::-ms-input-placeholder {
    color: #ed8936;
  }

  .md\:placeholder-orange-500::placeholder {
    color: #ed8936;
  }

  .md\:placeholder-orange-600::-webkit-input-placeholder {
    color: #dd6b20;
  }

  .md\:placeholder-orange-600::-moz-placeholder {
    color: #dd6b20;
  }

  .md\:placeholder-orange-600:-ms-input-placeholder {
    color: #dd6b20;
  }

  .md\:placeholder-orange-600::-ms-input-placeholder {
    color: #dd6b20;
  }

  .md\:placeholder-orange-600::placeholder {
    color: #dd6b20;
  }

  .md\:placeholder-orange-700::-webkit-input-placeholder {
    color: #c05621;
  }

  .md\:placeholder-orange-700::-moz-placeholder {
    color: #c05621;
  }

  .md\:placeholder-orange-700:-ms-input-placeholder {
    color: #c05621;
  }

  .md\:placeholder-orange-700::-ms-input-placeholder {
    color: #c05621;
  }

  .md\:placeholder-orange-700::placeholder {
    color: #c05621;
  }

  .md\:placeholder-orange-800::-webkit-input-placeholder {
    color: #9c4221;
  }

  .md\:placeholder-orange-800::-moz-placeholder {
    color: #9c4221;
  }

  .md\:placeholder-orange-800:-ms-input-placeholder {
    color: #9c4221;
  }

  .md\:placeholder-orange-800::-ms-input-placeholder {
    color: #9c4221;
  }

  .md\:placeholder-orange-800::placeholder {
    color: #9c4221;
  }

  .md\:placeholder-orange-900::-webkit-input-placeholder {
    color: #7b341e;
  }

  .md\:placeholder-orange-900::-moz-placeholder {
    color: #7b341e;
  }

  .md\:placeholder-orange-900:-ms-input-placeholder {
    color: #7b341e;
  }

  .md\:placeholder-orange-900::-ms-input-placeholder {
    color: #7b341e;
  }

  .md\:placeholder-orange-900::placeholder {
    color: #7b341e;
  }

  .md\:placeholder-yellow-100::-webkit-input-placeholder {
    color: #fffff0;
  }

  .md\:placeholder-yellow-100::-moz-placeholder {
    color: #fffff0;
  }

  .md\:placeholder-yellow-100:-ms-input-placeholder {
    color: #fffff0;
  }

  .md\:placeholder-yellow-100::-ms-input-placeholder {
    color: #fffff0;
  }

  .md\:placeholder-yellow-100::placeholder {
    color: #fffff0;
  }

  .md\:placeholder-yellow-200::-webkit-input-placeholder {
    color: #fefcbf;
  }

  .md\:placeholder-yellow-200::-moz-placeholder {
    color: #fefcbf;
  }

  .md\:placeholder-yellow-200:-ms-input-placeholder {
    color: #fefcbf;
  }

  .md\:placeholder-yellow-200::-ms-input-placeholder {
    color: #fefcbf;
  }

  .md\:placeholder-yellow-200::placeholder {
    color: #fefcbf;
  }

  .md\:placeholder-yellow-300::-webkit-input-placeholder {
    color: #faf089;
  }

  .md\:placeholder-yellow-300::-moz-placeholder {
    color: #faf089;
  }

  .md\:placeholder-yellow-300:-ms-input-placeholder {
    color: #faf089;
  }

  .md\:placeholder-yellow-300::-ms-input-placeholder {
    color: #faf089;
  }

  .md\:placeholder-yellow-300::placeholder {
    color: #faf089;
  }

  .md\:placeholder-yellow-400::-webkit-input-placeholder {
    color: #f6e05e;
  }

  .md\:placeholder-yellow-400::-moz-placeholder {
    color: #f6e05e;
  }

  .md\:placeholder-yellow-400:-ms-input-placeholder {
    color: #f6e05e;
  }

  .md\:placeholder-yellow-400::-ms-input-placeholder {
    color: #f6e05e;
  }

  .md\:placeholder-yellow-400::placeholder {
    color: #f6e05e;
  }

  .md\:placeholder-yellow-500::-webkit-input-placeholder {
    color: #ecc94b;
  }

  .md\:placeholder-yellow-500::-moz-placeholder {
    color: #ecc94b;
  }

  .md\:placeholder-yellow-500:-ms-input-placeholder {
    color: #ecc94b;
  }

  .md\:placeholder-yellow-500::-ms-input-placeholder {
    color: #ecc94b;
  }

  .md\:placeholder-yellow-500::placeholder {
    color: #ecc94b;
  }

  .md\:placeholder-yellow-600::-webkit-input-placeholder {
    color: #d69e2e;
  }

  .md\:placeholder-yellow-600::-moz-placeholder {
    color: #d69e2e;
  }

  .md\:placeholder-yellow-600:-ms-input-placeholder {
    color: #d69e2e;
  }

  .md\:placeholder-yellow-600::-ms-input-placeholder {
    color: #d69e2e;
  }

  .md\:placeholder-yellow-600::placeholder {
    color: #d69e2e;
  }

  .md\:placeholder-yellow-700::-webkit-input-placeholder {
    color: #b7791f;
  }

  .md\:placeholder-yellow-700::-moz-placeholder {
    color: #b7791f;
  }

  .md\:placeholder-yellow-700:-ms-input-placeholder {
    color: #b7791f;
  }

  .md\:placeholder-yellow-700::-ms-input-placeholder {
    color: #b7791f;
  }

  .md\:placeholder-yellow-700::placeholder {
    color: #b7791f;
  }

  .md\:placeholder-yellow-800::-webkit-input-placeholder {
    color: #975a16;
  }

  .md\:placeholder-yellow-800::-moz-placeholder {
    color: #975a16;
  }

  .md\:placeholder-yellow-800:-ms-input-placeholder {
    color: #975a16;
  }

  .md\:placeholder-yellow-800::-ms-input-placeholder {
    color: #975a16;
  }

  .md\:placeholder-yellow-800::placeholder {
    color: #975a16;
  }

  .md\:placeholder-yellow-900::-webkit-input-placeholder {
    color: #744210;
  }

  .md\:placeholder-yellow-900::-moz-placeholder {
    color: #744210;
  }

  .md\:placeholder-yellow-900:-ms-input-placeholder {
    color: #744210;
  }

  .md\:placeholder-yellow-900::-ms-input-placeholder {
    color: #744210;
  }

  .md\:placeholder-yellow-900::placeholder {
    color: #744210;
  }

  .md\:placeholder-green-100::-webkit-input-placeholder {
    color: #f0fff4;
  }

  .md\:placeholder-green-100::-moz-placeholder {
    color: #f0fff4;
  }

  .md\:placeholder-green-100:-ms-input-placeholder {
    color: #f0fff4;
  }

  .md\:placeholder-green-100::-ms-input-placeholder {
    color: #f0fff4;
  }

  .md\:placeholder-green-100::placeholder {
    color: #f0fff4;
  }

  .md\:placeholder-green-200::-webkit-input-placeholder {
    color: #c6f6d5;
  }

  .md\:placeholder-green-200::-moz-placeholder {
    color: #c6f6d5;
  }

  .md\:placeholder-green-200:-ms-input-placeholder {
    color: #c6f6d5;
  }

  .md\:placeholder-green-200::-ms-input-placeholder {
    color: #c6f6d5;
  }

  .md\:placeholder-green-200::placeholder {
    color: #c6f6d5;
  }

  .md\:placeholder-green-300::-webkit-input-placeholder {
    color: #9ae6b4;
  }

  .md\:placeholder-green-300::-moz-placeholder {
    color: #9ae6b4;
  }

  .md\:placeholder-green-300:-ms-input-placeholder {
    color: #9ae6b4;
  }

  .md\:placeholder-green-300::-ms-input-placeholder {
    color: #9ae6b4;
  }

  .md\:placeholder-green-300::placeholder {
    color: #9ae6b4;
  }

  .md\:placeholder-green-400::-webkit-input-placeholder {
    color: #68d391;
  }

  .md\:placeholder-green-400::-moz-placeholder {
    color: #68d391;
  }

  .md\:placeholder-green-400:-ms-input-placeholder {
    color: #68d391;
  }

  .md\:placeholder-green-400::-ms-input-placeholder {
    color: #68d391;
  }

  .md\:placeholder-green-400::placeholder {
    color: #68d391;
  }

  .md\:placeholder-green-500::-webkit-input-placeholder {
    color: #48bb78;
  }

  .md\:placeholder-green-500::-moz-placeholder {
    color: #48bb78;
  }

  .md\:placeholder-green-500:-ms-input-placeholder {
    color: #48bb78;
  }

  .md\:placeholder-green-500::-ms-input-placeholder {
    color: #48bb78;
  }

  .md\:placeholder-green-500::placeholder {
    color: #48bb78;
  }

  .md\:placeholder-green-600::-webkit-input-placeholder {
    color: #38a169;
  }

  .md\:placeholder-green-600::-moz-placeholder {
    color: #38a169;
  }

  .md\:placeholder-green-600:-ms-input-placeholder {
    color: #38a169;
  }

  .md\:placeholder-green-600::-ms-input-placeholder {
    color: #38a169;
  }

  .md\:placeholder-green-600::placeholder {
    color: #38a169;
  }

  .md\:placeholder-green-700::-webkit-input-placeholder {
    color: #2f855a;
  }

  .md\:placeholder-green-700::-moz-placeholder {
    color: #2f855a;
  }

  .md\:placeholder-green-700:-ms-input-placeholder {
    color: #2f855a;
  }

  .md\:placeholder-green-700::-ms-input-placeholder {
    color: #2f855a;
  }

  .md\:placeholder-green-700::placeholder {
    color: #2f855a;
  }

  .md\:placeholder-green-800::-webkit-input-placeholder {
    color: #276749;
  }

  .md\:placeholder-green-800::-moz-placeholder {
    color: #276749;
  }

  .md\:placeholder-green-800:-ms-input-placeholder {
    color: #276749;
  }

  .md\:placeholder-green-800::-ms-input-placeholder {
    color: #276749;
  }

  .md\:placeholder-green-800::placeholder {
    color: #276749;
  }

  .md\:placeholder-green-900::-webkit-input-placeholder {
    color: #22543d;
  }

  .md\:placeholder-green-900::-moz-placeholder {
    color: #22543d;
  }

  .md\:placeholder-green-900:-ms-input-placeholder {
    color: #22543d;
  }

  .md\:placeholder-green-900::-ms-input-placeholder {
    color: #22543d;
  }

  .md\:placeholder-green-900::placeholder {
    color: #22543d;
  }

  .md\:placeholder-teal-100::-webkit-input-placeholder {
    color: #e6fffa;
  }

  .md\:placeholder-teal-100::-moz-placeholder {
    color: #e6fffa;
  }

  .md\:placeholder-teal-100:-ms-input-placeholder {
    color: #e6fffa;
  }

  .md\:placeholder-teal-100::-ms-input-placeholder {
    color: #e6fffa;
  }

  .md\:placeholder-teal-100::placeholder {
    color: #e6fffa;
  }

  .md\:placeholder-teal-200::-webkit-input-placeholder {
    color: #b2f5ea;
  }

  .md\:placeholder-teal-200::-moz-placeholder {
    color: #b2f5ea;
  }

  .md\:placeholder-teal-200:-ms-input-placeholder {
    color: #b2f5ea;
  }

  .md\:placeholder-teal-200::-ms-input-placeholder {
    color: #b2f5ea;
  }

  .md\:placeholder-teal-200::placeholder {
    color: #b2f5ea;
  }

  .md\:placeholder-teal-300::-webkit-input-placeholder {
    color: #81e6d9;
  }

  .md\:placeholder-teal-300::-moz-placeholder {
    color: #81e6d9;
  }

  .md\:placeholder-teal-300:-ms-input-placeholder {
    color: #81e6d9;
  }

  .md\:placeholder-teal-300::-ms-input-placeholder {
    color: #81e6d9;
  }

  .md\:placeholder-teal-300::placeholder {
    color: #81e6d9;
  }

  .md\:placeholder-teal-400::-webkit-input-placeholder {
    color: #4fd1c5;
  }

  .md\:placeholder-teal-400::-moz-placeholder {
    color: #4fd1c5;
  }

  .md\:placeholder-teal-400:-ms-input-placeholder {
    color: #4fd1c5;
  }

  .md\:placeholder-teal-400::-ms-input-placeholder {
    color: #4fd1c5;
  }

  .md\:placeholder-teal-400::placeholder {
    color: #4fd1c5;
  }

  .md\:placeholder-teal-500::-webkit-input-placeholder {
    color: #38b2ac;
  }

  .md\:placeholder-teal-500::-moz-placeholder {
    color: #38b2ac;
  }

  .md\:placeholder-teal-500:-ms-input-placeholder {
    color: #38b2ac;
  }

  .md\:placeholder-teal-500::-ms-input-placeholder {
    color: #38b2ac;
  }

  .md\:placeholder-teal-500::placeholder {
    color: #38b2ac;
  }

  .md\:placeholder-teal-600::-webkit-input-placeholder {
    color: #319795;
  }

  .md\:placeholder-teal-600::-moz-placeholder {
    color: #319795;
  }

  .md\:placeholder-teal-600:-ms-input-placeholder {
    color: #319795;
  }

  .md\:placeholder-teal-600::-ms-input-placeholder {
    color: #319795;
  }

  .md\:placeholder-teal-600::placeholder {
    color: #319795;
  }

  .md\:placeholder-teal-700::-webkit-input-placeholder {
    color: #2c7a7b;
  }

  .md\:placeholder-teal-700::-moz-placeholder {
    color: #2c7a7b;
  }

  .md\:placeholder-teal-700:-ms-input-placeholder {
    color: #2c7a7b;
  }

  .md\:placeholder-teal-700::-ms-input-placeholder {
    color: #2c7a7b;
  }

  .md\:placeholder-teal-700::placeholder {
    color: #2c7a7b;
  }

  .md\:placeholder-teal-800::-webkit-input-placeholder {
    color: #285e61;
  }

  .md\:placeholder-teal-800::-moz-placeholder {
    color: #285e61;
  }

  .md\:placeholder-teal-800:-ms-input-placeholder {
    color: #285e61;
  }

  .md\:placeholder-teal-800::-ms-input-placeholder {
    color: #285e61;
  }

  .md\:placeholder-teal-800::placeholder {
    color: #285e61;
  }

  .md\:placeholder-teal-900::-webkit-input-placeholder {
    color: #234e52;
  }

  .md\:placeholder-teal-900::-moz-placeholder {
    color: #234e52;
  }

  .md\:placeholder-teal-900:-ms-input-placeholder {
    color: #234e52;
  }

  .md\:placeholder-teal-900::-ms-input-placeholder {
    color: #234e52;
  }

  .md\:placeholder-teal-900::placeholder {
    color: #234e52;
  }

  .md\:placeholder-blue-100::-webkit-input-placeholder {
    color: #ebf8ff;
  }

  .md\:placeholder-blue-100::-moz-placeholder {
    color: #ebf8ff;
  }

  .md\:placeholder-blue-100:-ms-input-placeholder {
    color: #ebf8ff;
  }

  .md\:placeholder-blue-100::-ms-input-placeholder {
    color: #ebf8ff;
  }

  .md\:placeholder-blue-100::placeholder {
    color: #ebf8ff;
  }

  .md\:placeholder-blue-200::-webkit-input-placeholder {
    color: #bee3f8;
  }

  .md\:placeholder-blue-200::-moz-placeholder {
    color: #bee3f8;
  }

  .md\:placeholder-blue-200:-ms-input-placeholder {
    color: #bee3f8;
  }

  .md\:placeholder-blue-200::-ms-input-placeholder {
    color: #bee3f8;
  }

  .md\:placeholder-blue-200::placeholder {
    color: #bee3f8;
  }

  .md\:placeholder-blue-300::-webkit-input-placeholder {
    color: #90cdf4;
  }

  .md\:placeholder-blue-300::-moz-placeholder {
    color: #90cdf4;
  }

  .md\:placeholder-blue-300:-ms-input-placeholder {
    color: #90cdf4;
  }

  .md\:placeholder-blue-300::-ms-input-placeholder {
    color: #90cdf4;
  }

  .md\:placeholder-blue-300::placeholder {
    color: #90cdf4;
  }

  .md\:placeholder-blue-400::-webkit-input-placeholder {
    color: #63b3ed;
  }

  .md\:placeholder-blue-400::-moz-placeholder {
    color: #63b3ed;
  }

  .md\:placeholder-blue-400:-ms-input-placeholder {
    color: #63b3ed;
  }

  .md\:placeholder-blue-400::-ms-input-placeholder {
    color: #63b3ed;
  }

  .md\:placeholder-blue-400::placeholder {
    color: #63b3ed;
  }

  .md\:placeholder-blue-500::-webkit-input-placeholder {
    color: #4299e1;
  }

  .md\:placeholder-blue-500::-moz-placeholder {
    color: #4299e1;
  }

  .md\:placeholder-blue-500:-ms-input-placeholder {
    color: #4299e1;
  }

  .md\:placeholder-blue-500::-ms-input-placeholder {
    color: #4299e1;
  }

  .md\:placeholder-blue-500::placeholder {
    color: #4299e1;
  }

  .md\:placeholder-blue-600::-webkit-input-placeholder {
    color: #3182ce;
  }

  .md\:placeholder-blue-600::-moz-placeholder {
    color: #3182ce;
  }

  .md\:placeholder-blue-600:-ms-input-placeholder {
    color: #3182ce;
  }

  .md\:placeholder-blue-600::-ms-input-placeholder {
    color: #3182ce;
  }

  .md\:placeholder-blue-600::placeholder {
    color: #3182ce;
  }

  .md\:placeholder-blue-700::-webkit-input-placeholder {
    color: #2b6cb0;
  }

  .md\:placeholder-blue-700::-moz-placeholder {
    color: #2b6cb0;
  }

  .md\:placeholder-blue-700:-ms-input-placeholder {
    color: #2b6cb0;
  }

  .md\:placeholder-blue-700::-ms-input-placeholder {
    color: #2b6cb0;
  }

  .md\:placeholder-blue-700::placeholder {
    color: #2b6cb0;
  }

  .md\:placeholder-blue-800::-webkit-input-placeholder {
    color: #2c5282;
  }

  .md\:placeholder-blue-800::-moz-placeholder {
    color: #2c5282;
  }

  .md\:placeholder-blue-800:-ms-input-placeholder {
    color: #2c5282;
  }

  .md\:placeholder-blue-800::-ms-input-placeholder {
    color: #2c5282;
  }

  .md\:placeholder-blue-800::placeholder {
    color: #2c5282;
  }

  .md\:placeholder-blue-900::-webkit-input-placeholder {
    color: #2a4365;
  }

  .md\:placeholder-blue-900::-moz-placeholder {
    color: #2a4365;
  }

  .md\:placeholder-blue-900:-ms-input-placeholder {
    color: #2a4365;
  }

  .md\:placeholder-blue-900::-ms-input-placeholder {
    color: #2a4365;
  }

  .md\:placeholder-blue-900::placeholder {
    color: #2a4365;
  }

  .md\:placeholder-indigo-100::-webkit-input-placeholder {
    color: #ebf4ff;
  }

  .md\:placeholder-indigo-100::-moz-placeholder {
    color: #ebf4ff;
  }

  .md\:placeholder-indigo-100:-ms-input-placeholder {
    color: #ebf4ff;
  }

  .md\:placeholder-indigo-100::-ms-input-placeholder {
    color: #ebf4ff;
  }

  .md\:placeholder-indigo-100::placeholder {
    color: #ebf4ff;
  }

  .md\:placeholder-indigo-200::-webkit-input-placeholder {
    color: #c3dafe;
  }

  .md\:placeholder-indigo-200::-moz-placeholder {
    color: #c3dafe;
  }

  .md\:placeholder-indigo-200:-ms-input-placeholder {
    color: #c3dafe;
  }

  .md\:placeholder-indigo-200::-ms-input-placeholder {
    color: #c3dafe;
  }

  .md\:placeholder-indigo-200::placeholder {
    color: #c3dafe;
  }

  .md\:placeholder-indigo-300::-webkit-input-placeholder {
    color: #a3bffa;
  }

  .md\:placeholder-indigo-300::-moz-placeholder {
    color: #a3bffa;
  }

  .md\:placeholder-indigo-300:-ms-input-placeholder {
    color: #a3bffa;
  }

  .md\:placeholder-indigo-300::-ms-input-placeholder {
    color: #a3bffa;
  }

  .md\:placeholder-indigo-300::placeholder {
    color: #a3bffa;
  }

  .md\:placeholder-indigo-400::-webkit-input-placeholder {
    color: #7f9cf5;
  }

  .md\:placeholder-indigo-400::-moz-placeholder {
    color: #7f9cf5;
  }

  .md\:placeholder-indigo-400:-ms-input-placeholder {
    color: #7f9cf5;
  }

  .md\:placeholder-indigo-400::-ms-input-placeholder {
    color: #7f9cf5;
  }

  .md\:placeholder-indigo-400::placeholder {
    color: #7f9cf5;
  }

  .md\:placeholder-indigo-500::-webkit-input-placeholder {
    color: #667eea;
  }

  .md\:placeholder-indigo-500::-moz-placeholder {
    color: #667eea;
  }

  .md\:placeholder-indigo-500:-ms-input-placeholder {
    color: #667eea;
  }

  .md\:placeholder-indigo-500::-ms-input-placeholder {
    color: #667eea;
  }

  .md\:placeholder-indigo-500::placeholder {
    color: #667eea;
  }

  .md\:placeholder-indigo-600::-webkit-input-placeholder {
    color: #5a67d8;
  }

  .md\:placeholder-indigo-600::-moz-placeholder {
    color: #5a67d8;
  }

  .md\:placeholder-indigo-600:-ms-input-placeholder {
    color: #5a67d8;
  }

  .md\:placeholder-indigo-600::-ms-input-placeholder {
    color: #5a67d8;
  }

  .md\:placeholder-indigo-600::placeholder {
    color: #5a67d8;
  }

  .md\:placeholder-indigo-700::-webkit-input-placeholder {
    color: #4c51bf;
  }

  .md\:placeholder-indigo-700::-moz-placeholder {
    color: #4c51bf;
  }

  .md\:placeholder-indigo-700:-ms-input-placeholder {
    color: #4c51bf;
  }

  .md\:placeholder-indigo-700::-ms-input-placeholder {
    color: #4c51bf;
  }

  .md\:placeholder-indigo-700::placeholder {
    color: #4c51bf;
  }

  .md\:placeholder-indigo-800::-webkit-input-placeholder {
    color: #434190;
  }

  .md\:placeholder-indigo-800::-moz-placeholder {
    color: #434190;
  }

  .md\:placeholder-indigo-800:-ms-input-placeholder {
    color: #434190;
  }

  .md\:placeholder-indigo-800::-ms-input-placeholder {
    color: #434190;
  }

  .md\:placeholder-indigo-800::placeholder {
    color: #434190;
  }

  .md\:placeholder-indigo-900::-webkit-input-placeholder {
    color: #3c366b;
  }

  .md\:placeholder-indigo-900::-moz-placeholder {
    color: #3c366b;
  }

  .md\:placeholder-indigo-900:-ms-input-placeholder {
    color: #3c366b;
  }

  .md\:placeholder-indigo-900::-ms-input-placeholder {
    color: #3c366b;
  }

  .md\:placeholder-indigo-900::placeholder {
    color: #3c366b;
  }

  .md\:placeholder-purple-100::-webkit-input-placeholder {
    color: #faf5ff;
  }

  .md\:placeholder-purple-100::-moz-placeholder {
    color: #faf5ff;
  }

  .md\:placeholder-purple-100:-ms-input-placeholder {
    color: #faf5ff;
  }

  .md\:placeholder-purple-100::-ms-input-placeholder {
    color: #faf5ff;
  }

  .md\:placeholder-purple-100::placeholder {
    color: #faf5ff;
  }

  .md\:placeholder-purple-200::-webkit-input-placeholder {
    color: #e9d8fd;
  }

  .md\:placeholder-purple-200::-moz-placeholder {
    color: #e9d8fd;
  }

  .md\:placeholder-purple-200:-ms-input-placeholder {
    color: #e9d8fd;
  }

  .md\:placeholder-purple-200::-ms-input-placeholder {
    color: #e9d8fd;
  }

  .md\:placeholder-purple-200::placeholder {
    color: #e9d8fd;
  }

  .md\:placeholder-purple-300::-webkit-input-placeholder {
    color: #d6bcfa;
  }

  .md\:placeholder-purple-300::-moz-placeholder {
    color: #d6bcfa;
  }

  .md\:placeholder-purple-300:-ms-input-placeholder {
    color: #d6bcfa;
  }

  .md\:placeholder-purple-300::-ms-input-placeholder {
    color: #d6bcfa;
  }

  .md\:placeholder-purple-300::placeholder {
    color: #d6bcfa;
  }

  .md\:placeholder-purple-400::-webkit-input-placeholder {
    color: #b794f4;
  }

  .md\:placeholder-purple-400::-moz-placeholder {
    color: #b794f4;
  }

  .md\:placeholder-purple-400:-ms-input-placeholder {
    color: #b794f4;
  }

  .md\:placeholder-purple-400::-ms-input-placeholder {
    color: #b794f4;
  }

  .md\:placeholder-purple-400::placeholder {
    color: #b794f4;
  }

  .md\:placeholder-purple-500::-webkit-input-placeholder {
    color: #9f7aea;
  }

  .md\:placeholder-purple-500::-moz-placeholder {
    color: #9f7aea;
  }

  .md\:placeholder-purple-500:-ms-input-placeholder {
    color: #9f7aea;
  }

  .md\:placeholder-purple-500::-ms-input-placeholder {
    color: #9f7aea;
  }

  .md\:placeholder-purple-500::placeholder {
    color: #9f7aea;
  }

  .md\:placeholder-purple-600::-webkit-input-placeholder {
    color: #805ad5;
  }

  .md\:placeholder-purple-600::-moz-placeholder {
    color: #805ad5;
  }

  .md\:placeholder-purple-600:-ms-input-placeholder {
    color: #805ad5;
  }

  .md\:placeholder-purple-600::-ms-input-placeholder {
    color: #805ad5;
  }

  .md\:placeholder-purple-600::placeholder {
    color: #805ad5;
  }

  .md\:placeholder-purple-700::-webkit-input-placeholder {
    color: #6b46c1;
  }

  .md\:placeholder-purple-700::-moz-placeholder {
    color: #6b46c1;
  }

  .md\:placeholder-purple-700:-ms-input-placeholder {
    color: #6b46c1;
  }

  .md\:placeholder-purple-700::-ms-input-placeholder {
    color: #6b46c1;
  }

  .md\:placeholder-purple-700::placeholder {
    color: #6b46c1;
  }

  .md\:placeholder-purple-800::-webkit-input-placeholder {
    color: #553c9a;
  }

  .md\:placeholder-purple-800::-moz-placeholder {
    color: #553c9a;
  }

  .md\:placeholder-purple-800:-ms-input-placeholder {
    color: #553c9a;
  }

  .md\:placeholder-purple-800::-ms-input-placeholder {
    color: #553c9a;
  }

  .md\:placeholder-purple-800::placeholder {
    color: #553c9a;
  }

  .md\:placeholder-purple-900::-webkit-input-placeholder {
    color: #44337a;
  }

  .md\:placeholder-purple-900::-moz-placeholder {
    color: #44337a;
  }

  .md\:placeholder-purple-900:-ms-input-placeholder {
    color: #44337a;
  }

  .md\:placeholder-purple-900::-ms-input-placeholder {
    color: #44337a;
  }

  .md\:placeholder-purple-900::placeholder {
    color: #44337a;
  }

  .md\:placeholder-pink-100::-webkit-input-placeholder {
    color: #fff5f7;
  }

  .md\:placeholder-pink-100::-moz-placeholder {
    color: #fff5f7;
  }

  .md\:placeholder-pink-100:-ms-input-placeholder {
    color: #fff5f7;
  }

  .md\:placeholder-pink-100::-ms-input-placeholder {
    color: #fff5f7;
  }

  .md\:placeholder-pink-100::placeholder {
    color: #fff5f7;
  }

  .md\:placeholder-pink-200::-webkit-input-placeholder {
    color: #fed7e2;
  }

  .md\:placeholder-pink-200::-moz-placeholder {
    color: #fed7e2;
  }

  .md\:placeholder-pink-200:-ms-input-placeholder {
    color: #fed7e2;
  }

  .md\:placeholder-pink-200::-ms-input-placeholder {
    color: #fed7e2;
  }

  .md\:placeholder-pink-200::placeholder {
    color: #fed7e2;
  }

  .md\:placeholder-pink-300::-webkit-input-placeholder {
    color: #fbb6ce;
  }

  .md\:placeholder-pink-300::-moz-placeholder {
    color: #fbb6ce;
  }

  .md\:placeholder-pink-300:-ms-input-placeholder {
    color: #fbb6ce;
  }

  .md\:placeholder-pink-300::-ms-input-placeholder {
    color: #fbb6ce;
  }

  .md\:placeholder-pink-300::placeholder {
    color: #fbb6ce;
  }

  .md\:placeholder-pink-400::-webkit-input-placeholder {
    color: #f687b3;
  }

  .md\:placeholder-pink-400::-moz-placeholder {
    color: #f687b3;
  }

  .md\:placeholder-pink-400:-ms-input-placeholder {
    color: #f687b3;
  }

  .md\:placeholder-pink-400::-ms-input-placeholder {
    color: #f687b3;
  }

  .md\:placeholder-pink-400::placeholder {
    color: #f687b3;
  }

  .md\:placeholder-pink-500::-webkit-input-placeholder {
    color: #ed64a6;
  }

  .md\:placeholder-pink-500::-moz-placeholder {
    color: #ed64a6;
  }

  .md\:placeholder-pink-500:-ms-input-placeholder {
    color: #ed64a6;
  }

  .md\:placeholder-pink-500::-ms-input-placeholder {
    color: #ed64a6;
  }

  .md\:placeholder-pink-500::placeholder {
    color: #ed64a6;
  }

  .md\:placeholder-pink-600::-webkit-input-placeholder {
    color: #d53f8c;
  }

  .md\:placeholder-pink-600::-moz-placeholder {
    color: #d53f8c;
  }

  .md\:placeholder-pink-600:-ms-input-placeholder {
    color: #d53f8c;
  }

  .md\:placeholder-pink-600::-ms-input-placeholder {
    color: #d53f8c;
  }

  .md\:placeholder-pink-600::placeholder {
    color: #d53f8c;
  }

  .md\:placeholder-pink-700::-webkit-input-placeholder {
    color: #b83280;
  }

  .md\:placeholder-pink-700::-moz-placeholder {
    color: #b83280;
  }

  .md\:placeholder-pink-700:-ms-input-placeholder {
    color: #b83280;
  }

  .md\:placeholder-pink-700::-ms-input-placeholder {
    color: #b83280;
  }

  .md\:placeholder-pink-700::placeholder {
    color: #b83280;
  }

  .md\:placeholder-pink-800::-webkit-input-placeholder {
    color: #97266d;
  }

  .md\:placeholder-pink-800::-moz-placeholder {
    color: #97266d;
  }

  .md\:placeholder-pink-800:-ms-input-placeholder {
    color: #97266d;
  }

  .md\:placeholder-pink-800::-ms-input-placeholder {
    color: #97266d;
  }

  .md\:placeholder-pink-800::placeholder {
    color: #97266d;
  }

  .md\:placeholder-pink-900::-webkit-input-placeholder {
    color: #702459;
  }

  .md\:placeholder-pink-900::-moz-placeholder {
    color: #702459;
  }

  .md\:placeholder-pink-900:-ms-input-placeholder {
    color: #702459;
  }

  .md\:placeholder-pink-900::-ms-input-placeholder {
    color: #702459;
  }

  .md\:placeholder-pink-900::placeholder {
    color: #702459;
  }

  .md\:focus\:placeholder-transparent:focus::-webkit-input-placeholder {
    color: transparent;
  }

  .md\:focus\:placeholder-transparent:focus::-moz-placeholder {
    color: transparent;
  }

  .md\:focus\:placeholder-transparent:focus:-ms-input-placeholder {
    color: transparent;
  }

  .md\:focus\:placeholder-transparent:focus::-ms-input-placeholder {
    color: transparent;
  }

  .md\:focus\:placeholder-transparent:focus::placeholder {
    color: transparent;
  }

  .md\:focus\:placeholder-black:focus::-webkit-input-placeholder {
    color: #000;
  }

  .md\:focus\:placeholder-black:focus::-moz-placeholder {
    color: #000;
  }

  .md\:focus\:placeholder-black:focus:-ms-input-placeholder {
    color: #000;
  }

  .md\:focus\:placeholder-black:focus::-ms-input-placeholder {
    color: #000;
  }

  .md\:focus\:placeholder-black:focus::placeholder {
    color: #000;
  }

  .md\:focus\:placeholder-white:focus::-webkit-input-placeholder {
    color: #fff;
  }

  .md\:focus\:placeholder-white:focus::-moz-placeholder {
    color: #fff;
  }

  .md\:focus\:placeholder-white:focus:-ms-input-placeholder {
    color: #fff;
  }

  .md\:focus\:placeholder-white:focus::-ms-input-placeholder {
    color: #fff;
  }

  .md\:focus\:placeholder-white:focus::placeholder {
    color: #fff;
  }

  .md\:focus\:placeholder-gray-100:focus::-webkit-input-placeholder {
    color: #f7fafc;
  }

  .md\:focus\:placeholder-gray-100:focus::-moz-placeholder {
    color: #f7fafc;
  }

  .md\:focus\:placeholder-gray-100:focus:-ms-input-placeholder {
    color: #f7fafc;
  }

  .md\:focus\:placeholder-gray-100:focus::-ms-input-placeholder {
    color: #f7fafc;
  }

  .md\:focus\:placeholder-gray-100:focus::placeholder {
    color: #f7fafc;
  }

  .md\:focus\:placeholder-gray-200:focus::-webkit-input-placeholder {
    color: #edf2f7;
  }

  .md\:focus\:placeholder-gray-200:focus::-moz-placeholder {
    color: #edf2f7;
  }

  .md\:focus\:placeholder-gray-200:focus:-ms-input-placeholder {
    color: #edf2f7;
  }

  .md\:focus\:placeholder-gray-200:focus::-ms-input-placeholder {
    color: #edf2f7;
  }

  .md\:focus\:placeholder-gray-200:focus::placeholder {
    color: #edf2f7;
  }

  .md\:focus\:placeholder-gray-300:focus::-webkit-input-placeholder {
    color: #e2e8f0;
  }

  .md\:focus\:placeholder-gray-300:focus::-moz-placeholder {
    color: #e2e8f0;
  }

  .md\:focus\:placeholder-gray-300:focus:-ms-input-placeholder {
    color: #e2e8f0;
  }

  .md\:focus\:placeholder-gray-300:focus::-ms-input-placeholder {
    color: #e2e8f0;
  }

  .md\:focus\:placeholder-gray-300:focus::placeholder {
    color: #e2e8f0;
  }

  .md\:focus\:placeholder-gray-400:focus::-webkit-input-placeholder {
    color: #cbd5e0;
  }

  .md\:focus\:placeholder-gray-400:focus::-moz-placeholder {
    color: #cbd5e0;
  }

  .md\:focus\:placeholder-gray-400:focus:-ms-input-placeholder {
    color: #cbd5e0;
  }

  .md\:focus\:placeholder-gray-400:focus::-ms-input-placeholder {
    color: #cbd5e0;
  }

  .md\:focus\:placeholder-gray-400:focus::placeholder {
    color: #cbd5e0;
  }

  .md\:focus\:placeholder-gray-500:focus::-webkit-input-placeholder {
    color: #a0aec0;
  }

  .md\:focus\:placeholder-gray-500:focus::-moz-placeholder {
    color: #a0aec0;
  }

  .md\:focus\:placeholder-gray-500:focus:-ms-input-placeholder {
    color: #a0aec0;
  }

  .md\:focus\:placeholder-gray-500:focus::-ms-input-placeholder {
    color: #a0aec0;
  }

  .md\:focus\:placeholder-gray-500:focus::placeholder {
    color: #a0aec0;
  }

  .md\:focus\:placeholder-gray-600:focus::-webkit-input-placeholder {
    color: #718096;
  }

  .md\:focus\:placeholder-gray-600:focus::-moz-placeholder {
    color: #718096;
  }

  .md\:focus\:placeholder-gray-600:focus:-ms-input-placeholder {
    color: #718096;
  }

  .md\:focus\:placeholder-gray-600:focus::-ms-input-placeholder {
    color: #718096;
  }

  .md\:focus\:placeholder-gray-600:focus::placeholder {
    color: #718096;
  }

  .md\:focus\:placeholder-gray-700:focus::-webkit-input-placeholder {
    color: #4a5568;
  }

  .md\:focus\:placeholder-gray-700:focus::-moz-placeholder {
    color: #4a5568;
  }

  .md\:focus\:placeholder-gray-700:focus:-ms-input-placeholder {
    color: #4a5568;
  }

  .md\:focus\:placeholder-gray-700:focus::-ms-input-placeholder {
    color: #4a5568;
  }

  .md\:focus\:placeholder-gray-700:focus::placeholder {
    color: #4a5568;
  }

  .md\:focus\:placeholder-gray-800:focus::-webkit-input-placeholder {
    color: #2d3748;
  }

  .md\:focus\:placeholder-gray-800:focus::-moz-placeholder {
    color: #2d3748;
  }

  .md\:focus\:placeholder-gray-800:focus:-ms-input-placeholder {
    color: #2d3748;
  }

  .md\:focus\:placeholder-gray-800:focus::-ms-input-placeholder {
    color: #2d3748;
  }

  .md\:focus\:placeholder-gray-800:focus::placeholder {
    color: #2d3748;
  }

  .md\:focus\:placeholder-gray-900:focus::-webkit-input-placeholder {
    color: #1a202c;
  }

  .md\:focus\:placeholder-gray-900:focus::-moz-placeholder {
    color: #1a202c;
  }

  .md\:focus\:placeholder-gray-900:focus:-ms-input-placeholder {
    color: #1a202c;
  }

  .md\:focus\:placeholder-gray-900:focus::-ms-input-placeholder {
    color: #1a202c;
  }

  .md\:focus\:placeholder-gray-900:focus::placeholder {
    color: #1a202c;
  }

  .md\:focus\:placeholder-red-100:focus::-webkit-input-placeholder {
    color: #fff5f5;
  }

  .md\:focus\:placeholder-red-100:focus::-moz-placeholder {
    color: #fff5f5;
  }

  .md\:focus\:placeholder-red-100:focus:-ms-input-placeholder {
    color: #fff5f5;
  }

  .md\:focus\:placeholder-red-100:focus::-ms-input-placeholder {
    color: #fff5f5;
  }

  .md\:focus\:placeholder-red-100:focus::placeholder {
    color: #fff5f5;
  }

  .md\:focus\:placeholder-red-200:focus::-webkit-input-placeholder {
    color: #fed7d7;
  }

  .md\:focus\:placeholder-red-200:focus::-moz-placeholder {
    color: #fed7d7;
  }

  .md\:focus\:placeholder-red-200:focus:-ms-input-placeholder {
    color: #fed7d7;
  }

  .md\:focus\:placeholder-red-200:focus::-ms-input-placeholder {
    color: #fed7d7;
  }

  .md\:focus\:placeholder-red-200:focus::placeholder {
    color: #fed7d7;
  }

  .md\:focus\:placeholder-red-300:focus::-webkit-input-placeholder {
    color: #feb2b2;
  }

  .md\:focus\:placeholder-red-300:focus::-moz-placeholder {
    color: #feb2b2;
  }

  .md\:focus\:placeholder-red-300:focus:-ms-input-placeholder {
    color: #feb2b2;
  }

  .md\:focus\:placeholder-red-300:focus::-ms-input-placeholder {
    color: #feb2b2;
  }

  .md\:focus\:placeholder-red-300:focus::placeholder {
    color: #feb2b2;
  }

  .md\:focus\:placeholder-red-400:focus::-webkit-input-placeholder {
    color: #fc8181;
  }

  .md\:focus\:placeholder-red-400:focus::-moz-placeholder {
    color: #fc8181;
  }

  .md\:focus\:placeholder-red-400:focus:-ms-input-placeholder {
    color: #fc8181;
  }

  .md\:focus\:placeholder-red-400:focus::-ms-input-placeholder {
    color: #fc8181;
  }

  .md\:focus\:placeholder-red-400:focus::placeholder {
    color: #fc8181;
  }

  .md\:focus\:placeholder-red-500:focus::-webkit-input-placeholder {
    color: #f56565;
  }

  .md\:focus\:placeholder-red-500:focus::-moz-placeholder {
    color: #f56565;
  }

  .md\:focus\:placeholder-red-500:focus:-ms-input-placeholder {
    color: #f56565;
  }

  .md\:focus\:placeholder-red-500:focus::-ms-input-placeholder {
    color: #f56565;
  }

  .md\:focus\:placeholder-red-500:focus::placeholder {
    color: #f56565;
  }

  .md\:focus\:placeholder-red-600:focus::-webkit-input-placeholder {
    color: #e53e3e;
  }

  .md\:focus\:placeholder-red-600:focus::-moz-placeholder {
    color: #e53e3e;
  }

  .md\:focus\:placeholder-red-600:focus:-ms-input-placeholder {
    color: #e53e3e;
  }

  .md\:focus\:placeholder-red-600:focus::-ms-input-placeholder {
    color: #e53e3e;
  }

  .md\:focus\:placeholder-red-600:focus::placeholder {
    color: #e53e3e;
  }

  .md\:focus\:placeholder-red-700:focus::-webkit-input-placeholder {
    color: #c53030;
  }

  .md\:focus\:placeholder-red-700:focus::-moz-placeholder {
    color: #c53030;
  }

  .md\:focus\:placeholder-red-700:focus:-ms-input-placeholder {
    color: #c53030;
  }

  .md\:focus\:placeholder-red-700:focus::-ms-input-placeholder {
    color: #c53030;
  }

  .md\:focus\:placeholder-red-700:focus::placeholder {
    color: #c53030;
  }

  .md\:focus\:placeholder-red-800:focus::-webkit-input-placeholder {
    color: #9b2c2c;
  }

  .md\:focus\:placeholder-red-800:focus::-moz-placeholder {
    color: #9b2c2c;
  }

  .md\:focus\:placeholder-red-800:focus:-ms-input-placeholder {
    color: #9b2c2c;
  }

  .md\:focus\:placeholder-red-800:focus::-ms-input-placeholder {
    color: #9b2c2c;
  }

  .md\:focus\:placeholder-red-800:focus::placeholder {
    color: #9b2c2c;
  }

  .md\:focus\:placeholder-red-900:focus::-webkit-input-placeholder {
    color: #742a2a;
  }

  .md\:focus\:placeholder-red-900:focus::-moz-placeholder {
    color: #742a2a;
  }

  .md\:focus\:placeholder-red-900:focus:-ms-input-placeholder {
    color: #742a2a;
  }

  .md\:focus\:placeholder-red-900:focus::-ms-input-placeholder {
    color: #742a2a;
  }

  .md\:focus\:placeholder-red-900:focus::placeholder {
    color: #742a2a;
  }

  .md\:focus\:placeholder-orange-100:focus::-webkit-input-placeholder {
    color: #fffaf0;
  }

  .md\:focus\:placeholder-orange-100:focus::-moz-placeholder {
    color: #fffaf0;
  }

  .md\:focus\:placeholder-orange-100:focus:-ms-input-placeholder {
    color: #fffaf0;
  }

  .md\:focus\:placeholder-orange-100:focus::-ms-input-placeholder {
    color: #fffaf0;
  }

  .md\:focus\:placeholder-orange-100:focus::placeholder {
    color: #fffaf0;
  }

  .md\:focus\:placeholder-orange-200:focus::-webkit-input-placeholder {
    color: #feebc8;
  }

  .md\:focus\:placeholder-orange-200:focus::-moz-placeholder {
    color: #feebc8;
  }

  .md\:focus\:placeholder-orange-200:focus:-ms-input-placeholder {
    color: #feebc8;
  }

  .md\:focus\:placeholder-orange-200:focus::-ms-input-placeholder {
    color: #feebc8;
  }

  .md\:focus\:placeholder-orange-200:focus::placeholder {
    color: #feebc8;
  }

  .md\:focus\:placeholder-orange-300:focus::-webkit-input-placeholder {
    color: #fbd38d;
  }

  .md\:focus\:placeholder-orange-300:focus::-moz-placeholder {
    color: #fbd38d;
  }

  .md\:focus\:placeholder-orange-300:focus:-ms-input-placeholder {
    color: #fbd38d;
  }

  .md\:focus\:placeholder-orange-300:focus::-ms-input-placeholder {
    color: #fbd38d;
  }

  .md\:focus\:placeholder-orange-300:focus::placeholder {
    color: #fbd38d;
  }

  .md\:focus\:placeholder-orange-400:focus::-webkit-input-placeholder {
    color: #f6ad55;
  }

  .md\:focus\:placeholder-orange-400:focus::-moz-placeholder {
    color: #f6ad55;
  }

  .md\:focus\:placeholder-orange-400:focus:-ms-input-placeholder {
    color: #f6ad55;
  }

  .md\:focus\:placeholder-orange-400:focus::-ms-input-placeholder {
    color: #f6ad55;
  }

  .md\:focus\:placeholder-orange-400:focus::placeholder {
    color: #f6ad55;
  }

  .md\:focus\:placeholder-orange-500:focus::-webkit-input-placeholder {
    color: #ed8936;
  }

  .md\:focus\:placeholder-orange-500:focus::-moz-placeholder {
    color: #ed8936;
  }

  .md\:focus\:placeholder-orange-500:focus:-ms-input-placeholder {
    color: #ed8936;
  }

  .md\:focus\:placeholder-orange-500:focus::-ms-input-placeholder {
    color: #ed8936;
  }

  .md\:focus\:placeholder-orange-500:focus::placeholder {
    color: #ed8936;
  }

  .md\:focus\:placeholder-orange-600:focus::-webkit-input-placeholder {
    color: #dd6b20;
  }

  .md\:focus\:placeholder-orange-600:focus::-moz-placeholder {
    color: #dd6b20;
  }

  .md\:focus\:placeholder-orange-600:focus:-ms-input-placeholder {
    color: #dd6b20;
  }

  .md\:focus\:placeholder-orange-600:focus::-ms-input-placeholder {
    color: #dd6b20;
  }

  .md\:focus\:placeholder-orange-600:focus::placeholder {
    color: #dd6b20;
  }

  .md\:focus\:placeholder-orange-700:focus::-webkit-input-placeholder {
    color: #c05621;
  }

  .md\:focus\:placeholder-orange-700:focus::-moz-placeholder {
    color: #c05621;
  }

  .md\:focus\:placeholder-orange-700:focus:-ms-input-placeholder {
    color: #c05621;
  }

  .md\:focus\:placeholder-orange-700:focus::-ms-input-placeholder {
    color: #c05621;
  }

  .md\:focus\:placeholder-orange-700:focus::placeholder {
    color: #c05621;
  }

  .md\:focus\:placeholder-orange-800:focus::-webkit-input-placeholder {
    color: #9c4221;
  }

  .md\:focus\:placeholder-orange-800:focus::-moz-placeholder {
    color: #9c4221;
  }

  .md\:focus\:placeholder-orange-800:focus:-ms-input-placeholder {
    color: #9c4221;
  }

  .md\:focus\:placeholder-orange-800:focus::-ms-input-placeholder {
    color: #9c4221;
  }

  .md\:focus\:placeholder-orange-800:focus::placeholder {
    color: #9c4221;
  }

  .md\:focus\:placeholder-orange-900:focus::-webkit-input-placeholder {
    color: #7b341e;
  }

  .md\:focus\:placeholder-orange-900:focus::-moz-placeholder {
    color: #7b341e;
  }

  .md\:focus\:placeholder-orange-900:focus:-ms-input-placeholder {
    color: #7b341e;
  }

  .md\:focus\:placeholder-orange-900:focus::-ms-input-placeholder {
    color: #7b341e;
  }

  .md\:focus\:placeholder-orange-900:focus::placeholder {
    color: #7b341e;
  }

  .md\:focus\:placeholder-yellow-100:focus::-webkit-input-placeholder {
    color: #fffff0;
  }

  .md\:focus\:placeholder-yellow-100:focus::-moz-placeholder {
    color: #fffff0;
  }

  .md\:focus\:placeholder-yellow-100:focus:-ms-input-placeholder {
    color: #fffff0;
  }

  .md\:focus\:placeholder-yellow-100:focus::-ms-input-placeholder {
    color: #fffff0;
  }

  .md\:focus\:placeholder-yellow-100:focus::placeholder {
    color: #fffff0;
  }

  .md\:focus\:placeholder-yellow-200:focus::-webkit-input-placeholder {
    color: #fefcbf;
  }

  .md\:focus\:placeholder-yellow-200:focus::-moz-placeholder {
    color: #fefcbf;
  }

  .md\:focus\:placeholder-yellow-200:focus:-ms-input-placeholder {
    color: #fefcbf;
  }

  .md\:focus\:placeholder-yellow-200:focus::-ms-input-placeholder {
    color: #fefcbf;
  }

  .md\:focus\:placeholder-yellow-200:focus::placeholder {
    color: #fefcbf;
  }

  .md\:focus\:placeholder-yellow-300:focus::-webkit-input-placeholder {
    color: #faf089;
  }

  .md\:focus\:placeholder-yellow-300:focus::-moz-placeholder {
    color: #faf089;
  }

  .md\:focus\:placeholder-yellow-300:focus:-ms-input-placeholder {
    color: #faf089;
  }

  .md\:focus\:placeholder-yellow-300:focus::-ms-input-placeholder {
    color: #faf089;
  }

  .md\:focus\:placeholder-yellow-300:focus::placeholder {
    color: #faf089;
  }

  .md\:focus\:placeholder-yellow-400:focus::-webkit-input-placeholder {
    color: #f6e05e;
  }

  .md\:focus\:placeholder-yellow-400:focus::-moz-placeholder {
    color: #f6e05e;
  }

  .md\:focus\:placeholder-yellow-400:focus:-ms-input-placeholder {
    color: #f6e05e;
  }

  .md\:focus\:placeholder-yellow-400:focus::-ms-input-placeholder {
    color: #f6e05e;
  }

  .md\:focus\:placeholder-yellow-400:focus::placeholder {
    color: #f6e05e;
  }

  .md\:focus\:placeholder-yellow-500:focus::-webkit-input-placeholder {
    color: #ecc94b;
  }

  .md\:focus\:placeholder-yellow-500:focus::-moz-placeholder {
    color: #ecc94b;
  }

  .md\:focus\:placeholder-yellow-500:focus:-ms-input-placeholder {
    color: #ecc94b;
  }

  .md\:focus\:placeholder-yellow-500:focus::-ms-input-placeholder {
    color: #ecc94b;
  }

  .md\:focus\:placeholder-yellow-500:focus::placeholder {
    color: #ecc94b;
  }

  .md\:focus\:placeholder-yellow-600:focus::-webkit-input-placeholder {
    color: #d69e2e;
  }

  .md\:focus\:placeholder-yellow-600:focus::-moz-placeholder {
    color: #d69e2e;
  }

  .md\:focus\:placeholder-yellow-600:focus:-ms-input-placeholder {
    color: #d69e2e;
  }

  .md\:focus\:placeholder-yellow-600:focus::-ms-input-placeholder {
    color: #d69e2e;
  }

  .md\:focus\:placeholder-yellow-600:focus::placeholder {
    color: #d69e2e;
  }

  .md\:focus\:placeholder-yellow-700:focus::-webkit-input-placeholder {
    color: #b7791f;
  }

  .md\:focus\:placeholder-yellow-700:focus::-moz-placeholder {
    color: #b7791f;
  }

  .md\:focus\:placeholder-yellow-700:focus:-ms-input-placeholder {
    color: #b7791f;
  }

  .md\:focus\:placeholder-yellow-700:focus::-ms-input-placeholder {
    color: #b7791f;
  }

  .md\:focus\:placeholder-yellow-700:focus::placeholder {
    color: #b7791f;
  }

  .md\:focus\:placeholder-yellow-800:focus::-webkit-input-placeholder {
    color: #975a16;
  }

  .md\:focus\:placeholder-yellow-800:focus::-moz-placeholder {
    color: #975a16;
  }

  .md\:focus\:placeholder-yellow-800:focus:-ms-input-placeholder {
    color: #975a16;
  }

  .md\:focus\:placeholder-yellow-800:focus::-ms-input-placeholder {
    color: #975a16;
  }

  .md\:focus\:placeholder-yellow-800:focus::placeholder {
    color: #975a16;
  }

  .md\:focus\:placeholder-yellow-900:focus::-webkit-input-placeholder {
    color: #744210;
  }

  .md\:focus\:placeholder-yellow-900:focus::-moz-placeholder {
    color: #744210;
  }

  .md\:focus\:placeholder-yellow-900:focus:-ms-input-placeholder {
    color: #744210;
  }

  .md\:focus\:placeholder-yellow-900:focus::-ms-input-placeholder {
    color: #744210;
  }

  .md\:focus\:placeholder-yellow-900:focus::placeholder {
    color: #744210;
  }

  .md\:focus\:placeholder-green-100:focus::-webkit-input-placeholder {
    color: #f0fff4;
  }

  .md\:focus\:placeholder-green-100:focus::-moz-placeholder {
    color: #f0fff4;
  }

  .md\:focus\:placeholder-green-100:focus:-ms-input-placeholder {
    color: #f0fff4;
  }

  .md\:focus\:placeholder-green-100:focus::-ms-input-placeholder {
    color: #f0fff4;
  }

  .md\:focus\:placeholder-green-100:focus::placeholder {
    color: #f0fff4;
  }

  .md\:focus\:placeholder-green-200:focus::-webkit-input-placeholder {
    color: #c6f6d5;
  }

  .md\:focus\:placeholder-green-200:focus::-moz-placeholder {
    color: #c6f6d5;
  }

  .md\:focus\:placeholder-green-200:focus:-ms-input-placeholder {
    color: #c6f6d5;
  }

  .md\:focus\:placeholder-green-200:focus::-ms-input-placeholder {
    color: #c6f6d5;
  }

  .md\:focus\:placeholder-green-200:focus::placeholder {
    color: #c6f6d5;
  }

  .md\:focus\:placeholder-green-300:focus::-webkit-input-placeholder {
    color: #9ae6b4;
  }

  .md\:focus\:placeholder-green-300:focus::-moz-placeholder {
    color: #9ae6b4;
  }

  .md\:focus\:placeholder-green-300:focus:-ms-input-placeholder {
    color: #9ae6b4;
  }

  .md\:focus\:placeholder-green-300:focus::-ms-input-placeholder {
    color: #9ae6b4;
  }

  .md\:focus\:placeholder-green-300:focus::placeholder {
    color: #9ae6b4;
  }

  .md\:focus\:placeholder-green-400:focus::-webkit-input-placeholder {
    color: #68d391;
  }

  .md\:focus\:placeholder-green-400:focus::-moz-placeholder {
    color: #68d391;
  }

  .md\:focus\:placeholder-green-400:focus:-ms-input-placeholder {
    color: #68d391;
  }

  .md\:focus\:placeholder-green-400:focus::-ms-input-placeholder {
    color: #68d391;
  }

  .md\:focus\:placeholder-green-400:focus::placeholder {
    color: #68d391;
  }

  .md\:focus\:placeholder-green-500:focus::-webkit-input-placeholder {
    color: #48bb78;
  }

  .md\:focus\:placeholder-green-500:focus::-moz-placeholder {
    color: #48bb78;
  }

  .md\:focus\:placeholder-green-500:focus:-ms-input-placeholder {
    color: #48bb78;
  }

  .md\:focus\:placeholder-green-500:focus::-ms-input-placeholder {
    color: #48bb78;
  }

  .md\:focus\:placeholder-green-500:focus::placeholder {
    color: #48bb78;
  }

  .md\:focus\:placeholder-green-600:focus::-webkit-input-placeholder {
    color: #38a169;
  }

  .md\:focus\:placeholder-green-600:focus::-moz-placeholder {
    color: #38a169;
  }

  .md\:focus\:placeholder-green-600:focus:-ms-input-placeholder {
    color: #38a169;
  }

  .md\:focus\:placeholder-green-600:focus::-ms-input-placeholder {
    color: #38a169;
  }

  .md\:focus\:placeholder-green-600:focus::placeholder {
    color: #38a169;
  }

  .md\:focus\:placeholder-green-700:focus::-webkit-input-placeholder {
    color: #2f855a;
  }

  .md\:focus\:placeholder-green-700:focus::-moz-placeholder {
    color: #2f855a;
  }

  .md\:focus\:placeholder-green-700:focus:-ms-input-placeholder {
    color: #2f855a;
  }

  .md\:focus\:placeholder-green-700:focus::-ms-input-placeholder {
    color: #2f855a;
  }

  .md\:focus\:placeholder-green-700:focus::placeholder {
    color: #2f855a;
  }

  .md\:focus\:placeholder-green-800:focus::-webkit-input-placeholder {
    color: #276749;
  }

  .md\:focus\:placeholder-green-800:focus::-moz-placeholder {
    color: #276749;
  }

  .md\:focus\:placeholder-green-800:focus:-ms-input-placeholder {
    color: #276749;
  }

  .md\:focus\:placeholder-green-800:focus::-ms-input-placeholder {
    color: #276749;
  }

  .md\:focus\:placeholder-green-800:focus::placeholder {
    color: #276749;
  }

  .md\:focus\:placeholder-green-900:focus::-webkit-input-placeholder {
    color: #22543d;
  }

  .md\:focus\:placeholder-green-900:focus::-moz-placeholder {
    color: #22543d;
  }

  .md\:focus\:placeholder-green-900:focus:-ms-input-placeholder {
    color: #22543d;
  }

  .md\:focus\:placeholder-green-900:focus::-ms-input-placeholder {
    color: #22543d;
  }

  .md\:focus\:placeholder-green-900:focus::placeholder {
    color: #22543d;
  }

  .md\:focus\:placeholder-teal-100:focus::-webkit-input-placeholder {
    color: #e6fffa;
  }

  .md\:focus\:placeholder-teal-100:focus::-moz-placeholder {
    color: #e6fffa;
  }

  .md\:focus\:placeholder-teal-100:focus:-ms-input-placeholder {
    color: #e6fffa;
  }

  .md\:focus\:placeholder-teal-100:focus::-ms-input-placeholder {
    color: #e6fffa;
  }

  .md\:focus\:placeholder-teal-100:focus::placeholder {
    color: #e6fffa;
  }

  .md\:focus\:placeholder-teal-200:focus::-webkit-input-placeholder {
    color: #b2f5ea;
  }

  .md\:focus\:placeholder-teal-200:focus::-moz-placeholder {
    color: #b2f5ea;
  }

  .md\:focus\:placeholder-teal-200:focus:-ms-input-placeholder {
    color: #b2f5ea;
  }

  .md\:focus\:placeholder-teal-200:focus::-ms-input-placeholder {
    color: #b2f5ea;
  }

  .md\:focus\:placeholder-teal-200:focus::placeholder {
    color: #b2f5ea;
  }

  .md\:focus\:placeholder-teal-300:focus::-webkit-input-placeholder {
    color: #81e6d9;
  }

  .md\:focus\:placeholder-teal-300:focus::-moz-placeholder {
    color: #81e6d9;
  }

  .md\:focus\:placeholder-teal-300:focus:-ms-input-placeholder {
    color: #81e6d9;
  }

  .md\:focus\:placeholder-teal-300:focus::-ms-input-placeholder {
    color: #81e6d9;
  }

  .md\:focus\:placeholder-teal-300:focus::placeholder {
    color: #81e6d9;
  }

  .md\:focus\:placeholder-teal-400:focus::-webkit-input-placeholder {
    color: #4fd1c5;
  }

  .md\:focus\:placeholder-teal-400:focus::-moz-placeholder {
    color: #4fd1c5;
  }

  .md\:focus\:placeholder-teal-400:focus:-ms-input-placeholder {
    color: #4fd1c5;
  }

  .md\:focus\:placeholder-teal-400:focus::-ms-input-placeholder {
    color: #4fd1c5;
  }

  .md\:focus\:placeholder-teal-400:focus::placeholder {
    color: #4fd1c5;
  }

  .md\:focus\:placeholder-teal-500:focus::-webkit-input-placeholder {
    color: #38b2ac;
  }

  .md\:focus\:placeholder-teal-500:focus::-moz-placeholder {
    color: #38b2ac;
  }

  .md\:focus\:placeholder-teal-500:focus:-ms-input-placeholder {
    color: #38b2ac;
  }

  .md\:focus\:placeholder-teal-500:focus::-ms-input-placeholder {
    color: #38b2ac;
  }

  .md\:focus\:placeholder-teal-500:focus::placeholder {
    color: #38b2ac;
  }

  .md\:focus\:placeholder-teal-600:focus::-webkit-input-placeholder {
    color: #319795;
  }

  .md\:focus\:placeholder-teal-600:focus::-moz-placeholder {
    color: #319795;
  }

  .md\:focus\:placeholder-teal-600:focus:-ms-input-placeholder {
    color: #319795;
  }

  .md\:focus\:placeholder-teal-600:focus::-ms-input-placeholder {
    color: #319795;
  }

  .md\:focus\:placeholder-teal-600:focus::placeholder {
    color: #319795;
  }

  .md\:focus\:placeholder-teal-700:focus::-webkit-input-placeholder {
    color: #2c7a7b;
  }

  .md\:focus\:placeholder-teal-700:focus::-moz-placeholder {
    color: #2c7a7b;
  }

  .md\:focus\:placeholder-teal-700:focus:-ms-input-placeholder {
    color: #2c7a7b;
  }

  .md\:focus\:placeholder-teal-700:focus::-ms-input-placeholder {
    color: #2c7a7b;
  }

  .md\:focus\:placeholder-teal-700:focus::placeholder {
    color: #2c7a7b;
  }

  .md\:focus\:placeholder-teal-800:focus::-webkit-input-placeholder {
    color: #285e61;
  }

  .md\:focus\:placeholder-teal-800:focus::-moz-placeholder {
    color: #285e61;
  }

  .md\:focus\:placeholder-teal-800:focus:-ms-input-placeholder {
    color: #285e61;
  }

  .md\:focus\:placeholder-teal-800:focus::-ms-input-placeholder {
    color: #285e61;
  }

  .md\:focus\:placeholder-teal-800:focus::placeholder {
    color: #285e61;
  }

  .md\:focus\:placeholder-teal-900:focus::-webkit-input-placeholder {
    color: #234e52;
  }

  .md\:focus\:placeholder-teal-900:focus::-moz-placeholder {
    color: #234e52;
  }

  .md\:focus\:placeholder-teal-900:focus:-ms-input-placeholder {
    color: #234e52;
  }

  .md\:focus\:placeholder-teal-900:focus::-ms-input-placeholder {
    color: #234e52;
  }

  .md\:focus\:placeholder-teal-900:focus::placeholder {
    color: #234e52;
  }

  .md\:focus\:placeholder-blue-100:focus::-webkit-input-placeholder {
    color: #ebf8ff;
  }

  .md\:focus\:placeholder-blue-100:focus::-moz-placeholder {
    color: #ebf8ff;
  }

  .md\:focus\:placeholder-blue-100:focus:-ms-input-placeholder {
    color: #ebf8ff;
  }

  .md\:focus\:placeholder-blue-100:focus::-ms-input-placeholder {
    color: #ebf8ff;
  }

  .md\:focus\:placeholder-blue-100:focus::placeholder {
    color: #ebf8ff;
  }

  .md\:focus\:placeholder-blue-200:focus::-webkit-input-placeholder {
    color: #bee3f8;
  }

  .md\:focus\:placeholder-blue-200:focus::-moz-placeholder {
    color: #bee3f8;
  }

  .md\:focus\:placeholder-blue-200:focus:-ms-input-placeholder {
    color: #bee3f8;
  }

  .md\:focus\:placeholder-blue-200:focus::-ms-input-placeholder {
    color: #bee3f8;
  }

  .md\:focus\:placeholder-blue-200:focus::placeholder {
    color: #bee3f8;
  }

  .md\:focus\:placeholder-blue-300:focus::-webkit-input-placeholder {
    color: #90cdf4;
  }

  .md\:focus\:placeholder-blue-300:focus::-moz-placeholder {
    color: #90cdf4;
  }

  .md\:focus\:placeholder-blue-300:focus:-ms-input-placeholder {
    color: #90cdf4;
  }

  .md\:focus\:placeholder-blue-300:focus::-ms-input-placeholder {
    color: #90cdf4;
  }

  .md\:focus\:placeholder-blue-300:focus::placeholder {
    color: #90cdf4;
  }

  .md\:focus\:placeholder-blue-400:focus::-webkit-input-placeholder {
    color: #63b3ed;
  }

  .md\:focus\:placeholder-blue-400:focus::-moz-placeholder {
    color: #63b3ed;
  }

  .md\:focus\:placeholder-blue-400:focus:-ms-input-placeholder {
    color: #63b3ed;
  }

  .md\:focus\:placeholder-blue-400:focus::-ms-input-placeholder {
    color: #63b3ed;
  }

  .md\:focus\:placeholder-blue-400:focus::placeholder {
    color: #63b3ed;
  }

  .md\:focus\:placeholder-blue-500:focus::-webkit-input-placeholder {
    color: #4299e1;
  }

  .md\:focus\:placeholder-blue-500:focus::-moz-placeholder {
    color: #4299e1;
  }

  .md\:focus\:placeholder-blue-500:focus:-ms-input-placeholder {
    color: #4299e1;
  }

  .md\:focus\:placeholder-blue-500:focus::-ms-input-placeholder {
    color: #4299e1;
  }

  .md\:focus\:placeholder-blue-500:focus::placeholder {
    color: #4299e1;
  }

  .md\:focus\:placeholder-blue-600:focus::-webkit-input-placeholder {
    color: #3182ce;
  }

  .md\:focus\:placeholder-blue-600:focus::-moz-placeholder {
    color: #3182ce;
  }

  .md\:focus\:placeholder-blue-600:focus:-ms-input-placeholder {
    color: #3182ce;
  }

  .md\:focus\:placeholder-blue-600:focus::-ms-input-placeholder {
    color: #3182ce;
  }

  .md\:focus\:placeholder-blue-600:focus::placeholder {
    color: #3182ce;
  }

  .md\:focus\:placeholder-blue-700:focus::-webkit-input-placeholder {
    color: #2b6cb0;
  }

  .md\:focus\:placeholder-blue-700:focus::-moz-placeholder {
    color: #2b6cb0;
  }

  .md\:focus\:placeholder-blue-700:focus:-ms-input-placeholder {
    color: #2b6cb0;
  }

  .md\:focus\:placeholder-blue-700:focus::-ms-input-placeholder {
    color: #2b6cb0;
  }

  .md\:focus\:placeholder-blue-700:focus::placeholder {
    color: #2b6cb0;
  }

  .md\:focus\:placeholder-blue-800:focus::-webkit-input-placeholder {
    color: #2c5282;
  }

  .md\:focus\:placeholder-blue-800:focus::-moz-placeholder {
    color: #2c5282;
  }

  .md\:focus\:placeholder-blue-800:focus:-ms-input-placeholder {
    color: #2c5282;
  }

  .md\:focus\:placeholder-blue-800:focus::-ms-input-placeholder {
    color: #2c5282;
  }

  .md\:focus\:placeholder-blue-800:focus::placeholder {
    color: #2c5282;
  }

  .md\:focus\:placeholder-blue-900:focus::-webkit-input-placeholder {
    color: #2a4365;
  }

  .md\:focus\:placeholder-blue-900:focus::-moz-placeholder {
    color: #2a4365;
  }

  .md\:focus\:placeholder-blue-900:focus:-ms-input-placeholder {
    color: #2a4365;
  }

  .md\:focus\:placeholder-blue-900:focus::-ms-input-placeholder {
    color: #2a4365;
  }

  .md\:focus\:placeholder-blue-900:focus::placeholder {
    color: #2a4365;
  }

  .md\:focus\:placeholder-indigo-100:focus::-webkit-input-placeholder {
    color: #ebf4ff;
  }

  .md\:focus\:placeholder-indigo-100:focus::-moz-placeholder {
    color: #ebf4ff;
  }

  .md\:focus\:placeholder-indigo-100:focus:-ms-input-placeholder {
    color: #ebf4ff;
  }

  .md\:focus\:placeholder-indigo-100:focus::-ms-input-placeholder {
    color: #ebf4ff;
  }

  .md\:focus\:placeholder-indigo-100:focus::placeholder {
    color: #ebf4ff;
  }

  .md\:focus\:placeholder-indigo-200:focus::-webkit-input-placeholder {
    color: #c3dafe;
  }

  .md\:focus\:placeholder-indigo-200:focus::-moz-placeholder {
    color: #c3dafe;
  }

  .md\:focus\:placeholder-indigo-200:focus:-ms-input-placeholder {
    color: #c3dafe;
  }

  .md\:focus\:placeholder-indigo-200:focus::-ms-input-placeholder {
    color: #c3dafe;
  }

  .md\:focus\:placeholder-indigo-200:focus::placeholder {
    color: #c3dafe;
  }

  .md\:focus\:placeholder-indigo-300:focus::-webkit-input-placeholder {
    color: #a3bffa;
  }

  .md\:focus\:placeholder-indigo-300:focus::-moz-placeholder {
    color: #a3bffa;
  }

  .md\:focus\:placeholder-indigo-300:focus:-ms-input-placeholder {
    color: #a3bffa;
  }

  .md\:focus\:placeholder-indigo-300:focus::-ms-input-placeholder {
    color: #a3bffa;
  }

  .md\:focus\:placeholder-indigo-300:focus::placeholder {
    color: #a3bffa;
  }

  .md\:focus\:placeholder-indigo-400:focus::-webkit-input-placeholder {
    color: #7f9cf5;
  }

  .md\:focus\:placeholder-indigo-400:focus::-moz-placeholder {
    color: #7f9cf5;
  }

  .md\:focus\:placeholder-indigo-400:focus:-ms-input-placeholder {
    color: #7f9cf5;
  }

  .md\:focus\:placeholder-indigo-400:focus::-ms-input-placeholder {
    color: #7f9cf5;
  }

  .md\:focus\:placeholder-indigo-400:focus::placeholder {
    color: #7f9cf5;
  }

  .md\:focus\:placeholder-indigo-500:focus::-webkit-input-placeholder {
    color: #667eea;
  }

  .md\:focus\:placeholder-indigo-500:focus::-moz-placeholder {
    color: #667eea;
  }

  .md\:focus\:placeholder-indigo-500:focus:-ms-input-placeholder {
    color: #667eea;
  }

  .md\:focus\:placeholder-indigo-500:focus::-ms-input-placeholder {
    color: #667eea;
  }

  .md\:focus\:placeholder-indigo-500:focus::placeholder {
    color: #667eea;
  }

  .md\:focus\:placeholder-indigo-600:focus::-webkit-input-placeholder {
    color: #5a67d8;
  }

  .md\:focus\:placeholder-indigo-600:focus::-moz-placeholder {
    color: #5a67d8;
  }

  .md\:focus\:placeholder-indigo-600:focus:-ms-input-placeholder {
    color: #5a67d8;
  }

  .md\:focus\:placeholder-indigo-600:focus::-ms-input-placeholder {
    color: #5a67d8;
  }

  .md\:focus\:placeholder-indigo-600:focus::placeholder {
    color: #5a67d8;
  }

  .md\:focus\:placeholder-indigo-700:focus::-webkit-input-placeholder {
    color: #4c51bf;
  }

  .md\:focus\:placeholder-indigo-700:focus::-moz-placeholder {
    color: #4c51bf;
  }

  .md\:focus\:placeholder-indigo-700:focus:-ms-input-placeholder {
    color: #4c51bf;
  }

  .md\:focus\:placeholder-indigo-700:focus::-ms-input-placeholder {
    color: #4c51bf;
  }

  .md\:focus\:placeholder-indigo-700:focus::placeholder {
    color: #4c51bf;
  }

  .md\:focus\:placeholder-indigo-800:focus::-webkit-input-placeholder {
    color: #434190;
  }

  .md\:focus\:placeholder-indigo-800:focus::-moz-placeholder {
    color: #434190;
  }

  .md\:focus\:placeholder-indigo-800:focus:-ms-input-placeholder {
    color: #434190;
  }

  .md\:focus\:placeholder-indigo-800:focus::-ms-input-placeholder {
    color: #434190;
  }

  .md\:focus\:placeholder-indigo-800:focus::placeholder {
    color: #434190;
  }

  .md\:focus\:placeholder-indigo-900:focus::-webkit-input-placeholder {
    color: #3c366b;
  }

  .md\:focus\:placeholder-indigo-900:focus::-moz-placeholder {
    color: #3c366b;
  }

  .md\:focus\:placeholder-indigo-900:focus:-ms-input-placeholder {
    color: #3c366b;
  }

  .md\:focus\:placeholder-indigo-900:focus::-ms-input-placeholder {
    color: #3c366b;
  }

  .md\:focus\:placeholder-indigo-900:focus::placeholder {
    color: #3c366b;
  }

  .md\:focus\:placeholder-purple-100:focus::-webkit-input-placeholder {
    color: #faf5ff;
  }

  .md\:focus\:placeholder-purple-100:focus::-moz-placeholder {
    color: #faf5ff;
  }

  .md\:focus\:placeholder-purple-100:focus:-ms-input-placeholder {
    color: #faf5ff;
  }

  .md\:focus\:placeholder-purple-100:focus::-ms-input-placeholder {
    color: #faf5ff;
  }

  .md\:focus\:placeholder-purple-100:focus::placeholder {
    color: #faf5ff;
  }

  .md\:focus\:placeholder-purple-200:focus::-webkit-input-placeholder {
    color: #e9d8fd;
  }

  .md\:focus\:placeholder-purple-200:focus::-moz-placeholder {
    color: #e9d8fd;
  }

  .md\:focus\:placeholder-purple-200:focus:-ms-input-placeholder {
    color: #e9d8fd;
  }

  .md\:focus\:placeholder-purple-200:focus::-ms-input-placeholder {
    color: #e9d8fd;
  }

  .md\:focus\:placeholder-purple-200:focus::placeholder {
    color: #e9d8fd;
  }

  .md\:focus\:placeholder-purple-300:focus::-webkit-input-placeholder {
    color: #d6bcfa;
  }

  .md\:focus\:placeholder-purple-300:focus::-moz-placeholder {
    color: #d6bcfa;
  }

  .md\:focus\:placeholder-purple-300:focus:-ms-input-placeholder {
    color: #d6bcfa;
  }

  .md\:focus\:placeholder-purple-300:focus::-ms-input-placeholder {
    color: #d6bcfa;
  }

  .md\:focus\:placeholder-purple-300:focus::placeholder {
    color: #d6bcfa;
  }

  .md\:focus\:placeholder-purple-400:focus::-webkit-input-placeholder {
    color: #b794f4;
  }

  .md\:focus\:placeholder-purple-400:focus::-moz-placeholder {
    color: #b794f4;
  }

  .md\:focus\:placeholder-purple-400:focus:-ms-input-placeholder {
    color: #b794f4;
  }

  .md\:focus\:placeholder-purple-400:focus::-ms-input-placeholder {
    color: #b794f4;
  }

  .md\:focus\:placeholder-purple-400:focus::placeholder {
    color: #b794f4;
  }

  .md\:focus\:placeholder-purple-500:focus::-webkit-input-placeholder {
    color: #9f7aea;
  }

  .md\:focus\:placeholder-purple-500:focus::-moz-placeholder {
    color: #9f7aea;
  }

  .md\:focus\:placeholder-purple-500:focus:-ms-input-placeholder {
    color: #9f7aea;
  }

  .md\:focus\:placeholder-purple-500:focus::-ms-input-placeholder {
    color: #9f7aea;
  }

  .md\:focus\:placeholder-purple-500:focus::placeholder {
    color: #9f7aea;
  }

  .md\:focus\:placeholder-purple-600:focus::-webkit-input-placeholder {
    color: #805ad5;
  }

  .md\:focus\:placeholder-purple-600:focus::-moz-placeholder {
    color: #805ad5;
  }

  .md\:focus\:placeholder-purple-600:focus:-ms-input-placeholder {
    color: #805ad5;
  }

  .md\:focus\:placeholder-purple-600:focus::-ms-input-placeholder {
    color: #805ad5;
  }

  .md\:focus\:placeholder-purple-600:focus::placeholder {
    color: #805ad5;
  }

  .md\:focus\:placeholder-purple-700:focus::-webkit-input-placeholder {
    color: #6b46c1;
  }

  .md\:focus\:placeholder-purple-700:focus::-moz-placeholder {
    color: #6b46c1;
  }

  .md\:focus\:placeholder-purple-700:focus:-ms-input-placeholder {
    color: #6b46c1;
  }

  .md\:focus\:placeholder-purple-700:focus::-ms-input-placeholder {
    color: #6b46c1;
  }

  .md\:focus\:placeholder-purple-700:focus::placeholder {
    color: #6b46c1;
  }

  .md\:focus\:placeholder-purple-800:focus::-webkit-input-placeholder {
    color: #553c9a;
  }

  .md\:focus\:placeholder-purple-800:focus::-moz-placeholder {
    color: #553c9a;
  }

  .md\:focus\:placeholder-purple-800:focus:-ms-input-placeholder {
    color: #553c9a;
  }

  .md\:focus\:placeholder-purple-800:focus::-ms-input-placeholder {
    color: #553c9a;
  }

  .md\:focus\:placeholder-purple-800:focus::placeholder {
    color: #553c9a;
  }

  .md\:focus\:placeholder-purple-900:focus::-webkit-input-placeholder {
    color: #44337a;
  }

  .md\:focus\:placeholder-purple-900:focus::-moz-placeholder {
    color: #44337a;
  }

  .md\:focus\:placeholder-purple-900:focus:-ms-input-placeholder {
    color: #44337a;
  }

  .md\:focus\:placeholder-purple-900:focus::-ms-input-placeholder {
    color: #44337a;
  }

  .md\:focus\:placeholder-purple-900:focus::placeholder {
    color: #44337a;
  }

  .md\:focus\:placeholder-pink-100:focus::-webkit-input-placeholder {
    color: #fff5f7;
  }

  .md\:focus\:placeholder-pink-100:focus::-moz-placeholder {
    color: #fff5f7;
  }

  .md\:focus\:placeholder-pink-100:focus:-ms-input-placeholder {
    color: #fff5f7;
  }

  .md\:focus\:placeholder-pink-100:focus::-ms-input-placeholder {
    color: #fff5f7;
  }

  .md\:focus\:placeholder-pink-100:focus::placeholder {
    color: #fff5f7;
  }

  .md\:focus\:placeholder-pink-200:focus::-webkit-input-placeholder {
    color: #fed7e2;
  }

  .md\:focus\:placeholder-pink-200:focus::-moz-placeholder {
    color: #fed7e2;
  }

  .md\:focus\:placeholder-pink-200:focus:-ms-input-placeholder {
    color: #fed7e2;
  }

  .md\:focus\:placeholder-pink-200:focus::-ms-input-placeholder {
    color: #fed7e2;
  }

  .md\:focus\:placeholder-pink-200:focus::placeholder {
    color: #fed7e2;
  }

  .md\:focus\:placeholder-pink-300:focus::-webkit-input-placeholder {
    color: #fbb6ce;
  }

  .md\:focus\:placeholder-pink-300:focus::-moz-placeholder {
    color: #fbb6ce;
  }

  .md\:focus\:placeholder-pink-300:focus:-ms-input-placeholder {
    color: #fbb6ce;
  }

  .md\:focus\:placeholder-pink-300:focus::-ms-input-placeholder {
    color: #fbb6ce;
  }

  .md\:focus\:placeholder-pink-300:focus::placeholder {
    color: #fbb6ce;
  }

  .md\:focus\:placeholder-pink-400:focus::-webkit-input-placeholder {
    color: #f687b3;
  }

  .md\:focus\:placeholder-pink-400:focus::-moz-placeholder {
    color: #f687b3;
  }

  .md\:focus\:placeholder-pink-400:focus:-ms-input-placeholder {
    color: #f687b3;
  }

  .md\:focus\:placeholder-pink-400:focus::-ms-input-placeholder {
    color: #f687b3;
  }

  .md\:focus\:placeholder-pink-400:focus::placeholder {
    color: #f687b3;
  }

  .md\:focus\:placeholder-pink-500:focus::-webkit-input-placeholder {
    color: #ed64a6;
  }

  .md\:focus\:placeholder-pink-500:focus::-moz-placeholder {
    color: #ed64a6;
  }

  .md\:focus\:placeholder-pink-500:focus:-ms-input-placeholder {
    color: #ed64a6;
  }

  .md\:focus\:placeholder-pink-500:focus::-ms-input-placeholder {
    color: #ed64a6;
  }

  .md\:focus\:placeholder-pink-500:focus::placeholder {
    color: #ed64a6;
  }

  .md\:focus\:placeholder-pink-600:focus::-webkit-input-placeholder {
    color: #d53f8c;
  }

  .md\:focus\:placeholder-pink-600:focus::-moz-placeholder {
    color: #d53f8c;
  }

  .md\:focus\:placeholder-pink-600:focus:-ms-input-placeholder {
    color: #d53f8c;
  }

  .md\:focus\:placeholder-pink-600:focus::-ms-input-placeholder {
    color: #d53f8c;
  }

  .md\:focus\:placeholder-pink-600:focus::placeholder {
    color: #d53f8c;
  }

  .md\:focus\:placeholder-pink-700:focus::-webkit-input-placeholder {
    color: #b83280;
  }

  .md\:focus\:placeholder-pink-700:focus::-moz-placeholder {
    color: #b83280;
  }

  .md\:focus\:placeholder-pink-700:focus:-ms-input-placeholder {
    color: #b83280;
  }

  .md\:focus\:placeholder-pink-700:focus::-ms-input-placeholder {
    color: #b83280;
  }

  .md\:focus\:placeholder-pink-700:focus::placeholder {
    color: #b83280;
  }

  .md\:focus\:placeholder-pink-800:focus::-webkit-input-placeholder {
    color: #97266d;
  }

  .md\:focus\:placeholder-pink-800:focus::-moz-placeholder {
    color: #97266d;
  }

  .md\:focus\:placeholder-pink-800:focus:-ms-input-placeholder {
    color: #97266d;
  }

  .md\:focus\:placeholder-pink-800:focus::-ms-input-placeholder {
    color: #97266d;
  }

  .md\:focus\:placeholder-pink-800:focus::placeholder {
    color: #97266d;
  }

  .md\:focus\:placeholder-pink-900:focus::-webkit-input-placeholder {
    color: #702459;
  }

  .md\:focus\:placeholder-pink-900:focus::-moz-placeholder {
    color: #702459;
  }

  .md\:focus\:placeholder-pink-900:focus:-ms-input-placeholder {
    color: #702459;
  }

  .md\:focus\:placeholder-pink-900:focus::-ms-input-placeholder {
    color: #702459;
  }

  .md\:focus\:placeholder-pink-900:focus::placeholder {
    color: #702459;
  }

  .md\:pointer-events-none {
    pointer-events: none;
  }

  .md\:pointer-events-auto {
    pointer-events: auto;
  }

  .md\:static {
    position: static;
  }

  .md\:fixed {
    position: fixed;
  }

  .md\:absolute {
    position: absolute;
  }

  .md\:relative {
    position: relative;
  }

  .md\:sticky {
    position: -webkit-sticky;
    position: sticky;
  }

  .md\:inset-0 {
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
  }

  .md\:inset-auto {
    top: auto;
    right: auto;
    bottom: auto;
    left: auto;
  }

  .md\:inset-y-0 {
    top: 0;
    bottom: 0;
  }

  .md\:inset-x-0 {
    right: 0;
    left: 0;
  }

  .md\:inset-y-auto {
    top: auto;
    bottom: auto;
  }

  .md\:inset-x-auto {
    right: auto;
    left: auto;
  }

  .md\:top-0 {
    top: 0;
  }

  .md\:right-0 {
    right: 0;
  }

  .md\:bottom-0 {
    bottom: 0;
  }

  .md\:left-0 {
    left: 0;
  }

  .md\:top-auto {
    top: auto;
  }

  .md\:right-auto {
    right: auto;
  }

  .md\:bottom-auto {
    bottom: auto;
  }

  .md\:left-auto {
    left: auto;
  }

  .md\:resize-none {
    resize: none;
  }

  .md\:resize-y {
    resize: vertical;
  }

  .md\:resize-x {
    resize: horizontal;
  }

  .md\:resize {
    resize: both;
  }

  .md\:shadow {
    box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06);
  }

  .md\:shadow-md {
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
  }

  .md\:shadow-lg {
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
  }

  .md\:shadow-xl {
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
  }

  .md\:shadow-2xl {
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
  }

  .md\:shadow-inner {
    box-shadow: inset 0 2px 4px 0 rgba(0, 0, 0, 0.06);
  }

  .md\:shadow-outline {
    box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.5);
  }

  .md\:shadow-none {
    box-shadow: none;
  }

  .md\:hover\:shadow:hover {
    box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06);
  }

  .md\:hover\:shadow-md:hover {
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
  }

  .md\:hover\:shadow-lg:hover {
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
  }

  .md\:hover\:shadow-xl:hover {
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
  }

  .md\:hover\:shadow-2xl:hover {
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
  }

  .md\:hover\:shadow-inner:hover {
    box-shadow: inset 0 2px 4px 0 rgba(0, 0, 0, 0.06);
  }

  .md\:hover\:shadow-outline:hover {
    box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.5);
  }

  .md\:hover\:shadow-none:hover {
    box-shadow: none;
  }

  .md\:focus\:shadow:focus {
    box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06);
  }

  .md\:focus\:shadow-md:focus {
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
  }

  .md\:focus\:shadow-lg:focus {
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
  }

  .md\:focus\:shadow-xl:focus {
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
  }

  .md\:focus\:shadow-2xl:focus {
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
  }

  .md\:focus\:shadow-inner:focus {
    box-shadow: inset 0 2px 4px 0 rgba(0, 0, 0, 0.06);
  }

  .md\:focus\:shadow-outline:focus {
    box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.5);
  }

  .md\:focus\:shadow-none:focus {
    box-shadow: none;
  }

  .md\:fill-current {
    fill: currentColor;
  }

  .md\:stroke-current {
    stroke: currentColor;
  }

  .md\:table-auto {
    table-layout: auto;
  }

  .md\:table-fixed {
    table-layout: fixed;
  }

  .md\:text-left {
    text-align: left;
  }

  .md\:text-center {
    text-align: center;
  }

  .md\:text-right {
    text-align: right;
  }

  .md\:text-justify {
    text-align: justify;
  }

  .md\:text-transparent {
    color: transparent;
  }

  .md\:text-black {
    color: #000;
  }

  .md\:text-white {
    color: #fff;
  }

  .md\:text-gray-100 {
    color: #f7fafc;
  }

  .md\:text-gray-200 {
    color: #edf2f7;
  }

  .md\:text-gray-300 {
    color: #e2e8f0;
  }

  .md\:text-gray-400 {
    color: #cbd5e0;
  }

  .md\:text-gray-500 {
    color: #a0aec0;
  }

  .md\:text-gray-600 {
    color: #718096;
  }

  .md\:text-gray-700 {
    color: #4a5568;
  }

  .md\:text-gray-800 {
    color: #2d3748;
  }

  .md\:text-gray-900 {
    color: #1a202c;
  }

  .md\:text-red-100 {
    color: #fff5f5;
  }

  .md\:text-red-200 {
    color: #fed7d7;
  }

  .md\:text-red-300 {
    color: #feb2b2;
  }

  .md\:text-red-400 {
    color: #fc8181;
  }

  .md\:text-red-500 {
    color: #f56565;
  }

  .md\:text-red-600 {
    color: #e53e3e;
  }

  .md\:text-red-700 {
    color: #c53030;
  }

  .md\:text-red-800 {
    color: #9b2c2c;
  }

  .md\:text-red-900 {
    color: #742a2a;
  }

  .md\:text-orange-100 {
    color: #fffaf0;
  }

  .md\:text-orange-200 {
    color: #feebc8;
  }

  .md\:text-orange-300 {
    color: #fbd38d;
  }

  .md\:text-orange-400 {
    color: #f6ad55;
  }

  .md\:text-orange-500 {
    color: #ed8936;
  }

  .md\:text-orange-600 {
    color: #dd6b20;
  }

  .md\:text-orange-700 {
    color: #c05621;
  }

  .md\:text-orange-800 {
    color: #9c4221;
  }

  .md\:text-orange-900 {
    color: #7b341e;
  }

  .md\:text-yellow-100 {
    color: #fffff0;
  }

  .md\:text-yellow-200 {
    color: #fefcbf;
  }

  .md\:text-yellow-300 {
    color: #faf089;
  }

  .md\:text-yellow-400 {
    color: #f6e05e;
  }

  .md\:text-yellow-500 {
    color: #ecc94b;
  }

  .md\:text-yellow-600 {
    color: #d69e2e;
  }

  .md\:text-yellow-700 {
    color: #b7791f;
  }

  .md\:text-yellow-800 {
    color: #975a16;
  }

  .md\:text-yellow-900 {
    color: #744210;
  }

  .md\:text-green-100 {
    color: #f0fff4;
  }

  .md\:text-green-200 {
    color: #c6f6d5;
  }

  .md\:text-green-300 {
    color: #9ae6b4;
  }

  .md\:text-green-400 {
    color: #68d391;
  }

  .md\:text-green-500 {
    color: #48bb78;
  }

  .md\:text-green-600 {
    color: #38a169;
  }

  .md\:text-green-700 {
    color: #2f855a;
  }

  .md\:text-green-800 {
    color: #276749;
  }

  .md\:text-green-900 {
    color: #22543d;
  }

  .md\:text-teal-100 {
    color: #e6fffa;
  }

  .md\:text-teal-200 {
    color: #b2f5ea;
  }

  .md\:text-teal-300 {
    color: #81e6d9;
  }

  .md\:text-teal-400 {
    color: #4fd1c5;
  }

  .md\:text-teal-500 {
    color: #38b2ac;
  }

  .md\:text-teal-600 {
    color: #319795;
  }

  .md\:text-teal-700 {
    color: #2c7a7b;
  }

  .md\:text-teal-800 {
    color: #285e61;
  }

  .md\:text-teal-900 {
    color: #234e52;
  }

  .md\:text-blue-100 {
    color: #ebf8ff;
  }

  .md\:text-blue-200 {
    color: #bee3f8;
  }

  .md\:text-blue-300 {
    color: #90cdf4;
  }

  .md\:text-blue-400 {
    color: #63b3ed;
  }

  .md\:text-blue-500 {
    color: #4299e1;
  }

  .md\:text-blue-600 {
    color: #3182ce;
  }

  .md\:text-blue-700 {
    color: #2b6cb0;
  }

  .md\:text-blue-800 {
    color: #2c5282;
  }

  .md\:text-blue-900 {
    color: #2a4365;
  }

  .md\:text-indigo-100 {
    color: #ebf4ff;
  }

  .md\:text-indigo-200 {
    color: #c3dafe;
  }

  .md\:text-indigo-300 {
    color: #a3bffa;
  }

  .md\:text-indigo-400 {
    color: #7f9cf5;
  }

  .md\:text-indigo-500 {
    color: #667eea;
  }

  .md\:text-indigo-600 {
    color: #5a67d8;
  }

  .md\:text-indigo-700 {
    color: #4c51bf;
  }

  .md\:text-indigo-800 {
    color: #434190;
  }

  .md\:text-indigo-900 {
    color: #3c366b;
  }

  .md\:text-purple-100 {
    color: #faf5ff;
  }

  .md\:text-purple-200 {
    color: #e9d8fd;
  }

  .md\:text-purple-300 {
    color: #d6bcfa;
  }

  .md\:text-purple-400 {
    color: #b794f4;
  }

  .md\:text-purple-500 {
    color: #9f7aea;
  }

  .md\:text-purple-600 {
    color: #805ad5;
  }

  .md\:text-purple-700 {
    color: #6b46c1;
  }

  .md\:text-purple-800 {
    color: #553c9a;
  }

  .md\:text-purple-900 {
    color: #44337a;
  }

  .md\:text-pink-100 {
    color: #fff5f7;
  }

  .md\:text-pink-200 {
    color: #fed7e2;
  }

  .md\:text-pink-300 {
    color: #fbb6ce;
  }

  .md\:text-pink-400 {
    color: #f687b3;
  }

  .md\:text-pink-500 {
    color: #ed64a6;
  }

  .md\:text-pink-600 {
    color: #d53f8c;
  }

  .md\:text-pink-700 {
    color: #b83280;
  }

  .md\:text-pink-800 {
    color: #97266d;
  }

  .md\:text-pink-900 {
    color: #702459;
  }

  .md\:hover\:text-transparent:hover {
    color: transparent;
  }

  .md\:hover\:text-black:hover {
    color: #000;
  }

  .md\:hover\:text-white:hover {
    color: #fff;
  }

  .md\:hover\:text-gray-100:hover {
    color: #f7fafc;
  }

  .md\:hover\:text-gray-200:hover {
    color: #edf2f7;
  }

  .md\:hover\:text-gray-300:hover {
    color: #e2e8f0;
  }

  .md\:hover\:text-gray-400:hover {
    color: #cbd5e0;
  }

  .md\:hover\:text-gray-500:hover {
    color: #a0aec0;
  }

  .md\:hover\:text-gray-600:hover {
    color: #718096;
  }

  .md\:hover\:text-gray-700:hover {
    color: #4a5568;
  }

  .md\:hover\:text-gray-800:hover {
    color: #2d3748;
  }

  .md\:hover\:text-gray-900:hover {
    color: #1a202c;
  }

  .md\:hover\:text-red-100:hover {
    color: #fff5f5;
  }

  .md\:hover\:text-red-200:hover {
    color: #fed7d7;
  }

  .md\:hover\:text-red-300:hover {
    color: #feb2b2;
  }

  .md\:hover\:text-red-400:hover {
    color: #fc8181;
  }

  .md\:hover\:text-red-500:hover {
    color: #f56565;
  }

  .md\:hover\:text-red-600:hover {
    color: #e53e3e;
  }

  .md\:hover\:text-red-700:hover {
    color: #c53030;
  }

  .md\:hover\:text-red-800:hover {
    color: #9b2c2c;
  }

  .md\:hover\:text-red-900:hover {
    color: #742a2a;
  }

  .md\:hover\:text-orange-100:hover {
    color: #fffaf0;
  }

  .md\:hover\:text-orange-200:hover {
    color: #feebc8;
  }

  .md\:hover\:text-orange-300:hover {
    color: #fbd38d;
  }

  .md\:hover\:text-orange-400:hover {
    color: #f6ad55;
  }

  .md\:hover\:text-orange-500:hover {
    color: #ed8936;
  }

  .md\:hover\:text-orange-600:hover {
    color: #dd6b20;
  }

  .md\:hover\:text-orange-700:hover {
    color: #c05621;
  }

  .md\:hover\:text-orange-800:hover {
    color: #9c4221;
  }

  .md\:hover\:text-orange-900:hover {
    color: #7b341e;
  }

  .md\:hover\:text-yellow-100:hover {
    color: #fffff0;
  }

  .md\:hover\:text-yellow-200:hover {
    color: #fefcbf;
  }

  .md\:hover\:text-yellow-300:hover {
    color: #faf089;
  }

  .md\:hover\:text-yellow-400:hover {
    color: #f6e05e;
  }

  .md\:hover\:text-yellow-500:hover {
    color: #ecc94b;
  }

  .md\:hover\:text-yellow-600:hover {
    color: #d69e2e;
  }

  .md\:hover\:text-yellow-700:hover {
    color: #b7791f;
  }

  .md\:hover\:text-yellow-800:hover {
    color: #975a16;
  }

  .md\:hover\:text-yellow-900:hover {
    color: #744210;
  }

  .md\:hover\:text-green-100:hover {
    color: #f0fff4;
  }

  .md\:hover\:text-green-200:hover {
    color: #c6f6d5;
  }

  .md\:hover\:text-green-300:hover {
    color: #9ae6b4;
  }

  .md\:hover\:text-green-400:hover {
    color: #68d391;
  }

  .md\:hover\:text-green-500:hover {
    color: #48bb78;
  }

  .md\:hover\:text-green-600:hover {
    color: #38a169;
  }

  .md\:hover\:text-green-700:hover {
    color: #2f855a;
  }

  .md\:hover\:text-green-800:hover {
    color: #276749;
  }

  .md\:hover\:text-green-900:hover {
    color: #22543d;
  }

  .md\:hover\:text-teal-100:hover {
    color: #e6fffa;
  }

  .md\:hover\:text-teal-200:hover {
    color: #b2f5ea;
  }

  .md\:hover\:text-teal-300:hover {
    color: #81e6d9;
  }

  .md\:hover\:text-teal-400:hover {
    color: #4fd1c5;
  }

  .md\:hover\:text-teal-500:hover {
    color: #38b2ac;
  }

  .md\:hover\:text-teal-600:hover {
    color: #319795;
  }

  .md\:hover\:text-teal-700:hover {
    color: #2c7a7b;
  }

  .md\:hover\:text-teal-800:hover {
    color: #285e61;
  }

  .md\:hover\:text-teal-900:hover {
    color: #234e52;
  }

  .md\:hover\:text-blue-100:hover {
    color: #ebf8ff;
  }

  .md\:hover\:text-blue-200:hover {
    color: #bee3f8;
  }

  .md\:hover\:text-blue-300:hover {
    color: #90cdf4;
  }

  .md\:hover\:text-blue-400:hover {
    color: #63b3ed;
  }

  .md\:hover\:text-blue-500:hover {
    color: #4299e1;
  }

  .md\:hover\:text-blue-600:hover {
    color: #3182ce;
  }

  .md\:hover\:text-blue-700:hover {
    color: #2b6cb0;
  }

  .md\:hover\:text-blue-800:hover {
    color: #2c5282;
  }

  .md\:hover\:text-blue-900:hover {
    color: #2a4365;
  }

  .md\:hover\:text-indigo-100:hover {
    color: #ebf4ff;
  }

  .md\:hover\:text-indigo-200:hover {
    color: #c3dafe;
  }

  .md\:hover\:text-indigo-300:hover {
    color: #a3bffa;
  }

  .md\:hover\:text-indigo-400:hover {
    color: #7f9cf5;
  }

  .md\:hover\:text-indigo-500:hover {
    color: #667eea;
  }

  .md\:hover\:text-indigo-600:hover {
    color: #5a67d8;
  }

  .md\:hover\:text-indigo-700:hover {
    color: #4c51bf;
  }

  .md\:hover\:text-indigo-800:hover {
    color: #434190;
  }

  .md\:hover\:text-indigo-900:hover {
    color: #3c366b;
  }

  .md\:hover\:text-purple-100:hover {
    color: #faf5ff;
  }

  .md\:hover\:text-purple-200:hover {
    color: #e9d8fd;
  }

  .md\:hover\:text-purple-300:hover {
    color: #d6bcfa;
  }

  .md\:hover\:text-purple-400:hover {
    color: #b794f4;
  }

  .md\:hover\:text-purple-500:hover {
    color: #9f7aea;
  }

  .md\:hover\:text-purple-600:hover {
    color: #805ad5;
  }

  .md\:hover\:text-purple-700:hover {
    color: #6b46c1;
  }

  .md\:hover\:text-purple-800:hover {
    color: #553c9a;
  }

  .md\:hover\:text-purple-900:hover {
    color: #44337a;
  }

  .md\:hover\:text-pink-100:hover {
    color: #fff5f7;
  }

  .md\:hover\:text-pink-200:hover {
    color: #fed7e2;
  }

  .md\:hover\:text-pink-300:hover {
    color: #fbb6ce;
  }

  .md\:hover\:text-pink-400:hover {
    color: #f687b3;
  }

  .md\:hover\:text-pink-500:hover {
    color: #ed64a6;
  }

  .md\:hover\:text-pink-600:hover {
    color: #d53f8c;
  }

  .md\:hover\:text-pink-700:hover {
    color: #b83280;
  }

  .md\:hover\:text-pink-800:hover {
    color: #97266d;
  }

  .md\:hover\:text-pink-900:hover {
    color: #702459;
  }

  .md\:focus\:text-transparent:focus {
    color: transparent;
  }

  .md\:focus\:text-black:focus {
    color: #000;
  }

  .md\:focus\:text-white:focus {
    color: #fff;
  }

  .md\:focus\:text-gray-100:focus {
    color: #f7fafc;
  }

  .md\:focus\:text-gray-200:focus {
    color: #edf2f7;
  }

  .md\:focus\:text-gray-300:focus {
    color: #e2e8f0;
  }

  .md\:focus\:text-gray-400:focus {
    color: #cbd5e0;
  }

  .md\:focus\:text-gray-500:focus {
    color: #a0aec0;
  }

  .md\:focus\:text-gray-600:focus {
    color: #718096;
  }

  .md\:focus\:text-gray-700:focus {
    color: #4a5568;
  }

  .md\:focus\:text-gray-800:focus {
    color: #2d3748;
  }

  .md\:focus\:text-gray-900:focus {
    color: #1a202c;
  }

  .md\:focus\:text-red-100:focus {
    color: #fff5f5;
  }

  .md\:focus\:text-red-200:focus {
    color: #fed7d7;
  }

  .md\:focus\:text-red-300:focus {
    color: #feb2b2;
  }

  .md\:focus\:text-red-400:focus {
    color: #fc8181;
  }

  .md\:focus\:text-red-500:focus {
    color: #f56565;
  }

  .md\:focus\:text-red-600:focus {
    color: #e53e3e;
  }

  .md\:focus\:text-red-700:focus {
    color: #c53030;
  }

  .md\:focus\:text-red-800:focus {
    color: #9b2c2c;
  }

  .md\:focus\:text-red-900:focus {
    color: #742a2a;
  }

  .md\:focus\:text-orange-100:focus {
    color: #fffaf0;
  }

  .md\:focus\:text-orange-200:focus {
    color: #feebc8;
  }

  .md\:focus\:text-orange-300:focus {
    color: #fbd38d;
  }

  .md\:focus\:text-orange-400:focus {
    color: #f6ad55;
  }

  .md\:focus\:text-orange-500:focus {
    color: #ed8936;
  }

  .md\:focus\:text-orange-600:focus {
    color: #dd6b20;
  }

  .md\:focus\:text-orange-700:focus {
    color: #c05621;
  }

  .md\:focus\:text-orange-800:focus {
    color: #9c4221;
  }

  .md\:focus\:text-orange-900:focus {
    color: #7b341e;
  }

  .md\:focus\:text-yellow-100:focus {
    color: #fffff0;
  }

  .md\:focus\:text-yellow-200:focus {
    color: #fefcbf;
  }

  .md\:focus\:text-yellow-300:focus {
    color: #faf089;
  }

  .md\:focus\:text-yellow-400:focus {
    color: #f6e05e;
  }

  .md\:focus\:text-yellow-500:focus {
    color: #ecc94b;
  }

  .md\:focus\:text-yellow-600:focus {
    color: #d69e2e;
  }

  .md\:focus\:text-yellow-700:focus {
    color: #b7791f;
  }

  .md\:focus\:text-yellow-800:focus {
    color: #975a16;
  }

  .md\:focus\:text-yellow-900:focus {
    color: #744210;
  }

  .md\:focus\:text-green-100:focus {
    color: #f0fff4;
  }

  .md\:focus\:text-green-200:focus {
    color: #c6f6d5;
  }

  .md\:focus\:text-green-300:focus {
    color: #9ae6b4;
  }

  .md\:focus\:text-green-400:focus {
    color: #68d391;
  }

  .md\:focus\:text-green-500:focus {
    color: #48bb78;
  }

  .md\:focus\:text-green-600:focus {
    color: #38a169;
  }

  .md\:focus\:text-green-700:focus {
    color: #2f855a;
  }

  .md\:focus\:text-green-800:focus {
    color: #276749;
  }

  .md\:focus\:text-green-900:focus {
    color: #22543d;
  }

  .md\:focus\:text-teal-100:focus {
    color: #e6fffa;
  }

  .md\:focus\:text-teal-200:focus {
    color: #b2f5ea;
  }

  .md\:focus\:text-teal-300:focus {
    color: #81e6d9;
  }

  .md\:focus\:text-teal-400:focus {
    color: #4fd1c5;
  }

  .md\:focus\:text-teal-500:focus {
    color: #38b2ac;
  }

  .md\:focus\:text-teal-600:focus {
    color: #319795;
  }

  .md\:focus\:text-teal-700:focus {
    color: #2c7a7b;
  }

  .md\:focus\:text-teal-800:focus {
    color: #285e61;
  }

  .md\:focus\:text-teal-900:focus {
    color: #234e52;
  }

  .md\:focus\:text-blue-100:focus {
    color: #ebf8ff;
  }

  .md\:focus\:text-blue-200:focus {
    color: #bee3f8;
  }

  .md\:focus\:text-blue-300:focus {
    color: #90cdf4;
  }

  .md\:focus\:text-blue-400:focus {
    color: #63b3ed;
  }

  .md\:focus\:text-blue-500:focus {
    color: #4299e1;
  }

  .md\:focus\:text-blue-600:focus {
    color: #3182ce;
  }

  .md\:focus\:text-blue-700:focus {
    color: #2b6cb0;
  }

  .md\:focus\:text-blue-800:focus {
    color: #2c5282;
  }

  .md\:focus\:text-blue-900:focus {
    color: #2a4365;
  }

  .md\:focus\:text-indigo-100:focus {
    color: #ebf4ff;
  }

  .md\:focus\:text-indigo-200:focus {
    color: #c3dafe;
  }

  .md\:focus\:text-indigo-300:focus {
    color: #a3bffa;
  }

  .md\:focus\:text-indigo-400:focus {
    color: #7f9cf5;
  }

  .md\:focus\:text-indigo-500:focus {
    color: #667eea;
  }

  .md\:focus\:text-indigo-600:focus {
    color: #5a67d8;
  }

  .md\:focus\:text-indigo-700:focus {
    color: #4c51bf;
  }

  .md\:focus\:text-indigo-800:focus {
    color: #434190;
  }

  .md\:focus\:text-indigo-900:focus {
    color: #3c366b;
  }

  .md\:focus\:text-purple-100:focus {
    color: #faf5ff;
  }

  .md\:focus\:text-purple-200:focus {
    color: #e9d8fd;
  }

  .md\:focus\:text-purple-300:focus {
    color: #d6bcfa;
  }

  .md\:focus\:text-purple-400:focus {
    color: #b794f4;
  }

  .md\:focus\:text-purple-500:focus {
    color: #9f7aea;
  }

  .md\:focus\:text-purple-600:focus {
    color: #805ad5;
  }

  .md\:focus\:text-purple-700:focus {
    color: #6b46c1;
  }

  .md\:focus\:text-purple-800:focus {
    color: #553c9a;
  }

  .md\:focus\:text-purple-900:focus {
    color: #44337a;
  }

  .md\:focus\:text-pink-100:focus {
    color: #fff5f7;
  }

  .md\:focus\:text-pink-200:focus {
    color: #fed7e2;
  }

  .md\:focus\:text-pink-300:focus {
    color: #fbb6ce;
  }

  .md\:focus\:text-pink-400:focus {
    color: #f687b3;
  }

  .md\:focus\:text-pink-500:focus {
    color: #ed64a6;
  }

  .md\:focus\:text-pink-600:focus {
    color: #d53f8c;
  }

  .md\:focus\:text-pink-700:focus {
    color: #b83280;
  }

  .md\:focus\:text-pink-800:focus {
    color: #97266d;
  }

  .md\:focus\:text-pink-900:focus {
    color: #702459;
  }

  .md\:text-xs {
    font-size: 0.75rem;
  }

  .md\:text-sm {
    font-size: 0.875rem;
  }

  .md\:text-base {
    font-size: 1rem;
  }

  .md\:text-lg {
    font-size: 1.125rem;
  }

  .md\:text-xl {
    font-size: 1.25rem;
  }

  .md\:text-2xl {
    font-size: 1.5rem;
  }

  .md\:text-3xl {
    font-size: 1.875rem;
  }

  .md\:text-4xl {
    font-size: 2.25rem;
  }

  .md\:text-5xl {
    font-size: 3rem;
  }

  .md\:text-6xl {
    font-size: 4rem;
  }

  .md\:italic {
    font-style: italic;
  }

  .md\:not-italic {
    font-style: normal;
  }

  .md\:uppercase {
    text-transform: uppercase;
  }

  .md\:lowercase {
    text-transform: lowercase;
  }

  .md\:capitalize {
    text-transform: capitalize;
  }

  .md\:normal-case {
    text-transform: none;
  }

  .md\:underline {
    text-decoration: underline;
  }

  .md\:line-through {
    text-decoration: line-through;
  }

  .md\:no-underline {
    text-decoration: none;
  }

  .md\:hover\:underline:hover {
    text-decoration: underline;
  }

  .md\:hover\:line-through:hover {
    text-decoration: line-through;
  }

  .md\:hover\:no-underline:hover {
    text-decoration: none;
  }

  .md\:focus\:underline:focus {
    text-decoration: underline;
  }

  .md\:focus\:line-through:focus {
    text-decoration: line-through;
  }

  .md\:focus\:no-underline:focus {
    text-decoration: none;
  }

  .md\:antialiased {
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
  }

  .md\:subpixel-antialiased {
    -webkit-font-smoothing: auto;
    -moz-osx-font-smoothing: auto;
  }

  .md\:tracking-tighter {
    letter-spacing: -0.05em;
  }

  .md\:tracking-tight {
    letter-spacing: -0.025em;
  }

  .md\:tracking-normal {
    letter-spacing: 0;
  }

  .md\:tracking-wide {
    letter-spacing: 0.025em;
  }

  .md\:tracking-wider {
    letter-spacing: 0.05em;
  }

  .md\:tracking-widest {
    letter-spacing: 0.1em;
  }

  .md\:select-none {
    -webkit-user-select: none;
       -moz-user-select: none;
        -ms-user-select: none;
            user-select: none;
  }

  .md\:select-text {
    -webkit-user-select: text;
       -moz-user-select: text;
        -ms-user-select: text;
            user-select: text;
  }

  .md\:select-all {
    -webkit-user-select: all;
       -moz-user-select: all;
        -ms-user-select: all;
            user-select: all;
  }

  .md\:select-auto {
    -webkit-user-select: auto;
       -moz-user-select: auto;
        -ms-user-select: auto;
            user-select: auto;
  }

  .md\:align-baseline {
    vertical-align: baseline;
  }

  .md\:align-top {
    vertical-align: top;
  }

  .md\:align-middle {
    vertical-align: middle;
  }

  .md\:align-bottom {
    vertical-align: bottom;
  }

  .md\:align-text-top {
    vertical-align: text-top;
  }

  .md\:align-text-bottom {
    vertical-align: text-bottom;
  }

  .md\:visible {
    visibility: visible;
  }

  .md\:invisible {
    visibility: hidden;
  }

  .md\:whitespace-normal {
    white-space: normal;
  }

  .md\:whitespace-no-wrap {
    white-space: nowrap;
  }

  .md\:whitespace-pre {
    white-space: pre;
  }

  .md\:whitespace-pre-line {
    white-space: pre-line;
  }

  .md\:whitespace-pre-wrap {
    white-space: pre-wrap;
  }

  .md\:break-normal {
    overflow-wrap: normal;
    word-break: normal;
  }

  .md\:break-words {
    overflow-wrap: break-word;
  }

  .md\:break-all {
    word-break: break-all;
  }

  .md\:truncate {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .md\:w-0 {
    width: 0;
  }

  .md\:w-1 {
    width: 0.25rem;
  }

  .md\:w-2 {
    width: 0.5rem;
  }

  .md\:w-3 {
    width: 0.75rem;
  }

  .md\:w-4 {
    width: 1rem;
  }

  .md\:w-5 {
    width: 1.25rem;
  }

  .md\:w-6 {
    width: 1.5rem;
  }

  .md\:w-8 {
    width: 2rem;
  }

  .md\:w-10 {
    width: 2.5rem;
  }

  .md\:w-12 {
    width: 3rem;
  }

  .md\:w-16 {
    width: 4rem;
  }

  .md\:w-20 {
    width: 5rem;
  }

  .md\:w-24 {
    width: 6rem;
  }

  .md\:w-32 {
    width: 8rem;
  }

  .md\:w-40 {
    width: 10rem;
  }

  .md\:w-48 {
    width: 12rem;
  }

  .md\:w-56 {
    width: 14rem;
  }

  .md\:w-64 {
    width: 16rem;
  }

  .md\:w-auto {
    width: auto;
  }

  .md\:w-px {
    width: 1px;
  }

  .md\:w-1\/2 {
    width: 50%;
  }

  .md\:w-1\/3 {
    width: 33.333333%;
  }

  .md\:w-2\/3 {
    width: 66.666667%;
  }

  .md\:w-1\/4 {
    width: 25%;
  }

  .md\:w-2\/4 {
    width: 50%;
  }

  .md\:w-3\/4 {
    width: 75%;
  }

  .md\:w-1\/5 {
    width: 20%;
  }

  .md\:w-2\/5 {
    width: 40%;
  }

  .md\:w-3\/5 {
    width: 60%;
  }

  .md\:w-4\/5 {
    width: 80%;
  }

  .md\:w-1\/6 {
    width: 16.666667%;
  }

  .md\:w-2\/6 {
    width: 33.333333%;
  }

  .md\:w-3\/6 {
    width: 50%;
  }

  .md\:w-4\/6 {
    width: 66.666667%;
  }

  .md\:w-5\/6 {
    width: 83.333333%;
  }

  .md\:w-1\/12 {
    width: 8.333333%;
  }

  .md\:w-2\/12 {
    width: 16.666667%;
  }

  .md\:w-3\/12 {
    width: 25%;
  }

  .md\:w-4\/12 {
    width: 33.333333%;
  }

  .md\:w-5\/12 {
    width: 41.666667%;
  }

  .md\:w-6\/12 {
    width: 50%;
  }

  .md\:w-7\/12 {
    width: 58.333333%;
  }

  .md\:w-8\/12 {
    width: 66.666667%;
  }

  .md\:w-9\/12 {
    width: 75%;
  }

  .md\:w-10\/12 {
    width: 83.333333%;
  }

  .md\:w-11\/12 {
    width: 91.666667%;
  }

  .md\:w-full {
    width: 100%;
  }

  .md\:w-screen {
    width: 100vw;
  }

  .md\:z-0 {
    z-index: 0;
  }

  .md\:z-10 {
    z-index: 10;
  }

  .md\:z-20 {
    z-index: 20;
  }

  .md\:z-30 {
    z-index: 30;
  }

  .md\:z-40 {
    z-index: 40;
  }

  .md\:z-50 {
    z-index: 50;
  }

  .md\:z-auto {
    z-index: auto;
  }
}

@media (min-width: 1024px) {
  .lg\:sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border-width: 0;
  }

  .lg\:not-sr-only {
    position: static;
    width: auto;
    height: auto;
    padding: 0;
    margin: 0;
    overflow: visible;
    clip: auto;
    white-space: normal;
  }

  .lg\:focus\:sr-only:focus {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border-width: 0;
  }

  .lg\:focus\:not-sr-only:focus {
    position: static;
    width: auto;
    height: auto;
    padding: 0;
    margin: 0;
    overflow: visible;
    clip: auto;
    white-space: normal;
  }

  .lg\:appearance-none {
    -webkit-appearance: none;
       -moz-appearance: none;
            appearance: none;
  }

  .lg\:bg-fixed {
    background-attachment: fixed;
  }

  .lg\:bg-local {
    background-attachment: local;
  }

  .lg\:bg-scroll {
    background-attachment: scroll;
  }

  .lg\:bg-transparent {
    background-color: transparent;
  }

  .lg\:bg-black {
    background-color: #000;
  }

  .lg\:bg-white {
    background-color: #fff;
  }

  .lg\:bg-gray-100 {
    background-color: #f7fafc;
  }

  .lg\:bg-gray-200 {
    background-color: #edf2f7;
  }

  .lg\:bg-gray-300 {
    background-color: #e2e8f0;
  }

  .lg\:bg-gray-400 {
    background-color: #cbd5e0;
  }

  .lg\:bg-gray-500 {
    background-color: #a0aec0;
  }

  .lg\:bg-gray-600 {
    background-color: #718096;
  }

  .lg\:bg-gray-700 {
    background-color: #4a5568;
  }

  .lg\:bg-gray-800 {
    background-color: #2d3748;
  }

  .lg\:bg-gray-900 {
    background-color: #1a202c;
  }

  .lg\:bg-red-100 {
    background-color: #fff5f5;
  }

  .lg\:bg-red-200 {
    background-color: #fed7d7;
  }

  .lg\:bg-red-300 {
    background-color: #feb2b2;
  }

  .lg\:bg-red-400 {
    background-color: #fc8181;
  }

  .lg\:bg-red-500 {
    background-color: #f56565;
  }

  .lg\:bg-red-600 {
    background-color: #e53e3e;
  }

  .lg\:bg-red-700 {
    background-color: #c53030;
  }

  .lg\:bg-red-800 {
    background-color: #9b2c2c;
  }

  .lg\:bg-red-900 {
    background-color: #742a2a;
  }

  .lg\:bg-orange-100 {
    background-color: #fffaf0;
  }

  .lg\:bg-orange-200 {
    background-color: #feebc8;
  }

  .lg\:bg-orange-300 {
    background-color: #fbd38d;
  }

  .lg\:bg-orange-400 {
    background-color: #f6ad55;
  }

  .lg\:bg-orange-500 {
    background-color: #ed8936;
  }

  .lg\:bg-orange-600 {
    background-color: #dd6b20;
  }

  .lg\:bg-orange-700 {
    background-color: #c05621;
  }

  .lg\:bg-orange-800 {
    background-color: #9c4221;
  }

  .lg\:bg-orange-900 {
    background-color: #7b341e;
  }

  .lg\:bg-yellow-100 {
    background-color: #fffff0;
  }

  .lg\:bg-yellow-200 {
    background-color: #fefcbf;
  }

  .lg\:bg-yellow-300 {
    background-color: #faf089;
  }

  .lg\:bg-yellow-400 {
    background-color: #f6e05e;
  }

  .lg\:bg-yellow-500 {
    background-color: #ecc94b;
  }

  .lg\:bg-yellow-600 {
    background-color: #d69e2e;
  }

  .lg\:bg-yellow-700 {
    background-color: #b7791f;
  }

  .lg\:bg-yellow-800 {
    background-color: #975a16;
  }

  .lg\:bg-yellow-900 {
    background-color: #744210;
  }

  .lg\:bg-green-100 {
    background-color: #f0fff4;
  }

  .lg\:bg-green-200 {
    background-color: #c6f6d5;
  }

  .lg\:bg-green-300 {
    background-color: #9ae6b4;
  }

  .lg\:bg-green-400 {
    background-color: #68d391;
  }

  .lg\:bg-green-500 {
    background-color: #48bb78;
  }

  .lg\:bg-green-600 {
    background-color: #38a169;
  }

  .lg\:bg-green-700 {
    background-color: #2f855a;
  }

  .lg\:bg-green-800 {
    background-color: #276749;
  }

  .lg\:bg-green-900 {
    background-color: #22543d;
  }

  .lg\:bg-teal-100 {
    background-color: #e6fffa;
  }

  .lg\:bg-teal-200 {
    background-color: #b2f5ea;
  }

  .lg\:bg-teal-300 {
    background-color: #81e6d9;
  }

  .lg\:bg-teal-400 {
    background-color: #4fd1c5;
  }

  .lg\:bg-teal-500 {
    background-color: #38b2ac;
  }

  .lg\:bg-teal-600 {
    background-color: #319795;
  }

  .lg\:bg-teal-700 {
    background-color: #2c7a7b;
  }

  .lg\:bg-teal-800 {
    background-color: #285e61;
  }

  .lg\:bg-teal-900 {
    background-color: #234e52;
  }

  .lg\:bg-blue-100 {
    background-color: #ebf8ff;
  }

  .lg\:bg-blue-200 {
    background-color: #bee3f8;
  }

  .lg\:bg-blue-300 {
    background-color: #90cdf4;
  }

  .lg\:bg-blue-400 {
    background-color: #63b3ed;
  }

  .lg\:bg-blue-500 {
    background-color: #4299e1;
  }

  .lg\:bg-blue-600 {
    background-color: #3182ce;
  }

  .lg\:bg-blue-700 {
    background-color: #2b6cb0;
  }

  .lg\:bg-blue-800 {
    background-color: #2c5282;
  }

  .lg\:bg-blue-900 {
    background-color: #2a4365;
  }

  .lg\:bg-indigo-100 {
    background-color: #ebf4ff;
  }

  .lg\:bg-indigo-200 {
    background-color: #c3dafe;
  }

  .lg\:bg-indigo-300 {
    background-color: #a3bffa;
  }

  .lg\:bg-indigo-400 {
    background-color: #7f9cf5;
  }

  .lg\:bg-indigo-500 {
    background-color: #667eea;
  }

  .lg\:bg-indigo-600 {
    background-color: #5a67d8;
  }

  .lg\:bg-indigo-700 {
    background-color: #4c51bf;
  }

  .lg\:bg-indigo-800 {
    background-color: #434190;
  }

  .lg\:bg-indigo-900 {
    background-color: #3c366b;
  }

  .lg\:bg-purple-100 {
    background-color: #faf5ff;
  }

  .lg\:bg-purple-200 {
    background-color: #e9d8fd;
  }

  .lg\:bg-purple-300 {
    background-color: #d6bcfa;
  }

  .lg\:bg-purple-400 {
    background-color: #b794f4;
  }

  .lg\:bg-purple-500 {
    background-color: #9f7aea;
  }

  .lg\:bg-purple-600 {
    background-color: #805ad5;
  }

  .lg\:bg-purple-700 {
    background-color: #6b46c1;
  }

  .lg\:bg-purple-800 {
    background-color: #553c9a;
  }

  .lg\:bg-purple-900 {
    background-color: #44337a;
  }

  .lg\:bg-pink-100 {
    background-color: #fff5f7;
  }

  .lg\:bg-pink-200 {
    background-color: #fed7e2;
  }

  .lg\:bg-pink-300 {
    background-color: #fbb6ce;
  }

  .lg\:bg-pink-400 {
    background-color: #f687b3;
  }

  .lg\:bg-pink-500 {
    background-color: #ed64a6;
  }

  .lg\:bg-pink-600 {
    background-color: #d53f8c;
  }

  .lg\:bg-pink-700 {
    background-color: #b83280;
  }

  .lg\:bg-pink-800 {
    background-color: #97266d;
  }

  .lg\:bg-pink-900 {
    background-color: #702459;
  }

  .lg\:hover\:bg-transparent:hover {
    background-color: transparent;
  }

  .lg\:hover\:bg-black:hover {
    background-color: #000;
  }

  .lg\:hover\:bg-white:hover {
    background-color: #fff;
  }

  .lg\:hover\:bg-gray-100:hover {
    background-color: #f7fafc;
  }

  .lg\:hover\:bg-gray-200:hover {
    background-color: #edf2f7;
  }

  .lg\:hover\:bg-gray-300:hover {
    background-color: #e2e8f0;
  }

  .lg\:hover\:bg-gray-400:hover {
    background-color: #cbd5e0;
  }

  .lg\:hover\:bg-gray-500:hover {
    background-color: #a0aec0;
  }

  .lg\:hover\:bg-gray-600:hover {
    background-color: #718096;
  }

  .lg\:hover\:bg-gray-700:hover {
    background-color: #4a5568;
  }

  .lg\:hover\:bg-gray-800:hover {
    background-color: #2d3748;
  }

  .lg\:hover\:bg-gray-900:hover {
    background-color: #1a202c;
  }

  .lg\:hover\:bg-red-100:hover {
    background-color: #fff5f5;
  }

  .lg\:hover\:bg-red-200:hover {
    background-color: #fed7d7;
  }

  .lg\:hover\:bg-red-300:hover {
    background-color: #feb2b2;
  }

  .lg\:hover\:bg-red-400:hover {
    background-color: #fc8181;
  }

  .lg\:hover\:bg-red-500:hover {
    background-color: #f56565;
  }

  .lg\:hover\:bg-red-600:hover {
    background-color: #e53e3e;
  }

  .lg\:hover\:bg-red-700:hover {
    background-color: #c53030;
  }

  .lg\:hover\:bg-red-800:hover {
    background-color: #9b2c2c;
  }

  .lg\:hover\:bg-red-900:hover {
    background-color: #742a2a;
  }

  .lg\:hover\:bg-orange-100:hover {
    background-color: #fffaf0;
  }

  .lg\:hover\:bg-orange-200:hover {
    background-color: #feebc8;
  }

  .lg\:hover\:bg-orange-300:hover {
    background-color: #fbd38d;
  }

  .lg\:hover\:bg-orange-400:hover {
    background-color: #f6ad55;
  }

  .lg\:hover\:bg-orange-500:hover {
    background-color: #ed8936;
  }

  .lg\:hover\:bg-orange-600:hover {
    background-color: #dd6b20;
  }

  .lg\:hover\:bg-orange-700:hover {
    background-color: #c05621;
  }

  .lg\:hover\:bg-orange-800:hover {
    background-color: #9c4221;
  }

  .lg\:hover\:bg-orange-900:hover {
    background-color: #7b341e;
  }

  .lg\:hover\:bg-yellow-100:hover {
    background-color: #fffff0;
  }

  .lg\:hover\:bg-yellow-200:hover {
    background-color: #fefcbf;
  }

  .lg\:hover\:bg-yellow-300:hover {
    background-color: #faf089;
  }

  .lg\:hover\:bg-yellow-400:hover {
    background-color: #f6e05e;
  }

  .lg\:hover\:bg-yellow-500:hover {
    background-color: #ecc94b;
  }

  .lg\:hover\:bg-yellow-600:hover {
    background-color: #d69e2e;
  }

  .lg\:hover\:bg-yellow-700:hover {
    background-color: #b7791f;
  }

  .lg\:hover\:bg-yellow-800:hover {
    background-color: #975a16;
  }

  .lg\:hover\:bg-yellow-900:hover {
    background-color: #744210;
  }

  .lg\:hover\:bg-green-100:hover {
    background-color: #f0fff4;
  }

  .lg\:hover\:bg-green-200:hover {
    background-color: #c6f6d5;
  }

  .lg\:hover\:bg-green-300:hover {
    background-color: #9ae6b4;
  }

  .lg\:hover\:bg-green-400:hover {
    background-color: #68d391;
  }

  .lg\:hover\:bg-green-500:hover {
    background-color: #48bb78;
  }

  .lg\:hover\:bg-green-600:hover {
    background-color: #38a169;
  }

  .lg\:hover\:bg-green-700:hover {
    background-color: #2f855a;
  }

  .lg\:hover\:bg-green-800:hover {
    background-color: #276749;
  }

  .lg\:hover\:bg-green-900:hover {
    background-color: #22543d;
  }

  .lg\:hover\:bg-teal-100:hover {
    background-color: #e6fffa;
  }

  .lg\:hover\:bg-teal-200:hover {
    background-color: #b2f5ea;
  }

  .lg\:hover\:bg-teal-300:hover {
    background-color: #81e6d9;
  }

  .lg\:hover\:bg-teal-400:hover {
    background-color: #4fd1c5;
  }

  .lg\:hover\:bg-teal-500:hover {
    background-color: #38b2ac;
  }

  .lg\:hover\:bg-teal-600:hover {
    background-color: #319795;
  }

  .lg\:hover\:bg-teal-700:hover {
    background-color: #2c7a7b;
  }

  .lg\:hover\:bg-teal-800:hover {
    background-color: #285e61;
  }

  .lg\:hover\:bg-teal-900:hover {
    background-color: #234e52;
  }

  .lg\:hover\:bg-blue-100:hover {
    background-color: #ebf8ff;
  }

  .lg\:hover\:bg-blue-200:hover {
    background-color: #bee3f8;
  }

  .lg\:hover\:bg-blue-300:hover {
    background-color: #90cdf4;
  }

  .lg\:hover\:bg-blue-400:hover {
    background-color: #63b3ed;
  }

  .lg\:hover\:bg-blue-500:hover {
    background-color: #4299e1;
  }

  .lg\:hover\:bg-blue-600:hover {
    background-color: #3182ce;
  }

  .lg\:hover\:bg-blue-700:hover {
    background-color: #2b6cb0;
  }

  .lg\:hover\:bg-blue-800:hover {
    background-color: #2c5282;
  }

  .lg\:hover\:bg-blue-900:hover {
    background-color: #2a4365;
  }

  .lg\:hover\:bg-indigo-100:hover {
    background-color: #ebf4ff;
  }

  .lg\:hover\:bg-indigo-200:hover {
    background-color: #c3dafe;
  }

  .lg\:hover\:bg-indigo-300:hover {
    background-color: #a3bffa;
  }

  .lg\:hover\:bg-indigo-400:hover {
    background-color: #7f9cf5;
  }

  .lg\:hover\:bg-indigo-500:hover {
    background-color: #667eea;
  }

  .lg\:hover\:bg-indigo-600:hover {
    background-color: #5a67d8;
  }

  .lg\:hover\:bg-indigo-700:hover {
    background-color: #4c51bf;
  }

  .lg\:hover\:bg-indigo-800:hover {
    background-color: #434190;
  }

  .lg\:hover\:bg-indigo-900:hover {
    background-color: #3c366b;
  }

  .lg\:hover\:bg-purple-100:hover {
    background-color: #faf5ff;
  }

  .lg\:hover\:bg-purple-200:hover {
    background-color: #e9d8fd;
  }

  .lg\:hover\:bg-purple-300:hover {
    background-color: #d6bcfa;
  }

  .lg\:hover\:bg-purple-400:hover {
    background-color: #b794f4;
  }

  .lg\:hover\:bg-purple-500:hover {
    background-color: #9f7aea;
  }

  .lg\:hover\:bg-purple-600:hover {
    background-color: #805ad5;
  }

  .lg\:hover\:bg-purple-700:hover {
    background-color: #6b46c1;
  }

  .lg\:hover\:bg-purple-800:hover {
    background-color: #553c9a;
  }

  .lg\:hover\:bg-purple-900:hover {
    background-color: #44337a;
  }

  .lg\:hover\:bg-pink-100:hover {
    background-color: #fff5f7;
  }

  .lg\:hover\:bg-pink-200:hover {
    background-color: #fed7e2;
  }

  .lg\:hover\:bg-pink-300:hover {
    background-color: #fbb6ce;
  }

  .lg\:hover\:bg-pink-400:hover {
    background-color: #f687b3;
  }

  .lg\:hover\:bg-pink-500:hover {
    background-color: #ed64a6;
  }

  .lg\:hover\:bg-pink-600:hover {
    background-color: #d53f8c;
  }

  .lg\:hover\:bg-pink-700:hover {
    background-color: #b83280;
  }

  .lg\:hover\:bg-pink-800:hover {
    background-color: #97266d;
  }

  .lg\:hover\:bg-pink-900:hover {
    background-color: #702459;
  }

  .lg\:focus\:bg-transparent:focus {
    background-color: transparent;
  }

  .lg\:focus\:bg-black:focus {
    background-color: #000;
  }

  .lg\:focus\:bg-white:focus {
    background-color: #fff;
  }

  .lg\:focus\:bg-gray-100:focus {
    background-color: #f7fafc;
  }

  .lg\:focus\:bg-gray-200:focus {
    background-color: #edf2f7;
  }

  .lg\:focus\:bg-gray-300:focus {
    background-color: #e2e8f0;
  }

  .lg\:focus\:bg-gray-400:focus {
    background-color: #cbd5e0;
  }

  .lg\:focus\:bg-gray-500:focus {
    background-color: #a0aec0;
  }

  .lg\:focus\:bg-gray-600:focus {
    background-color: #718096;
  }

  .lg\:focus\:bg-gray-700:focus {
    background-color: #4a5568;
  }

  .lg\:focus\:bg-gray-800:focus {
    background-color: #2d3748;
  }

  .lg\:focus\:bg-gray-900:focus {
    background-color: #1a202c;
  }

  .lg\:focus\:bg-red-100:focus {
    background-color: #fff5f5;
  }

  .lg\:focus\:bg-red-200:focus {
    background-color: #fed7d7;
  }

  .lg\:focus\:bg-red-300:focus {
    background-color: #feb2b2;
  }

  .lg\:focus\:bg-red-400:focus {
    background-color: #fc8181;
  }

  .lg\:focus\:bg-red-500:focus {
    background-color: #f56565;
  }

  .lg\:focus\:bg-red-600:focus {
    background-color: #e53e3e;
  }

  .lg\:focus\:bg-red-700:focus {
    background-color: #c53030;
  }

  .lg\:focus\:bg-red-800:focus {
    background-color: #9b2c2c;
  }

  .lg\:focus\:bg-red-900:focus {
    background-color: #742a2a;
  }

  .lg\:focus\:bg-orange-100:focus {
    background-color: #fffaf0;
  }

  .lg\:focus\:bg-orange-200:focus {
    background-color: #feebc8;
  }

  .lg\:focus\:bg-orange-300:focus {
    background-color: #fbd38d;
  }

  .lg\:focus\:bg-orange-400:focus {
    background-color: #f6ad55;
  }

  .lg\:focus\:bg-orange-500:focus {
    background-color: #ed8936;
  }

  .lg\:focus\:bg-orange-600:focus {
    background-color: #dd6b20;
  }

  .lg\:focus\:bg-orange-700:focus {
    background-color: #c05621;
  }

  .lg\:focus\:bg-orange-800:focus {
    background-color: #9c4221;
  }

  .lg\:focus\:bg-orange-900:focus {
    background-color: #7b341e;
  }

  .lg\:focus\:bg-yellow-100:focus {
    background-color: #fffff0;
  }

  .lg\:focus\:bg-yellow-200:focus {
    background-color: #fefcbf;
  }

  .lg\:focus\:bg-yellow-300:focus {
    background-color: #faf089;
  }

  .lg\:focus\:bg-yellow-400:focus {
    background-color: #f6e05e;
  }

  .lg\:focus\:bg-yellow-500:focus {
    background-color: #ecc94b;
  }

  .lg\:focus\:bg-yellow-600:focus {
    background-color: #d69e2e;
  }

  .lg\:focus\:bg-yellow-700:focus {
    background-color: #b7791f;
  }

  .lg\:focus\:bg-yellow-800:focus {
    background-color: #975a16;
  }

  .lg\:focus\:bg-yellow-900:focus {
    background-color: #744210;
  }

  .lg\:focus\:bg-green-100:focus {
    background-color: #f0fff4;
  }

  .lg\:focus\:bg-green-200:focus {
    background-color: #c6f6d5;
  }

  .lg\:focus\:bg-green-300:focus {
    background-color: #9ae6b4;
  }

  .lg\:focus\:bg-green-400:focus {
    background-color: #68d391;
  }

  .lg\:focus\:bg-green-500:focus {
    background-color: #48bb78;
  }

  .lg\:focus\:bg-green-600:focus {
    background-color: #38a169;
  }

  .lg\:focus\:bg-green-700:focus {
    background-color: #2f855a;
  }

  .lg\:focus\:bg-green-800:focus {
    background-color: #276749;
  }

  .lg\:focus\:bg-green-900:focus {
    background-color: #22543d;
  }

  .lg\:focus\:bg-teal-100:focus {
    background-color: #e6fffa;
  }

  .lg\:focus\:bg-teal-200:focus {
    background-color: #b2f5ea;
  }

  .lg\:focus\:bg-teal-300:focus {
    background-color: #81e6d9;
  }

  .lg\:focus\:bg-teal-400:focus {
    background-color: #4fd1c5;
  }

  .lg\:focus\:bg-teal-500:focus {
    background-color: #38b2ac;
  }

  .lg\:focus\:bg-teal-600:focus {
    background-color: #319795;
  }

  .lg\:focus\:bg-teal-700:focus {
    background-color: #2c7a7b;
  }

  .lg\:focus\:bg-teal-800:focus {
    background-color: #285e61;
  }

  .lg\:focus\:bg-teal-900:focus {
    background-color: #234e52;
  }

  .lg\:focus\:bg-blue-100:focus {
    background-color: #ebf8ff;
  }

  .lg\:focus\:bg-blue-200:focus {
    background-color: #bee3f8;
  }

  .lg\:focus\:bg-blue-300:focus {
    background-color: #90cdf4;
  }

  .lg\:focus\:bg-blue-400:focus {
    background-color: #63b3ed;
  }

  .lg\:focus\:bg-blue-500:focus {
    background-color: #4299e1;
  }

  .lg\:focus\:bg-blue-600:focus {
    background-color: #3182ce;
  }

  .lg\:focus\:bg-blue-700:focus {
    background-color: #2b6cb0;
  }

  .lg\:focus\:bg-blue-800:focus {
    background-color: #2c5282;
  }

  .lg\:focus\:bg-blue-900:focus {
    background-color: #2a4365;
  }

  .lg\:focus\:bg-indigo-100:focus {
    background-color: #ebf4ff;
  }

  .lg\:focus\:bg-indigo-200:focus {
    background-color: #c3dafe;
  }

  .lg\:focus\:bg-indigo-300:focus {
    background-color: #a3bffa;
  }

  .lg\:focus\:bg-indigo-400:focus {
    background-color: #7f9cf5;
  }

  .lg\:focus\:bg-indigo-500:focus {
    background-color: #667eea;
  }

  .lg\:focus\:bg-indigo-600:focus {
    background-color: #5a67d8;
  }

  .lg\:focus\:bg-indigo-700:focus {
    background-color: #4c51bf;
  }

  .lg\:focus\:bg-indigo-800:focus {
    background-color: #434190;
  }

  .lg\:focus\:bg-indigo-900:focus {
    background-color: #3c366b;
  }

  .lg\:focus\:bg-purple-100:focus {
    background-color: #faf5ff;
  }

  .lg\:focus\:bg-purple-200:focus {
    background-color: #e9d8fd;
  }

  .lg\:focus\:bg-purple-300:focus {
    background-color: #d6bcfa;
  }

  .lg\:focus\:bg-purple-400:focus {
    background-color: #b794f4;
  }

  .lg\:focus\:bg-purple-500:focus {
    background-color: #9f7aea;
  }

  .lg\:focus\:bg-purple-600:focus {
    background-color: #805ad5;
  }

  .lg\:focus\:bg-purple-700:focus {
    background-color: #6b46c1;
  }

  .lg\:focus\:bg-purple-800:focus {
    background-color: #553c9a;
  }

  .lg\:focus\:bg-purple-900:focus {
    background-color: #44337a;
  }

  .lg\:focus\:bg-pink-100:focus {
    background-color: #fff5f7;
  }

  .lg\:focus\:bg-pink-200:focus {
    background-color: #fed7e2;
  }

  .lg\:focus\:bg-pink-300:focus {
    background-color: #fbb6ce;
  }

  .lg\:focus\:bg-pink-400:focus {
    background-color: #f687b3;
  }

  .lg\:focus\:bg-pink-500:focus {
    background-color: #ed64a6;
  }

  .lg\:focus\:bg-pink-600:focus {
    background-color: #d53f8c;
  }

  .lg\:focus\:bg-pink-700:focus {
    background-color: #b83280;
  }

  .lg\:focus\:bg-pink-800:focus {
    background-color: #97266d;
  }

  .lg\:focus\:bg-pink-900:focus {
    background-color: #702459;
  }

  .lg\:bg-bottom {
    background-position: bottom;
  }

  .lg\:bg-center {
    background-position: center;
  }

  .lg\:bg-left {
    background-position: left;
  }

  .lg\:bg-left-bottom {
    background-position: left bottom;
  }

  .lg\:bg-left-top {
    background-position: left top;
  }

  .lg\:bg-right {
    background-position: right;
  }

  .lg\:bg-right-bottom {
    background-position: right bottom;
  }

  .lg\:bg-right-top {
    background-position: right top;
  }

  .lg\:bg-top {
    background-position: top;
  }

  .lg\:bg-repeat {
    background-repeat: repeat;
  }

  .lg\:bg-no-repeat {
    background-repeat: no-repeat;
  }

  .lg\:bg-repeat-x {
    background-repeat: repeat-x;
  }

  .lg\:bg-repeat-y {
    background-repeat: repeat-y;
  }

  .lg\:bg-repeat-round {
    background-repeat: round;
  }

  .lg\:bg-repeat-space {
    background-repeat: space;
  }

  .lg\:bg-auto {
    background-size: auto;
  }

  .lg\:bg-cover {
    background-size: cover;
  }

  .lg\:bg-contain {
    background-size: contain;
  }

  .lg\:border-collapse {
    border-collapse: collapse;
  }

  .lg\:border-separate {
    border-collapse: separate;
  }

  .lg\:border-transparent {
    border-color: transparent;
  }

  .lg\:border-black {
    border-color: #000;
  }

  .lg\:border-white {
    border-color: #fff;
  }

  .lg\:border-gray-100 {
    border-color: #f7fafc;
  }

  .lg\:border-gray-200 {
    border-color: #edf2f7;
  }

  .lg\:border-gray-300 {
    border-color: #e2e8f0;
  }

  .lg\:border-gray-400 {
    border-color: #cbd5e0;
  }

  .lg\:border-gray-500 {
    border-color: #a0aec0;
  }

  .lg\:border-gray-600 {
    border-color: #718096;
  }

  .lg\:border-gray-700 {
    border-color: #4a5568;
  }

  .lg\:border-gray-800 {
    border-color: #2d3748;
  }

  .lg\:border-gray-900 {
    border-color: #1a202c;
  }

  .lg\:border-red-100 {
    border-color: #fff5f5;
  }

  .lg\:border-red-200 {
    border-color: #fed7d7;
  }

  .lg\:border-red-300 {
    border-color: #feb2b2;
  }

  .lg\:border-red-400 {
    border-color: #fc8181;
  }

  .lg\:border-red-500 {
    border-color: #f56565;
  }

  .lg\:border-red-600 {
    border-color: #e53e3e;
  }

  .lg\:border-red-700 {
    border-color: #c53030;
  }

  .lg\:border-red-800 {
    border-color: #9b2c2c;
  }

  .lg\:border-red-900 {
    border-color: #742a2a;
  }

  .lg\:border-orange-100 {
    border-color: #fffaf0;
  }

  .lg\:border-orange-200 {
    border-color: #feebc8;
  }

  .lg\:border-orange-300 {
    border-color: #fbd38d;
  }

  .lg\:border-orange-400 {
    border-color: #f6ad55;
  }

  .lg\:border-orange-500 {
    border-color: #ed8936;
  }

  .lg\:border-orange-600 {
    border-color: #dd6b20;
  }

  .lg\:border-orange-700 {
    border-color: #c05621;
  }

  .lg\:border-orange-800 {
    border-color: #9c4221;
  }

  .lg\:border-orange-900 {
    border-color: #7b341e;
  }

  .lg\:border-yellow-100 {
    border-color: #fffff0;
  }

  .lg\:border-yellow-200 {
    border-color: #fefcbf;
  }

  .lg\:border-yellow-300 {
    border-color: #faf089;
  }

  .lg\:border-yellow-400 {
    border-color: #f6e05e;
  }

  .lg\:border-yellow-500 {
    border-color: #ecc94b;
  }

  .lg\:border-yellow-600 {
    border-color: #d69e2e;
  }

  .lg\:border-yellow-700 {
    border-color: #b7791f;
  }

  .lg\:border-yellow-800 {
    border-color: #975a16;
  }

  .lg\:border-yellow-900 {
    border-color: #744210;
  }

  .lg\:border-green-100 {
    border-color: #f0fff4;
  }

  .lg\:border-green-200 {
    border-color: #c6f6d5;
  }

  .lg\:border-green-300 {
    border-color: #9ae6b4;
  }

  .lg\:border-green-400 {
    border-color: #68d391;
  }

  .lg\:border-green-500 {
    border-color: #48bb78;
  }

  .lg\:border-green-600 {
    border-color: #38a169;
  }

  .lg\:border-green-700 {
    border-color: #2f855a;
  }

  .lg\:border-green-800 {
    border-color: #276749;
  }

  .lg\:border-green-900 {
    border-color: #22543d;
  }

  .lg\:border-teal-100 {
    border-color: #e6fffa;
  }

  .lg\:border-teal-200 {
    border-color: #b2f5ea;
  }

  .lg\:border-teal-300 {
    border-color: #81e6d9;
  }

  .lg\:border-teal-400 {
    border-color: #4fd1c5;
  }

  .lg\:border-teal-500 {
    border-color: #38b2ac;
  }

  .lg\:border-teal-600 {
    border-color: #319795;
  }

  .lg\:border-teal-700 {
    border-color: #2c7a7b;
  }

  .lg\:border-teal-800 {
    border-color: #285e61;
  }

  .lg\:border-teal-900 {
    border-color: #234e52;
  }

  .lg\:border-blue-100 {
    border-color: #ebf8ff;
  }

  .lg\:border-blue-200 {
    border-color: #bee3f8;
  }

  .lg\:border-blue-300 {
    border-color: #90cdf4;
  }

  .lg\:border-blue-400 {
    border-color: #63b3ed;
  }

  .lg\:border-blue-500 {
    border-color: #4299e1;
  }

  .lg\:border-blue-600 {
    border-color: #3182ce;
  }

  .lg\:border-blue-700 {
    border-color: #2b6cb0;
  }

  .lg\:border-blue-800 {
    border-color: #2c5282;
  }

  .lg\:border-blue-900 {
    border-color: #2a4365;
  }

  .lg\:border-indigo-100 {
    border-color: #ebf4ff;
  }

  .lg\:border-indigo-200 {
    border-color: #c3dafe;
  }

  .lg\:border-indigo-300 {
    border-color: #a3bffa;
  }

  .lg\:border-indigo-400 {
    border-color: #7f9cf5;
  }

  .lg\:border-indigo-500 {
    border-color: #667eea;
  }

  .lg\:border-indigo-600 {
    border-color: #5a67d8;
  }

  .lg\:border-indigo-700 {
    border-color: #4c51bf;
  }

  .lg\:border-indigo-800 {
    border-color: #434190;
  }

  .lg\:border-indigo-900 {
    border-color: #3c366b;
  }

  .lg\:border-purple-100 {
    border-color: #faf5ff;
  }

  .lg\:border-purple-200 {
    border-color: #e9d8fd;
  }

  .lg\:border-purple-300 {
    border-color: #d6bcfa;
  }

  .lg\:border-purple-400 {
    border-color: #b794f4;
  }

  .lg\:border-purple-500 {
    border-color: #9f7aea;
  }

  .lg\:border-purple-600 {
    border-color: #805ad5;
  }

  .lg\:border-purple-700 {
    border-color: #6b46c1;
  }

  .lg\:border-purple-800 {
    border-color: #553c9a;
  }

  .lg\:border-purple-900 {
    border-color: #44337a;
  }

  .lg\:border-pink-100 {
    border-color: #fff5f7;
  }

  .lg\:border-pink-200 {
    border-color: #fed7e2;
  }

  .lg\:border-pink-300 {
    border-color: #fbb6ce;
  }

  .lg\:border-pink-400 {
    border-color: #f687b3;
  }

  .lg\:border-pink-500 {
    border-color: #ed64a6;
  }

  .lg\:border-pink-600 {
    border-color: #d53f8c;
  }

  .lg\:border-pink-700 {
    border-color: #b83280;
  }

  .lg\:border-pink-800 {
    border-color: #97266d;
  }

  .lg\:border-pink-900 {
    border-color: #702459;
  }

  .lg\:hover\:border-transparent:hover {
    border-color: transparent;
  }

  .lg\:hover\:border-black:hover {
    border-color: #000;
  }

  .lg\:hover\:border-white:hover {
    border-color: #fff;
  }

  .lg\:hover\:border-gray-100:hover {
    border-color: #f7fafc;
  }

  .lg\:hover\:border-gray-200:hover {
    border-color: #edf2f7;
  }

  .lg\:hover\:border-gray-300:hover {
    border-color: #e2e8f0;
  }

  .lg\:hover\:border-gray-400:hover {
    border-color: #cbd5e0;
  }

  .lg\:hover\:border-gray-500:hover {
    border-color: #a0aec0;
  }

  .lg\:hover\:border-gray-600:hover {
    border-color: #718096;
  }

  .lg\:hover\:border-gray-700:hover {
    border-color: #4a5568;
  }

  .lg\:hover\:border-gray-800:hover {
    border-color: #2d3748;
  }

  .lg\:hover\:border-gray-900:hover {
    border-color: #1a202c;
  }

  .lg\:hover\:border-red-100:hover {
    border-color: #fff5f5;
  }

  .lg\:hover\:border-red-200:hover {
    border-color: #fed7d7;
  }

  .lg\:hover\:border-red-300:hover {
    border-color: #feb2b2;
  }

  .lg\:hover\:border-red-400:hover {
    border-color: #fc8181;
  }

  .lg\:hover\:border-red-500:hover {
    border-color: #f56565;
  }

  .lg\:hover\:border-red-600:hover {
    border-color: #e53e3e;
  }

  .lg\:hover\:border-red-700:hover {
    border-color: #c53030;
  }

  .lg\:hover\:border-red-800:hover {
    border-color: #9b2c2c;
  }

  .lg\:hover\:border-red-900:hover {
    border-color: #742a2a;
  }

  .lg\:hover\:border-orange-100:hover {
    border-color: #fffaf0;
  }

  .lg\:hover\:border-orange-200:hover {
    border-color: #feebc8;
  }

  .lg\:hover\:border-orange-300:hover {
    border-color: #fbd38d;
  }

  .lg\:hover\:border-orange-400:hover {
    border-color: #f6ad55;
  }

  .lg\:hover\:border-orange-500:hover {
    border-color: #ed8936;
  }

  .lg\:hover\:border-orange-600:hover {
    border-color: #dd6b20;
  }

  .lg\:hover\:border-orange-700:hover {
    border-color: #c05621;
  }

  .lg\:hover\:border-orange-800:hover {
    border-color: #9c4221;
  }

  .lg\:hover\:border-orange-900:hover {
    border-color: #7b341e;
  }

  .lg\:hover\:border-yellow-100:hover {
    border-color: #fffff0;
  }

  .lg\:hover\:border-yellow-200:hover {
    border-color: #fefcbf;
  }

  .lg\:hover\:border-yellow-300:hover {
    border-color: #faf089;
  }

  .lg\:hover\:border-yellow-400:hover {
    border-color: #f6e05e;
  }

  .lg\:hover\:border-yellow-500:hover {
    border-color: #ecc94b;
  }

  .lg\:hover\:border-yellow-600:hover {
    border-color: #d69e2e;
  }

  .lg\:hover\:border-yellow-700:hover {
    border-color: #b7791f;
  }

  .lg\:hover\:border-yellow-800:hover {
    border-color: #975a16;
  }

  .lg\:hover\:border-yellow-900:hover {
    border-color: #744210;
  }

  .lg\:hover\:border-green-100:hover {
    border-color: #f0fff4;
  }

  .lg\:hover\:border-green-200:hover {
    border-color: #c6f6d5;
  }

  .lg\:hover\:border-green-300:hover {
    border-color: #9ae6b4;
  }

  .lg\:hover\:border-green-400:hover {
    border-color: #68d391;
  }

  .lg\:hover\:border-green-500:hover {
    border-color: #48bb78;
  }

  .lg\:hover\:border-green-600:hover {
    border-color: #38a169;
  }

  .lg\:hover\:border-green-700:hover {
    border-color: #2f855a;
  }

  .lg\:hover\:border-green-800:hover {
    border-color: #276749;
  }

  .lg\:hover\:border-green-900:hover {
    border-color: #22543d;
  }

  .lg\:hover\:border-teal-100:hover {
    border-color: #e6fffa;
  }

  .lg\:hover\:border-teal-200:hover {
    border-color: #b2f5ea;
  }

  .lg\:hover\:border-teal-300:hover {
    border-color: #81e6d9;
  }

  .lg\:hover\:border-teal-400:hover {
    border-color: #4fd1c5;
  }

  .lg\:hover\:border-teal-500:hover {
    border-color: #38b2ac;
  }

  .lg\:hover\:border-teal-600:hover {
    border-color: #319795;
  }

  .lg\:hover\:border-teal-700:hover {
    border-color: #2c7a7b;
  }

  .lg\:hover\:border-teal-800:hover {
    border-color: #285e61;
  }

  .lg\:hover\:border-teal-900:hover {
    border-color: #234e52;
  }

  .lg\:hover\:border-blue-100:hover {
    border-color: #ebf8ff;
  }

  .lg\:hover\:border-blue-200:hover {
    border-color: #bee3f8;
  }

  .lg\:hover\:border-blue-300:hover {
    border-color: #90cdf4;
  }

  .lg\:hover\:border-blue-400:hover {
    border-color: #63b3ed;
  }

  .lg\:hover\:border-blue-500:hover {
    border-color: #4299e1;
  }

  .lg\:hover\:border-blue-600:hover {
    border-color: #3182ce;
  }

  .lg\:hover\:border-blue-700:hover {
    border-color: #2b6cb0;
  }

  .lg\:hover\:border-blue-800:hover {
    border-color: #2c5282;
  }

  .lg\:hover\:border-blue-900:hover {
    border-color: #2a4365;
  }

  .lg\:hover\:border-indigo-100:hover {
    border-color: #ebf4ff;
  }

  .lg\:hover\:border-indigo-200:hover {
    border-color: #c3dafe;
  }

  .lg\:hover\:border-indigo-300:hover {
    border-color: #a3bffa;
  }

  .lg\:hover\:border-indigo-400:hover {
    border-color: #7f9cf5;
  }

  .lg\:hover\:border-indigo-500:hover {
    border-color: #667eea;
  }

  .lg\:hover\:border-indigo-600:hover {
    border-color: #5a67d8;
  }

  .lg\:hover\:border-indigo-700:hover {
    border-color: #4c51bf;
  }

  .lg\:hover\:border-indigo-800:hover {
    border-color: #434190;
  }

  .lg\:hover\:border-indigo-900:hover {
    border-color: #3c366b;
  }

  .lg\:hover\:border-purple-100:hover {
    border-color: #faf5ff;
  }

  .lg\:hover\:border-purple-200:hover {
    border-color: #e9d8fd;
  }

  .lg\:hover\:border-purple-300:hover {
    border-color: #d6bcfa;
  }

  .lg\:hover\:border-purple-400:hover {
    border-color: #b794f4;
  }

  .lg\:hover\:border-purple-500:hover {
    border-color: #9f7aea;
  }

  .lg\:hover\:border-purple-600:hover {
    border-color: #805ad5;
  }

  .lg\:hover\:border-purple-700:hover {
    border-color: #6b46c1;
  }

  .lg\:hover\:border-purple-800:hover {
    border-color: #553c9a;
  }

  .lg\:hover\:border-purple-900:hover {
    border-color: #44337a;
  }

  .lg\:hover\:border-pink-100:hover {
    border-color: #fff5f7;
  }

  .lg\:hover\:border-pink-200:hover {
    border-color: #fed7e2;
  }

  .lg\:hover\:border-pink-300:hover {
    border-color: #fbb6ce;
  }

  .lg\:hover\:border-pink-400:hover {
    border-color: #f687b3;
  }

  .lg\:hover\:border-pink-500:hover {
    border-color: #ed64a6;
  }

  .lg\:hover\:border-pink-600:hover {
    border-color: #d53f8c;
  }

  .lg\:hover\:border-pink-700:hover {
    border-color: #b83280;
  }

  .lg\:hover\:border-pink-800:hover {
    border-color: #97266d;
  }

  .lg\:hover\:border-pink-900:hover {
    border-color: #702459;
  }

  .lg\:focus\:border-transparent:focus {
    border-color: transparent;
  }

  .lg\:focus\:border-black:focus {
    border-color: #000;
  }

  .lg\:focus\:border-white:focus {
    border-color: #fff;
  }

  .lg\:focus\:border-gray-100:focus {
    border-color: #f7fafc;
  }

  .lg\:focus\:border-gray-200:focus {
    border-color: #edf2f7;
  }

  .lg\:focus\:border-gray-300:focus {
    border-color: #e2e8f0;
  }

  .lg\:focus\:border-gray-400:focus {
    border-color: #cbd5e0;
  }

  .lg\:focus\:border-gray-500:focus {
    border-color: #a0aec0;
  }

  .lg\:focus\:border-gray-600:focus {
    border-color: #718096;
  }

  .lg\:focus\:border-gray-700:focus {
    border-color: #4a5568;
  }

  .lg\:focus\:border-gray-800:focus {
    border-color: #2d3748;
  }

  .lg\:focus\:border-gray-900:focus {
    border-color: #1a202c;
  }

  .lg\:focus\:border-red-100:focus {
    border-color: #fff5f5;
  }

  .lg\:focus\:border-red-200:focus {
    border-color: #fed7d7;
  }

  .lg\:focus\:border-red-300:focus {
    border-color: #feb2b2;
  }

  .lg\:focus\:border-red-400:focus {
    border-color: #fc8181;
  }

  .lg\:focus\:border-red-500:focus {
    border-color: #f56565;
  }

  .lg\:focus\:border-red-600:focus {
    border-color: #e53e3e;
  }

  .lg\:focus\:border-red-700:focus {
    border-color: #c53030;
  }

  .lg\:focus\:border-red-800:focus {
    border-color: #9b2c2c;
  }

  .lg\:focus\:border-red-900:focus {
    border-color: #742a2a;
  }

  .lg\:focus\:border-orange-100:focus {
    border-color: #fffaf0;
  }

  .lg\:focus\:border-orange-200:focus {
    border-color: #feebc8;
  }

  .lg\:focus\:border-orange-300:focus {
    border-color: #fbd38d;
  }

  .lg\:focus\:border-orange-400:focus {
    border-color: #f6ad55;
  }

  .lg\:focus\:border-orange-500:focus {
    border-color: #ed8936;
  }

  .lg\:focus\:border-orange-600:focus {
    border-color: #dd6b20;
  }

  .lg\:focus\:border-orange-700:focus {
    border-color: #c05621;
  }

  .lg\:focus\:border-orange-800:focus {
    border-color: #9c4221;
  }

  .lg\:focus\:border-orange-900:focus {
    border-color: #7b341e;
  }

  .lg\:focus\:border-yellow-100:focus {
    border-color: #fffff0;
  }

  .lg\:focus\:border-yellow-200:focus {
    border-color: #fefcbf;
  }

  .lg\:focus\:border-yellow-300:focus {
    border-color: #faf089;
  }

  .lg\:focus\:border-yellow-400:focus {
    border-color: #f6e05e;
  }

  .lg\:focus\:border-yellow-500:focus {
    border-color: #ecc94b;
  }

  .lg\:focus\:border-yellow-600:focus {
    border-color: #d69e2e;
  }

  .lg\:focus\:border-yellow-700:focus {
    border-color: #b7791f;
  }

  .lg\:focus\:border-yellow-800:focus {
    border-color: #975a16;
  }

  .lg\:focus\:border-yellow-900:focus {
    border-color: #744210;
  }

  .lg\:focus\:border-green-100:focus {
    border-color: #f0fff4;
  }

  .lg\:focus\:border-green-200:focus {
    border-color: #c6f6d5;
  }

  .lg\:focus\:border-green-300:focus {
    border-color: #9ae6b4;
  }

  .lg\:focus\:border-green-400:focus {
    border-color: #68d391;
  }

  .lg\:focus\:border-green-500:focus {
    border-color: #48bb78;
  }

  .lg\:focus\:border-green-600:focus {
    border-color: #38a169;
  }

  .lg\:focus\:border-green-700:focus {
    border-color: #2f855a;
  }

  .lg\:focus\:border-green-800:focus {
    border-color: #276749;
  }

  .lg\:focus\:border-green-900:focus {
    border-color: #22543d;
  }

  .lg\:focus\:border-teal-100:focus {
    border-color: #e6fffa;
  }

  .lg\:focus\:border-teal-200:focus {
    border-color: #b2f5ea;
  }

  .lg\:focus\:border-teal-300:focus {
    border-color: #81e6d9;
  }

  .lg\:focus\:border-teal-400:focus {
    border-color: #4fd1c5;
  }

  .lg\:focus\:border-teal-500:focus {
    border-color: #38b2ac;
  }

  .lg\:focus\:border-teal-600:focus {
    border-color: #319795;
  }

  .lg\:focus\:border-teal-700:focus {
    border-color: #2c7a7b;
  }

  .lg\:focus\:border-teal-800:focus {
    border-color: #285e61;
  }

  .lg\:focus\:border-teal-900:focus {
    border-color: #234e52;
  }

  .lg\:focus\:border-blue-100:focus {
    border-color: #ebf8ff;
  }

  .lg\:focus\:border-blue-200:focus {
    border-color: #bee3f8;
  }

  .lg\:focus\:border-blue-300:focus {
    border-color: #90cdf4;
  }

  .lg\:focus\:border-blue-400:focus {
    border-color: #63b3ed;
  }

  .lg\:focus\:border-blue-500:focus {
    border-color: #4299e1;
  }

  .lg\:focus\:border-blue-600:focus {
    border-color: #3182ce;
  }

  .lg\:focus\:border-blue-700:focus {
    border-color: #2b6cb0;
  }

  .lg\:focus\:border-blue-800:focus {
    border-color: #2c5282;
  }

  .lg\:focus\:border-blue-900:focus {
    border-color: #2a4365;
  }

  .lg\:focus\:border-indigo-100:focus {
    border-color: #ebf4ff;
  }

  .lg\:focus\:border-indigo-200:focus {
    border-color: #c3dafe;
  }

  .lg\:focus\:border-indigo-300:focus {
    border-color: #a3bffa;
  }

  .lg\:focus\:border-indigo-400:focus {
    border-color: #7f9cf5;
  }

  .lg\:focus\:border-indigo-500:focus {
    border-color: #667eea;
  }

  .lg\:focus\:border-indigo-600:focus {
    border-color: #5a67d8;
  }

  .lg\:focus\:border-indigo-700:focus {
    border-color: #4c51bf;
  }

  .lg\:focus\:border-indigo-800:focus {
    border-color: #434190;
  }

  .lg\:focus\:border-indigo-900:focus {
    border-color: #3c366b;
  }

  .lg\:focus\:border-purple-100:focus {
    border-color: #faf5ff;
  }

  .lg\:focus\:border-purple-200:focus {
    border-color: #e9d8fd;
  }

  .lg\:focus\:border-purple-300:focus {
    border-color: #d6bcfa;
  }

  .lg\:focus\:border-purple-400:focus {
    border-color: #b794f4;
  }

  .lg\:focus\:border-purple-500:focus {
    border-color: #9f7aea;
  }

  .lg\:focus\:border-purple-600:focus {
    border-color: #805ad5;
  }

  .lg\:focus\:border-purple-700:focus {
    border-color: #6b46c1;
  }

  .lg\:focus\:border-purple-800:focus {
    border-color: #553c9a;
  }

  .lg\:focus\:border-purple-900:focus {
    border-color: #44337a;
  }

  .lg\:focus\:border-pink-100:focus {
    border-color: #fff5f7;
  }

  .lg\:focus\:border-pink-200:focus {
    border-color: #fed7e2;
  }

  .lg\:focus\:border-pink-300:focus {
    border-color: #fbb6ce;
  }

  .lg\:focus\:border-pink-400:focus {
    border-color: #f687b3;
  }

  .lg\:focus\:border-pink-500:focus {
    border-color: #ed64a6;
  }

  .lg\:focus\:border-pink-600:focus {
    border-color: #d53f8c;
  }

  .lg\:focus\:border-pink-700:focus {
    border-color: #b83280;
  }

  .lg\:focus\:border-pink-800:focus {
    border-color: #97266d;
  }

  .lg\:focus\:border-pink-900:focus {
    border-color: #702459;
  }

  .lg\:rounded-none {
    border-radius: 0;
  }

  .lg\:rounded-sm {
    border-radius: 0.125rem;
  }

  .lg\:rounded {
    border-radius: 0.25rem;
  }

  .lg\:rounded-lg {
    border-radius: 0.5rem;
  }

  .lg\:rounded-full {
    border-radius: 9999px;
  }

  .lg\:rounded-t-none {
    border-top-left-radius: 0;
    border-top-right-radius: 0;
  }

  .lg\:rounded-r-none {
    border-top-right-radius: 0;
    border-bottom-right-radius: 0;
  }

  .lg\:rounded-b-none {
    border-bottom-right-radius: 0;
    border-bottom-left-radius: 0;
  }

  .lg\:rounded-l-none {
    border-top-left-radius: 0;
    border-bottom-left-radius: 0;
  }

  .lg\:rounded-t-sm {
    border-top-left-radius: 0.125rem;
    border-top-right-radius: 0.125rem;
  }

  .lg\:rounded-r-sm {
    border-top-right-radius: 0.125rem;
    border-bottom-right-radius: 0.125rem;
  }

  .lg\:rounded-b-sm {
    border-bottom-right-radius: 0.125rem;
    border-bottom-left-radius: 0.125rem;
  }

  .lg\:rounded-l-sm {
    border-top-left-radius: 0.125rem;
    border-bottom-left-radius: 0.125rem;
  }

  .lg\:rounded-t {
    border-top-left-radius: 0.25rem;
    border-top-right-radius: 0.25rem;
  }

  .lg\:rounded-r {
    border-top-right-radius: 0.25rem;
    border-bottom-right-radius: 0.25rem;
  }

  .lg\:rounded-b {
    border-bottom-right-radius: 0.25rem;
    border-bottom-left-radius: 0.25rem;
  }

  .lg\:rounded-l {
    border-top-left-radius: 0.25rem;
    border-bottom-left-radius: 0.25rem;
  }

  .lg\:rounded-t-lg {
    border-top-left-radius: 0.5rem;
    border-top-right-radius: 0.5rem;
  }

  .lg\:rounded-r-lg {
    border-top-right-radius: 0.5rem;
    border-bottom-right-radius: 0.5rem;
  }

  .lg\:rounded-b-lg {
    border-bottom-right-radius: 0.5rem;
    border-bottom-left-radius: 0.5rem;
  }

  .lg\:rounded-l-lg {
    border-top-left-radius: 0.5rem;
    border-bottom-left-radius: 0.5rem;
  }

  .lg\:rounded-t-full {
    border-top-left-radius: 9999px;
    border-top-right-radius: 9999px;
  }

  .lg\:rounded-r-full {
    border-top-right-radius: 9999px;
    border-bottom-right-radius: 9999px;
  }

  .lg\:rounded-b-full {
    border-bottom-right-radius: 9999px;
    border-bottom-left-radius: 9999px;
  }

  .lg\:rounded-l-full {
    border-top-left-radius: 9999px;
    border-bottom-left-radius: 9999px;
  }

  .lg\:rounded-tl-none {
    border-top-left-radius: 0;
  }

  .lg\:rounded-tr-none {
    border-top-right-radius: 0;
  }

  .lg\:rounded-br-none {
    border-bottom-right-radius: 0;
  }

  .lg\:rounded-bl-none {
    border-bottom-left-radius: 0;
  }

  .lg\:rounded-tl-sm {
    border-top-left-radius: 0.125rem;
  }

  .lg\:rounded-tr-sm {
    border-top-right-radius: 0.125rem;
  }

  .lg\:rounded-br-sm {
    border-bottom-right-radius: 0.125rem;
  }

  .lg\:rounded-bl-sm {
    border-bottom-left-radius: 0.125rem;
  }

  .lg\:rounded-tl {
    border-top-left-radius: 0.25rem;
  }

  .lg\:rounded-tr {
    border-top-right-radius: 0.25rem;
  }

  .lg\:rounded-br {
    border-bottom-right-radius: 0.25rem;
  }

  .lg\:rounded-bl {
    border-bottom-left-radius: 0.25rem;
  }

  .lg\:rounded-tl-lg {
    border-top-left-radius: 0.5rem;
  }

  .lg\:rounded-tr-lg {
    border-top-right-radius: 0.5rem;
  }

  .lg\:rounded-br-lg {
    border-bottom-right-radius: 0.5rem;
  }

  .lg\:rounded-bl-lg {
    border-bottom-left-radius: 0.5rem;
  }

  .lg\:rounded-tl-full {
    border-top-left-radius: 9999px;
  }

  .lg\:rounded-tr-full {
    border-top-right-radius: 9999px;
  }

  .lg\:rounded-br-full {
    border-bottom-right-radius: 9999px;
  }

  .lg\:rounded-bl-full {
    border-bottom-left-radius: 9999px;
  }

  .lg\:border-solid {
    border-style: solid;
  }

  .lg\:border-dashed {
    border-style: dashed;
  }

  .lg\:border-dotted {
    border-style: dotted;
  }

  .lg\:border-double {
    border-style: double;
  }

  .lg\:border-none {
    border-style: none;
  }

  .lg\:border-0 {
    border-width: 0;
  }

  .lg\:border-2 {
    border-width: 2px;
  }

  .lg\:border-4 {
    border-width: 4px;
  }

  .lg\:border-8 {
    border-width: 8px;
  }

  .lg\:border {
    border-width: 1px;
  }

  .lg\:border-t-0 {
    border-top-width: 0;
  }

  .lg\:border-r-0 {
    border-right-width: 0;
  }

  .lg\:border-b-0 {
    border-bottom-width: 0;
  }

  .lg\:border-l-0 {
    border-left-width: 0;
  }

  .lg\:border-t-2 {
    border-top-width: 2px;
  }

  .lg\:border-r-2 {
    border-right-width: 2px;
  }

  .lg\:border-b-2 {
    border-bottom-width: 2px;
  }

  .lg\:border-l-2 {
    border-left-width: 2px;
  }

  .lg\:border-t-4 {
    border-top-width: 4px;
  }

  .lg\:border-r-4 {
    border-right-width: 4px;
  }

  .lg\:border-b-4 {
    border-bottom-width: 4px;
  }

  .lg\:border-l-4 {
    border-left-width: 4px;
  }

  .lg\:border-t-8 {
    border-top-width: 8px;
  }

  .lg\:border-r-8 {
    border-right-width: 8px;
  }

  .lg\:border-b-8 {
    border-bottom-width: 8px;
  }

  .lg\:border-l-8 {
    border-left-width: 8px;
  }

  .lg\:border-t {
    border-top-width: 1px;
  }

  .lg\:border-r {
    border-right-width: 1px;
  }

  .lg\:border-b {
    border-bottom-width: 1px;
  }

  .lg\:border-l {
    border-left-width: 1px;
  }

  .lg\:cursor-auto {
    cursor: auto;
  }

  .lg\:cursor-default {
    cursor: default;
  }

  .lg\:cursor-pointer {
    cursor: pointer;
  }

  .lg\:cursor-wait {
    cursor: wait;
  }

  .lg\:cursor-text {
    cursor: text;
  }

  .lg\:cursor-move {
    cursor: move;
  }

  .lg\:cursor-not-allowed {
    cursor: not-allowed;
  }

  .lg\:block {
    display: block;
  }

  .lg\:inline-block {
    display: inline-block;
  }

  .lg\:inline {
    display: inline;
  }

  .lg\:flex {
    display: -webkit-box;
    display: flex;
  }

  .lg\:inline-flex {
    display: -webkit-inline-box;
    display: inline-flex;
  }

  .lg\:table {
    display: table;
  }

  .lg\:table-row {
    display: table-row;
  }

  .lg\:table-cell {
    display: table-cell;
  }

  .lg\:hidden {
    display: none;
  }

  .lg\:flex-row {
    -webkit-box-orient: horizontal;
    -webkit-box-direction: normal;
            flex-direction: row;
  }

  .lg\:flex-row-reverse {
    -webkit-box-orient: horizontal;
    -webkit-box-direction: reverse;
            flex-direction: row-reverse;
  }

  .lg\:flex-col {
    -webkit-box-orient: vertical;
    -webkit-box-direction: normal;
            flex-direction: column;
  }

  .lg\:flex-col-reverse {
    -webkit-box-orient: vertical;
    -webkit-box-direction: reverse;
            flex-direction: column-reverse;
  }

  .lg\:flex-wrap {
    flex-wrap: wrap;
  }

  .lg\:flex-wrap-reverse {
    flex-wrap: wrap-reverse;
  }

  .lg\:flex-no-wrap {
    flex-wrap: nowrap;
  }

  .lg\:items-start {
    -webkit-box-align: start;
            align-items: flex-start;
  }

  .lg\:items-end {
    -webkit-box-align: end;
            align-items: flex-end;
  }

  .lg\:items-center {
    -webkit-box-align: center;
            align-items: center;
  }

  .lg\:items-baseline {
    -webkit-box-align: baseline;
            align-items: baseline;
  }

  .lg\:items-stretch {
    -webkit-box-align: stretch;
            align-items: stretch;
  }

  .lg\:self-auto {
    align-self: auto;
  }

  .lg\:self-start {
    align-self: flex-start;
  }

  .lg\:self-end {
    align-self: flex-end;
  }

  .lg\:self-center {
    align-self: center;
  }

  .lg\:self-stretch {
    align-self: stretch;
  }

  .lg\:justify-start {
    -webkit-box-pack: start;
            justify-content: flex-start;
  }

  .lg\:justify-end {
    -webkit-box-pack: end;
            justify-content: flex-end;
  }

  .lg\:justify-center {
    -webkit-box-pack: center;
            justify-content: center;
  }

  .lg\:justify-between {
    -webkit-box-pack: justify;
            justify-content: space-between;
  }

  .lg\:justify-around {
    justify-content: space-around;
  }

  .lg\:content-center {
    align-content: center;
  }

  .lg\:content-start {
    align-content: flex-start;
  }

  .lg\:content-end {
    align-content: flex-end;
  }

  .lg\:content-between {
    align-content: space-between;
  }

  .lg\:content-around {
    align-content: space-around;
  }

  .lg\:flex-1 {
    -webkit-box-flex: 1;
            flex: 1 1 0%;
  }

  .lg\:flex-auto {
    -webkit-box-flex: 1;
            flex: 1 1 auto;
  }

  .lg\:flex-initial {
    -webkit-box-flex: 0;
            flex: 0 1 auto;
  }

  .lg\:flex-none {
    -webkit-box-flex: 0;
            flex: none;
  }

  .lg\:flex-grow-0 {
    -webkit-box-flex: 0;
            flex-grow: 0;
  }

  .lg\:flex-grow {
    -webkit-box-flex: 1;
            flex-grow: 1;
  }

  .lg\:flex-shrink-0 {
    flex-shrink: 0;
  }

  .lg\:flex-shrink {
    flex-shrink: 1;
  }

  .lg\:order-1 {
    -webkit-box-ordinal-group: 2;
            order: 1;
  }

  .lg\:order-2 {
    -webkit-box-ordinal-group: 3;
            order: 2;
  }

  .lg\:order-3 {
    -webkit-box-ordinal-group: 4;
            order: 3;
  }

  .lg\:order-4 {
    -webkit-box-ordinal-group: 5;
            order: 4;
  }

  .lg\:order-5 {
    -webkit-box-ordinal-group: 6;
            order: 5;
  }

  .lg\:order-6 {
    -webkit-box-ordinal-group: 7;
            order: 6;
  }

  .lg\:order-7 {
    -webkit-box-ordinal-group: 8;
            order: 7;
  }

  .lg\:order-8 {
    -webkit-box-ordinal-group: 9;
            order: 8;
  }

  .lg\:order-9 {
    -webkit-box-ordinal-group: 10;
            order: 9;
  }

  .lg\:order-10 {
    -webkit-box-ordinal-group: 11;
            order: 10;
  }

  .lg\:order-11 {
    -webkit-box-ordinal-group: 12;
            order: 11;
  }

  .lg\:order-12 {
    -webkit-box-ordinal-group: 13;
            order: 12;
  }

  .lg\:order-first {
    -webkit-box-ordinal-group: -9998;
            order: -9999;
  }

  .lg\:order-last {
    -webkit-box-ordinal-group: 10000;
            order: 9999;
  }

  .lg\:order-none {
    -webkit-box-ordinal-group: 1;
            order: 0;
  }

  .lg\:float-right {
    float: right;
  }

  .lg\:float-left {
    float: left;
  }

  .lg\:float-none {
    float: none;
  }

  .lg\:clearfix:after {
    content: "";
    display: table;
    clear: both;
  }

  .lg\:font-sans {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji";
  }

  .lg\:font-serif {
    font-family: Georgia, Cambria, "Times New Roman", Times, serif;
  }

  .lg\:font-mono {
    font-family: Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  }

  .lg\:font-hairline {
    font-weight: 100;
  }

  .lg\:font-thin {
    font-weight: 200;
  }

  .lg\:font-light {
    font-weight: 300;
  }

  .lg\:font-normal {
    font-weight: 400;
  }

  .lg\:font-medium {
    font-weight: 500;
  }

  .lg\:font-semibold {
    font-weight: 600;
  }

  .lg\:font-bold {
    font-weight: 700;
  }

  .lg\:font-extrabold {
    font-weight: 800;
  }

  .lg\:font-black {
    font-weight: 900;
  }

  .lg\:hover\:font-hairline:hover {
    font-weight: 100;
  }

  .lg\:hover\:font-thin:hover {
    font-weight: 200;
  }

  .lg\:hover\:font-light:hover {
    font-weight: 300;
  }

  .lg\:hover\:font-normal:hover {
    font-weight: 400;
  }

  .lg\:hover\:font-medium:hover {
    font-weight: 500;
  }

  .lg\:hover\:font-semibold:hover {
    font-weight: 600;
  }

  .lg\:hover\:font-bold:hover {
    font-weight: 700;
  }

  .lg\:hover\:font-extrabold:hover {
    font-weight: 800;
  }

  .lg\:hover\:font-black:hover {
    font-weight: 900;
  }

  .lg\:focus\:font-hairline:focus {
    font-weight: 100;
  }

  .lg\:focus\:font-thin:focus {
    font-weight: 200;
  }

  .lg\:focus\:font-light:focus {
    font-weight: 300;
  }

  .lg\:focus\:font-normal:focus {
    font-weight: 400;
  }

  .lg\:focus\:font-medium:focus {
    font-weight: 500;
  }

  .lg\:focus\:font-semibold:focus {
    font-weight: 600;
  }

  .lg\:focus\:font-bold:focus {
    font-weight: 700;
  }

  .lg\:focus\:font-extrabold:focus {
    font-weight: 800;
  }

  .lg\:focus\:font-black:focus {
    font-weight: 900;
  }

  .lg\:h-0 {
    height: 0;
  }

  .lg\:h-1 {
    height: 0.25rem;
  }

  .lg\:h-2 {
    height: 0.5rem;
  }

  .lg\:h-3 {
    height: 0.75rem;
  }

  .lg\:h-4 {
    height: 1rem;
  }

  .lg\:h-5 {
    height: 1.25rem;
  }

  .lg\:h-6 {
    height: 1.5rem;
  }

  .lg\:h-8 {
    height: 2rem;
  }

  .lg\:h-10 {
    height: 2.5rem;
  }

  .lg\:h-12 {
    height: 3rem;
  }

  .lg\:h-16 {
    height: 4rem;
  }

  .lg\:h-20 {
    height: 5rem;
  }

  .lg\:h-24 {
    height: 6rem;
  }

  .lg\:h-32 {
    height: 8rem;
  }

  .lg\:h-40 {
    height: 10rem;
  }

  .lg\:h-48 {
    height: 12rem;
  }

  .lg\:h-56 {
    height: 14rem;
  }

  .lg\:h-64 {
    height: 16rem;
  }

  .lg\:h-auto {
    height: auto;
  }

  .lg\:h-px {
    height: 1px;
  }

  .lg\:h-full {
    height: 100%;
  }

  .lg\:h-screen {
    height: 100vh;
  }

  .lg\:leading-none {
    line-height: 1;
  }

  .lg\:leading-tight {
    line-height: 1.25;
  }

  .lg\:leading-snug {
    line-height: 1.375;
  }

  .lg\:leading-normal {
    line-height: 1.5;
  }

  .lg\:leading-relaxed {
    line-height: 1.625;
  }

  .lg\:leading-loose {
    line-height: 2;
  }

  .lg\:list-inside {
    list-style-position: inside;
  }

  .lg\:list-outside {
    list-style-position: outside;
  }

  .lg\:list-none {
    list-style-type: none;
  }

  .lg\:list-disc {
    list-style-type: disc;
  }

  .lg\:list-decimal {
    list-style-type: decimal;
  }

  .lg\:m-0 {
    margin: 0;
  }

  .lg\:m-1 {
    margin: 0.25rem;
  }

  .lg\:m-2 {
    margin: 0.5rem;
  }

  .lg\:m-3 {
    margin: 0.75rem;
  }

  .lg\:m-4 {
    margin: 1rem;
  }

  .lg\:m-5 {
    margin: 1.25rem;
  }

  .lg\:m-6 {
    margin: 1.5rem;
  }

  .lg\:m-8 {
    margin: 2rem;
  }

  .lg\:m-10 {
    margin: 2.5rem;
  }

  .lg\:m-12 {
    margin: 3rem;
  }

  .lg\:m-16 {
    margin: 4rem;
  }

  .lg\:m-20 {
    margin: 5rem;
  }

  .lg\:m-24 {
    margin: 6rem;
  }

  .lg\:m-32 {
    margin: 8rem;
  }

  .lg\:m-40 {
    margin: 10rem;
  }

  .lg\:m-48 {
    margin: 12rem;
  }

  .lg\:m-56 {
    margin: 14rem;
  }

  .lg\:m-64 {
    margin: 16rem;
  }

  .lg\:m-auto {
    margin: auto;
  }

  .lg\:m-px {
    margin: 1px;
  }

  .lg\:-m-1 {
    margin: -0.25rem;
  }

  .lg\:-m-2 {
    margin: -0.5rem;
  }

  .lg\:-m-3 {
    margin: -0.75rem;
  }

  .lg\:-m-4 {
    margin: -1rem;
  }

  .lg\:-m-5 {
    margin: -1.25rem;
  }

  .lg\:-m-6 {
    margin: -1.5rem;
  }

  .lg\:-m-8 {
    margin: -2rem;
  }

  .lg\:-m-10 {
    margin: -2.5rem;
  }

  .lg\:-m-12 {
    margin: -3rem;
  }

  .lg\:-m-16 {
    margin: -4rem;
  }

  .lg\:-m-20 {
    margin: -5rem;
  }

  .lg\:-m-24 {
    margin: -6rem;
  }

  .lg\:-m-32 {
    margin: -8rem;
  }

  .lg\:-m-40 {
    margin: -10rem;
  }

  .lg\:-m-48 {
    margin: -12rem;
  }

  .lg\:-m-56 {
    margin: -14rem;
  }

  .lg\:-m-64 {
    margin: -16rem;
  }

  .lg\:-m-px {
    margin: -1px;
  }

  .lg\:my-0 {
    margin-top: 0;
    margin-bottom: 0;
  }

  .lg\:mx-0 {
    margin-left: 0;
    margin-right: 0;
  }

  .lg\:my-1 {
    margin-top: 0.25rem;
    margin-bottom: 0.25rem;
  }

  .lg\:mx-1 {
    margin-left: 0.25rem;
    margin-right: 0.25rem;
  }

  .lg\:my-2 {
    margin-top: 0.5rem;
    margin-bottom: 0.5rem;
  }

  .lg\:mx-2 {
    margin-left: 0.5rem;
    margin-right: 0.5rem;
  }

  .lg\:my-3 {
    margin-top: 0.75rem;
    margin-bottom: 0.75rem;
  }

  .lg\:mx-3 {
    margin-left: 0.75rem;
    margin-right: 0.75rem;
  }

  .lg\:my-4 {
    margin-top: 1rem;
    margin-bottom: 1rem;
  }

  .lg\:mx-4 {
    margin-left: 1rem;
    margin-right: 1rem;
  }

  .lg\:my-5 {
    margin-top: 1.25rem;
    margin-bottom: 1.25rem;
  }

  .lg\:mx-5 {
    margin-left: 1.25rem;
    margin-right: 1.25rem;
  }

  .lg\:my-6 {
    margin-top: 1.5rem;
    margin-bottom: 1.5rem;
  }

  .lg\:mx-6 {
    margin-left: 1.5rem;
    margin-right: 1.5rem;
  }

  .lg\:my-8 {
    margin-top: 2rem;
    margin-bottom: 2rem;
  }

  .lg\:mx-8 {
    margin-left: 2rem;
    margin-right: 2rem;
  }

  .lg\:my-10 {
    margin-top: 2.5rem;
    margin-bottom: 2.5rem;
  }

  .lg\:mx-10 {
    margin-left: 2.5rem;
    margin-right: 2.5rem;
  }

  .lg\:my-12 {
    margin-top: 3rem;
    margin-bottom: 3rem;
  }

  .lg\:mx-12 {
    margin-left: 3rem;
    margin-right: 3rem;
  }

  .lg\:my-16 {
    margin-top: 4rem;
    margin-bottom: 4rem;
  }

  .lg\:mx-16 {
    margin-left: 4rem;
    margin-right: 4rem;
  }

  .lg\:my-20 {
    margin-top: 5rem;
    margin-bottom: 5rem;
  }

  .lg\:mx-20 {
    margin-left: 5rem;
    margin-right: 5rem;
  }

  .lg\:my-24 {
    margin-top: 6rem;
    margin-bottom: 6rem;
  }

  .lg\:mx-24 {
    margin-left: 6rem;
    margin-right: 6rem;
  }

  .lg\:my-32 {
    margin-top: 8rem;
    margin-bottom: 8rem;
  }

  .lg\:mx-32 {
    margin-left: 8rem;
    margin-right: 8rem;
  }

  .lg\:my-40 {
    margin-top: 10rem;
    margin-bottom: 10rem;
  }

  .lg\:mx-40 {
    margin-left: 10rem;
    margin-right: 10rem;
  }

  .lg\:my-48 {
    margin-top: 12rem;
    margin-bottom: 12rem;
  }

  .lg\:mx-48 {
    margin-left: 12rem;
    margin-right: 12rem;
  }

  .lg\:my-56 {
    margin-top: 14rem;
    margin-bottom: 14rem;
  }

  .lg\:mx-56 {
    margin-left: 14rem;
    margin-right: 14rem;
  }

  .lg\:my-64 {
    margin-top: 16rem;
    margin-bottom: 16rem;
  }

  .lg\:mx-64 {
    margin-left: 16rem;
    margin-right: 16rem;
  }

  .lg\:my-auto {
    margin-top: auto;
    margin-bottom: auto;
  }

  .lg\:mx-auto {
    margin-left: auto;
    margin-right: auto;
  }

  .lg\:my-px {
    margin-top: 1px;
    margin-bottom: 1px;
  }

  .lg\:mx-px {
    margin-left: 1px;
    margin-right: 1px;
  }

  .lg\:-my-1 {
    margin-top: -0.25rem;
    margin-bottom: -0.25rem;
  }

  .lg\:-mx-1 {
    margin-left: -0.25rem;
    margin-right: -0.25rem;
  }

  .lg\:-my-2 {
    margin-top: -0.5rem;
    margin-bottom: -0.5rem;
  }

  .lg\:-mx-2 {
    margin-left: -0.5rem;
    margin-right: -0.5rem;
  }

  .lg\:-my-3 {
    margin-top: -0.75rem;
    margin-bottom: -0.75rem;
  }

  .lg\:-mx-3 {
    margin-left: -0.75rem;
    margin-right: -0.75rem;
  }

  .lg\:-my-4 {
    margin-top: -1rem;
    margin-bottom: -1rem;
  }

  .lg\:-mx-4 {
    margin-left: -1rem;
    margin-right: -1rem;
  }

  .lg\:-my-5 {
    margin-top: -1.25rem;
    margin-bottom: -1.25rem;
  }

  .lg\:-mx-5 {
    margin-left: -1.25rem;
    margin-right: -1.25rem;
  }

  .lg\:-my-6 {
    margin-top: -1.5rem;
    margin-bottom: -1.5rem;
  }

  .lg\:-mx-6 {
    margin-left: -1.5rem;
    margin-right: -1.5rem;
  }

  .lg\:-my-8 {
    margin-top: -2rem;
    margin-bottom: -2rem;
  }

  .lg\:-mx-8 {
    margin-left: -2rem;
    margin-right: -2rem;
  }

  .lg\:-my-10 {
    margin-top: -2.5rem;
    margin-bottom: -2.5rem;
  }

  .lg\:-mx-10 {
    margin-left: -2.5rem;
    margin-right: -2.5rem;
  }

  .lg\:-my-12 {
    margin-top: -3rem;
    margin-bottom: -3rem;
  }

  .lg\:-mx-12 {
    margin-left: -3rem;
    margin-right: -3rem;
  }

  .lg\:-my-16 {
    margin-top: -4rem;
    margin-bottom: -4rem;
  }

  .lg\:-mx-16 {
    margin-left: -4rem;
    margin-right: -4rem;
  }

  .lg\:-my-20 {
    margin-top: -5rem;
    margin-bottom: -5rem;
  }

  .lg\:-mx-20 {
    margin-left: -5rem;
    margin-right: -5rem;
  }

  .lg\:-my-24 {
    margin-top: -6rem;
    margin-bottom: -6rem;
  }

  .lg\:-mx-24 {
    margin-left: -6rem;
    margin-right: -6rem;
  }

  .lg\:-my-32 {
    margin-top: -8rem;
    margin-bottom: -8rem;
  }

  .lg\:-mx-32 {
    margin-left: -8rem;
    margin-right: -8rem;
  }

  .lg\:-my-40 {
    margin-top: -10rem;
    margin-bottom: -10rem;
  }

  .lg\:-mx-40 {
    margin-left: -10rem;
    margin-right: -10rem;
  }

  .lg\:-my-48 {
    margin-top: -12rem;
    margin-bottom: -12rem;
  }

  .lg\:-mx-48 {
    margin-left: -12rem;
    margin-right: -12rem;
  }

  .lg\:-my-56 {
    margin-top: -14rem;
    margin-bottom: -14rem;
  }

  .lg\:-mx-56 {
    margin-left: -14rem;
    margin-right: -14rem;
  }

  .lg\:-my-64 {
    margin-top: -16rem;
    margin-bottom: -16rem;
  }

  .lg\:-mx-64 {
    margin-left: -16rem;
    margin-right: -16rem;
  }

  .lg\:-my-px {
    margin-top: -1px;
    margin-bottom: -1px;
  }

  .lg\:-mx-px {
    margin-left: -1px;
    margin-right: -1px;
  }

  .lg\:mt-0 {
    margin-top: 0;
  }

  .lg\:mr-0 {
    margin-right: 0;
  }

  .lg\:mb-0 {
    margin-bottom: 0;
  }

  .lg\:ml-0 {
    margin-left: 0;
  }

  .lg\:mt-1 {
    margin-top: 0.25rem;
  }

  .lg\:mr-1 {
    margin-right: 0.25rem;
  }

  .lg\:mb-1 {
    margin-bottom: 0.25rem;
  }

  .lg\:ml-1 {
    margin-left: 0.25rem;
  }

  .lg\:mt-2 {
    margin-top: 0.5rem;
  }

  .lg\:mr-2 {
    margin-right: 0.5rem;
  }

  .lg\:mb-2 {
    margin-bottom: 0.5rem;
  }

  .lg\:ml-2 {
    margin-left: 0.5rem;
  }

  .lg\:mt-3 {
    margin-top: 0.75rem;
  }

  .lg\:mr-3 {
    margin-right: 0.75rem;
  }

  .lg\:mb-3 {
    margin-bottom: 0.75rem;
  }

  .lg\:ml-3 {
    margin-left: 0.75rem;
  }

  .lg\:mt-4 {
    margin-top: 1rem;
  }

  .lg\:mr-4 {
    margin-right: 1rem;
  }

  .lg\:mb-4 {
    margin-bottom: 1rem;
  }

  .lg\:ml-4 {
    margin-left: 1rem;
  }

  .lg\:mt-5 {
    margin-top: 1.25rem;
  }

  .lg\:mr-5 {
    margin-right: 1.25rem;
  }

  .lg\:mb-5 {
    margin-bottom: 1.25rem;
  }

  .lg\:ml-5 {
    margin-left: 1.25rem;
  }

  .lg\:mt-6 {
    margin-top: 1.5rem;
  }

  .lg\:mr-6 {
    margin-right: 1.5rem;
  }

  .lg\:mb-6 {
    margin-bottom: 1.5rem;
  }

  .lg\:ml-6 {
    margin-left: 1.5rem;
  }

  .lg\:mt-8 {
    margin-top: 2rem;
  }

  .lg\:mr-8 {
    margin-right: 2rem;
  }

  .lg\:mb-8 {
    margin-bottom: 2rem;
  }

  .lg\:ml-8 {
    margin-left: 2rem;
  }

  .lg\:mt-10 {
    margin-top: 2.5rem;
  }

  .lg\:mr-10 {
    margin-right: 2.5rem;
  }

  .lg\:mb-10 {
    margin-bottom: 2.5rem;
  }

  .lg\:ml-10 {
    margin-left: 2.5rem;
  }

  .lg\:mt-12 {
    margin-top: 3rem;
  }

  .lg\:mr-12 {
    margin-right: 3rem;
  }

  .lg\:mb-12 {
    margin-bottom: 3rem;
  }

  .lg\:ml-12 {
    margin-left: 3rem;
  }

  .lg\:mt-16 {
    margin-top: 4rem;
  }

  .lg\:mr-16 {
    margin-right: 4rem;
  }

  .lg\:mb-16 {
    margin-bottom: 4rem;
  }

  .lg\:ml-16 {
    margin-left: 4rem;
  }

  .lg\:mt-20 {
    margin-top: 5rem;
  }

  .lg\:mr-20 {
    margin-right: 5rem;
  }

  .lg\:mb-20 {
    margin-bottom: 5rem;
  }

  .lg\:ml-20 {
    margin-left: 5rem;
  }

  .lg\:mt-24 {
    margin-top: 6rem;
  }

  .lg\:mr-24 {
    margin-right: 6rem;
  }

  .lg\:mb-24 {
    margin-bottom: 6rem;
  }

  .lg\:ml-24 {
    margin-left: 6rem;
  }

  .lg\:mt-32 {
    margin-top: 8rem;
  }

  .lg\:mr-32 {
    margin-right: 8rem;
  }

  .lg\:mb-32 {
    margin-bottom: 8rem;
  }

  .lg\:ml-32 {
    margin-left: 8rem;
  }

  .lg\:mt-40 {
    margin-top: 10rem;
  }

  .lg\:mr-40 {
    margin-right: 10rem;
  }

  .lg\:mb-40 {
    margin-bottom: 10rem;
  }

  .lg\:ml-40 {
    margin-left: 10rem;
  }

  .lg\:mt-48 {
    margin-top: 12rem;
  }

  .lg\:mr-48 {
    margin-right: 12rem;
  }

  .lg\:mb-48 {
    margin-bottom: 12rem;
  }

  .lg\:ml-48 {
    margin-left: 12rem;
  }

  .lg\:mt-56 {
    margin-top: 14rem;
  }

  .lg\:mr-56 {
    margin-right: 14rem;
  }

  .lg\:mb-56 {
    margin-bottom: 14rem;
  }

  .lg\:ml-56 {
    margin-left: 14rem;
  }

  .lg\:mt-64 {
    margin-top: 16rem;
  }

  .lg\:mr-64 {
    margin-right: 16rem;
  }

  .lg\:mb-64 {
    margin-bottom: 16rem;
  }

  .lg\:ml-64 {
    margin-left: 16rem;
  }

  .lg\:mt-auto {
    margin-top: auto;
  }

  .lg\:mr-auto {
    margin-right: auto;
  }

  .lg\:mb-auto {
    margin-bottom: auto;
  }

  .lg\:ml-auto {
    margin-left: auto;
  }

  .lg\:mt-px {
    margin-top: 1px;
  }

  .lg\:mr-px {
    margin-right: 1px;
  }

  .lg\:mb-px {
    margin-bottom: 1px;
  }

  .lg\:ml-px {
    margin-left: 1px;
  }

  .lg\:-mt-1 {
    margin-top: -0.25rem;
  }

  .lg\:-mr-1 {
    margin-right: -0.25rem;
  }

  .lg\:-mb-1 {
    margin-bottom: -0.25rem;
  }

  .lg\:-ml-1 {
    margin-left: -0.25rem;
  }

  .lg\:-mt-2 {
    margin-top: -0.5rem;
  }

  .lg\:-mr-2 {
    margin-right: -0.5rem;
  }

  .lg\:-mb-2 {
    margin-bottom: -0.5rem;
  }

  .lg\:-ml-2 {
    margin-left: -0.5rem;
  }

  .lg\:-mt-3 {
    margin-top: -0.75rem;
  }

  .lg\:-mr-3 {
    margin-right: -0.75rem;
  }

  .lg\:-mb-3 {
    margin-bottom: -0.75rem;
  }

  .lg\:-ml-3 {
    margin-left: -0.75rem;
  }

  .lg\:-mt-4 {
    margin-top: -1rem;
  }

  .lg\:-mr-4 {
    margin-right: -1rem;
  }

  .lg\:-mb-4 {
    margin-bottom: -1rem;
  }

  .lg\:-ml-4 {
    margin-left: -1rem;
  }

  .lg\:-mt-5 {
    margin-top: -1.25rem;
  }

  .lg\:-mr-5 {
    margin-right: -1.25rem;
  }

  .lg\:-mb-5 {
    margin-bottom: -1.25rem;
  }

  .lg\:-ml-5 {
    margin-left: -1.25rem;
  }

  .lg\:-mt-6 {
    margin-top: -1.5rem;
  }

  .lg\:-mr-6 {
    margin-right: -1.5rem;
  }

  .lg\:-mb-6 {
    margin-bottom: -1.5rem;
  }

  .lg\:-ml-6 {
    margin-left: -1.5rem;
  }

  .lg\:-mt-8 {
    margin-top: -2rem;
  }

  .lg\:-mr-8 {
    margin-right: -2rem;
  }

  .lg\:-mb-8 {
    margin-bottom: -2rem;
  }

  .lg\:-ml-8 {
    margin-left: -2rem;
  }

  .lg\:-mt-10 {
    margin-top: -2.5rem;
  }

  .lg\:-mr-10 {
    margin-right: -2.5rem;
  }

  .lg\:-mb-10 {
    margin-bottom: -2.5rem;
  }

  .lg\:-ml-10 {
    margin-left: -2.5rem;
  }

  .lg\:-mt-12 {
    margin-top: -3rem;
  }

  .lg\:-mr-12 {
    margin-right: -3rem;
  }

  .lg\:-mb-12 {
    margin-bottom: -3rem;
  }

  .lg\:-ml-12 {
    margin-left: -3rem;
  }

  .lg\:-mt-16 {
    margin-top: -4rem;
  }

  .lg\:-mr-16 {
    margin-right: -4rem;
  }

  .lg\:-mb-16 {
    margin-bottom: -4rem;
  }

  .lg\:-ml-16 {
    margin-left: -4rem;
  }

  .lg\:-mt-20 {
    margin-top: -5rem;
  }

  .lg\:-mr-20 {
    margin-right: -5rem;
  }

  .lg\:-mb-20 {
    margin-bottom: -5rem;
  }

  .lg\:-ml-20 {
    margin-left: -5rem;
  }

  .lg\:-mt-24 {
    margin-top: -6rem;
  }

  .lg\:-mr-24 {
    margin-right: -6rem;
  }

  .lg\:-mb-24 {
    margin-bottom: -6rem;
  }

  .lg\:-ml-24 {
    margin-left: -6rem;
  }

  .lg\:-mt-32 {
    margin-top: -8rem;
  }

  .lg\:-mr-32 {
    margin-right: -8rem;
  }

  .lg\:-mb-32 {
    margin-bottom: -8rem;
  }

  .lg\:-ml-32 {
    margin-left: -8rem;
  }

  .lg\:-mt-40 {
    margin-top: -10rem;
  }

  .lg\:-mr-40 {
    margin-right: -10rem;
  }

  .lg\:-mb-40 {
    margin-bottom: -10rem;
  }

  .lg\:-ml-40 {
    margin-left: -10rem;
  }

  .lg\:-mt-48 {
    margin-top: -12rem;
  }

  .lg\:-mr-48 {
    margin-right: -12rem;
  }

  .lg\:-mb-48 {
    margin-bottom: -12rem;
  }

  .lg\:-ml-48 {
    margin-left: -12rem;
  }

  .lg\:-mt-56 {
    margin-top: -14rem;
  }

  .lg\:-mr-56 {
    margin-right: -14rem;
  }

  .lg\:-mb-56 {
    margin-bottom: -14rem;
  }

  .lg\:-ml-56 {
    margin-left: -14rem;
  }

  .lg\:-mt-64 {
    margin-top: -16rem;
  }

  .lg\:-mr-64 {
    margin-right: -16rem;
  }

  .lg\:-mb-64 {
    margin-bottom: -16rem;
  }

  .lg\:-ml-64 {
    margin-left: -16rem;
  }

  .lg\:-mt-px {
    margin-top: -1px;
  }

  .lg\:-mr-px {
    margin-right: -1px;
  }

  .lg\:-mb-px {
    margin-bottom: -1px;
  }

  .lg\:-ml-px {
    margin-left: -1px;
  }

  .lg\:max-h-full {
    max-height: 100%;
  }

  .lg\:max-h-screen {
    max-height: 100vh;
  }

  .lg\:max-w-xs {
    max-width: 20rem;
  }

  .lg\:max-w-sm {
    max-width: 24rem;
  }

  .lg\:max-w-md {
    max-width: 28rem;
  }

  .lg\:max-w-lg {
    max-width: 32rem;
  }

  .lg\:max-w-xl {
    max-width: 36rem;
  }

  .lg\:max-w-2xl {
    max-width: 42rem;
  }

  .lg\:max-w-3xl {
    max-width: 48rem;
  }

  .lg\:max-w-4xl {
    max-width: 56rem;
  }

  .lg\:max-w-5xl {
    max-width: 64rem;
  }

  .lg\:max-w-6xl {
    max-width: 72rem;
  }

  .lg\:max-w-full {
    max-width: 100%;
  }

  .lg\:min-h-0 {
    min-height: 0;
  }

  .lg\:min-h-full {
    min-height: 100%;
  }

  .lg\:min-h-screen {
    min-height: 100vh;
  }

  .lg\:min-w-0 {
    min-width: 0;
  }

  .lg\:min-w-full {
    min-width: 100%;
  }

  .lg\:object-contain {
    -o-object-fit: contain;
       object-fit: contain;
  }

  .lg\:object-cover {
    -o-object-fit: cover;
       object-fit: cover;
  }

  .lg\:object-fill {
    -o-object-fit: fill;
       object-fit: fill;
  }

  .lg\:object-none {
    -o-object-fit: none;
       object-fit: none;
  }

  .lg\:object-scale-down {
    -o-object-fit: scale-down;
       object-fit: scale-down;
  }

  .lg\:object-bottom {
    -o-object-position: bottom;
       object-position: bottom;
  }

  .lg\:object-center {
    -o-object-position: center;
       object-position: center;
  }

  .lg\:object-left {
    -o-object-position: left;
       object-position: left;
  }

  .lg\:object-left-bottom {
    -o-object-position: left bottom;
       object-position: left bottom;
  }

  .lg\:object-left-top {
    -o-object-position: left top;
       object-position: left top;
  }

  .lg\:object-right {
    -o-object-position: right;
       object-position: right;
  }

  .lg\:object-right-bottom {
    -o-object-position: right bottom;
       object-position: right bottom;
  }

  .lg\:object-right-top {
    -o-object-position: right top;
       object-position: right top;
  }

  .lg\:object-top {
    -o-object-position: top;
       object-position: top;
  }

  .lg\:opacity-0 {
    opacity: 0;
  }

  .lg\:opacity-25 {
    opacity: 0.25;
  }

  .lg\:opacity-50 {
    opacity: 0.5;
  }

  .lg\:opacity-75 {
    opacity: 0.75;
  }

  .lg\:opacity-100 {
    opacity: 1;
  }

  .lg\:hover\:opacity-0:hover {
    opacity: 0;
  }

  .lg\:hover\:opacity-25:hover {
    opacity: 0.25;
  }

  .lg\:hover\:opacity-50:hover {
    opacity: 0.5;
  }

  .lg\:hover\:opacity-75:hover {
    opacity: 0.75;
  }

  .lg\:hover\:opacity-100:hover {
    opacity: 1;
  }

  .lg\:focus\:opacity-0:focus {
    opacity: 0;
  }

  .lg\:focus\:opacity-25:focus {
    opacity: 0.25;
  }

  .lg\:focus\:opacity-50:focus {
    opacity: 0.5;
  }

  .lg\:focus\:opacity-75:focus {
    opacity: 0.75;
  }

  .lg\:focus\:opacity-100:focus {
    opacity: 1;
  }

  .lg\:outline-none {
    outline: 0;
  }

  .lg\:focus\:outline-none:focus {
    outline: 0;
  }

  .lg\:overflow-auto {
    overflow: auto;
  }

  .lg\:overflow-hidden {
    overflow: hidden;
  }

  .lg\:overflow-visible {
    overflow: visible;
  }

  .lg\:overflow-scroll {
    overflow: scroll;
  }

  .lg\:overflow-x-auto {
    overflow-x: auto;
  }

  .lg\:overflow-y-auto {
    overflow-y: auto;
  }

  .lg\:overflow-x-hidden {
    overflow-x: hidden;
  }

  .lg\:overflow-y-hidden {
    overflow-y: hidden;
  }

  .lg\:overflow-x-visible {
    overflow-x: visible;
  }

  .lg\:overflow-y-visible {
    overflow-y: visible;
  }

  .lg\:overflow-x-scroll {
    overflow-x: scroll;
  }

  .lg\:overflow-y-scroll {
    overflow-y: scroll;
  }

  .lg\:scrolling-touch {
    -webkit-overflow-scrolling: touch;
  }

  .lg\:scrolling-auto {
    -webkit-overflow-scrolling: auto;
  }

  .lg\:p-0 {
    padding: 0;
  }

  .lg\:p-1 {
    padding: 0.25rem;
  }

  .lg\:p-2 {
    padding: 0.5rem;
  }

  .lg\:p-3 {
    padding: 0.75rem;
  }

  .lg\:p-4 {
    padding: 1rem;
  }

  .lg\:p-5 {
    padding: 1.25rem;
  }

  .lg\:p-6 {
    padding: 1.5rem;
  }

  .lg\:p-8 {
    padding: 2rem;
  }

  .lg\:p-10 {
    padding: 2.5rem;
  }

  .lg\:p-12 {
    padding: 3rem;
  }

  .lg\:p-16 {
    padding: 4rem;
  }

  .lg\:p-20 {
    padding: 5rem;
  }

  .lg\:p-24 {
    padding: 6rem;
  }

  .lg\:p-32 {
    padding: 8rem;
  }

  .lg\:p-40 {
    padding: 10rem;
  }

  .lg\:p-48 {
    padding: 12rem;
  }

  .lg\:p-56 {
    padding: 14rem;
  }

  .lg\:p-64 {
    padding: 16rem;
  }

  .lg\:p-px {
    padding: 1px;
  }

  .lg\:py-0 {
    padding-top: 0;
    padding-bottom: 0;
  }

  .lg\:px-0 {
    padding-left: 0;
    padding-right: 0;
  }

  .lg\:py-1 {
    padding-top: 0.25rem;
    padding-bottom: 0.25rem;
  }

  .lg\:px-1 {
    padding-left: 0.25rem;
    padding-right: 0.25rem;
  }

  .lg\:py-2 {
    padding-top: 0.5rem;
    padding-bottom: 0.5rem;
  }

  .lg\:px-2 {
    padding-left: 0.5rem;
    padding-right: 0.5rem;
  }

  .lg\:py-3 {
    padding-top: 0.75rem;
    padding-bottom: 0.75rem;
  }

  .lg\:px-3 {
    padding-left: 0.75rem;
    padding-right: 0.75rem;
  }

  .lg\:py-4 {
    padding-top: 1rem;
    padding-bottom: 1rem;
  }

  .lg\:px-4 {
    padding-left: 1rem;
    padding-right: 1rem;
  }

  .lg\:py-5 {
    padding-top: 1.25rem;
    padding-bottom: 1.25rem;
  }

  .lg\:px-5 {
    padding-left: 1.25rem;
    padding-right: 1.25rem;
  }

  .lg\:py-6 {
    padding-top: 1.5rem;
    padding-bottom: 1.5rem;
  }

  .lg\:px-6 {
    padding-left: 1.5rem;
    padding-right: 1.5rem;
  }

  .lg\:py-8 {
    padding-top: 2rem;
    padding-bottom: 2rem;
  }

  .lg\:px-8 {
    padding-left: 2rem;
    padding-right: 2rem;
  }

  .lg\:py-10 {
    padding-top: 2.5rem;
    padding-bottom: 2.5rem;
  }

  .lg\:px-10 {
    padding-left: 2.5rem;
    padding-right: 2.5rem;
  }

  .lg\:py-12 {
    padding-top: 3rem;
    padding-bottom: 3rem;
  }

  .lg\:px-12 {
    padding-left: 3rem;
    padding-right: 3rem;
  }

  .lg\:py-16 {
    padding-top: 4rem;
    padding-bottom: 4rem;
  }

  .lg\:px-16 {
    padding-left: 4rem;
    padding-right: 4rem;
  }

  .lg\:py-20 {
    padding-top: 5rem;
    padding-bottom: 5rem;
  }

  .lg\:px-20 {
    padding-left: 5rem;
    padding-right: 5rem;
  }

  .lg\:py-24 {
    padding-top: 6rem;
    padding-bottom: 6rem;
  }

  .lg\:px-24 {
    padding-left: 6rem;
    padding-right: 6rem;
  }

  .lg\:py-32 {
    padding-top: 8rem;
    padding-bottom: 8rem;
  }

  .lg\:px-32 {
    padding-left: 8rem;
    padding-right: 8rem;
  }

  .lg\:py-40 {
    padding-top: 10rem;
    padding-bottom: 10rem;
  }

  .lg\:px-40 {
    padding-left: 10rem;
    padding-right: 10rem;
  }

  .lg\:py-48 {
    padding-top: 12rem;
    padding-bottom: 12rem;
  }

  .lg\:px-48 {
    padding-left: 12rem;
    padding-right: 12rem;
  }

  .lg\:py-56 {
    padding-top: 14rem;
    padding-bottom: 14rem;
  }

  .lg\:px-56 {
    padding-left: 14rem;
    padding-right: 14rem;
  }

  .lg\:py-64 {
    padding-top: 16rem;
    padding-bottom: 16rem;
  }

  .lg\:px-64 {
    padding-left: 16rem;
    padding-right: 16rem;
  }

  .lg\:py-px {
    padding-top: 1px;
    padding-bottom: 1px;
  }

  .lg\:px-px {
    padding-left: 1px;
    padding-right: 1px;
  }

  .lg\:pt-0 {
    padding-top: 0;
  }

  .lg\:pr-0 {
    padding-right: 0;
  }

  .lg\:pb-0 {
    padding-bottom: 0;
  }

  .lg\:pl-0 {
    padding-left: 0;
  }

  .lg\:pt-1 {
    padding-top: 0.25rem;
  }

  .lg\:pr-1 {
    padding-right: 0.25rem;
  }

  .lg\:pb-1 {
    padding-bottom: 0.25rem;
  }

  .lg\:pl-1 {
    padding-left: 0.25rem;
  }

  .lg\:pt-2 {
    padding-top: 0.5rem;
  }

  .lg\:pr-2 {
    padding-right: 0.5rem;
  }

  .lg\:pb-2 {
    padding-bottom: 0.5rem;
  }

  .lg\:pl-2 {
    padding-left: 0.5rem;
  }

  .lg\:pt-3 {
    padding-top: 0.75rem;
  }

  .lg\:pr-3 {
    padding-right: 0.75rem;
  }

  .lg\:pb-3 {
    padding-bottom: 0.75rem;
  }

  .lg\:pl-3 {
    padding-left: 0.75rem;
  }

  .lg\:pt-4 {
    padding-top: 1rem;
  }

  .lg\:pr-4 {
    padding-right: 1rem;
  }

  .lg\:pb-4 {
    padding-bottom: 1rem;
  }

  .lg\:pl-4 {
    padding-left: 1rem;
  }

  .lg\:pt-5 {
    padding-top: 1.25rem;
  }

  .lg\:pr-5 {
    padding-right: 1.25rem;
  }

  .lg\:pb-5 {
    padding-bottom: 1.25rem;
  }

  .lg\:pl-5 {
    padding-left: 1.25rem;
  }

  .lg\:pt-6 {
    padding-top: 1.5rem;
  }

  .lg\:pr-6 {
    padding-right: 1.5rem;
  }

  .lg\:pb-6 {
    padding-bottom: 1.5rem;
  }

  .lg\:pl-6 {
    padding-left: 1.5rem;
  }

  .lg\:pt-8 {
    padding-top: 2rem;
  }

  .lg\:pr-8 {
    padding-right: 2rem;
  }

  .lg\:pb-8 {
    padding-bottom: 2rem;
  }

  .lg\:pl-8 {
    padding-left: 2rem;
  }

  .lg\:pt-10 {
    padding-top: 2.5rem;
  }

  .lg\:pr-10 {
    padding-right: 2.5rem;
  }

  .lg\:pb-10 {
    padding-bottom: 2.5rem;
  }

  .lg\:pl-10 {
    padding-left: 2.5rem;
  }

  .lg\:pt-12 {
    padding-top: 3rem;
  }

  .lg\:pr-12 {
    padding-right: 3rem;
  }

  .lg\:pb-12 {
    padding-bottom: 3rem;
  }

  .lg\:pl-12 {
    padding-left: 3rem;
  }

  .lg\:pt-16 {
    padding-top: 4rem;
  }

  .lg\:pr-16 {
    padding-right: 4rem;
  }

  .lg\:pb-16 {
    padding-bottom: 4rem;
  }

  .lg\:pl-16 {
    padding-left: 4rem;
  }

  .lg\:pt-20 {
    padding-top: 5rem;
  }

  .lg\:pr-20 {
    padding-right: 5rem;
  }

  .lg\:pb-20 {
    padding-bottom: 5rem;
  }

  .lg\:pl-20 {
    padding-left: 5rem;
  }

  .lg\:pt-24 {
    padding-top: 6rem;
  }

  .lg\:pr-24 {
    padding-right: 6rem;
  }

  .lg\:pb-24 {
    padding-bottom: 6rem;
  }

  .lg\:pl-24 {
    padding-left: 6rem;
  }

  .lg\:pt-32 {
    padding-top: 8rem;
  }

  .lg\:pr-32 {
    padding-right: 8rem;
  }

  .lg\:pb-32 {
    padding-bottom: 8rem;
  }

  .lg\:pl-32 {
    padding-left: 8rem;
  }

  .lg\:pt-40 {
    padding-top: 10rem;
  }

  .lg\:pr-40 {
    padding-right: 10rem;
  }

  .lg\:pb-40 {
    padding-bottom: 10rem;
  }

  .lg\:pl-40 {
    padding-left: 10rem;
  }

  .lg\:pt-48 {
    padding-top: 12rem;
  }

  .lg\:pr-48 {
    padding-right: 12rem;
  }

  .lg\:pb-48 {
    padding-bottom: 12rem;
  }

  .lg\:pl-48 {
    padding-left: 12rem;
  }

  .lg\:pt-56 {
    padding-top: 14rem;
  }

  .lg\:pr-56 {
    padding-right: 14rem;
  }

  .lg\:pb-56 {
    padding-bottom: 14rem;
  }

  .lg\:pl-56 {
    padding-left: 14rem;
  }

  .lg\:pt-64 {
    padding-top: 16rem;
  }

  .lg\:pr-64 {
    padding-right: 16rem;
  }

  .lg\:pb-64 {
    padding-bottom: 16rem;
  }

  .lg\:pl-64 {
    padding-left: 16rem;
  }

  .lg\:pt-px {
    padding-top: 1px;
  }

  .lg\:pr-px {
    padding-right: 1px;
  }

  .lg\:pb-px {
    padding-bottom: 1px;
  }

  .lg\:pl-px {
    padding-left: 1px;
  }

  .lg\:placeholder-transparent::-webkit-input-placeholder {
    color: transparent;
  }

  .lg\:placeholder-transparent::-moz-placeholder {
    color: transparent;
  }

  .lg\:placeholder-transparent:-ms-input-placeholder {
    color: transparent;
  }

  .lg\:placeholder-transparent::-ms-input-placeholder {
    color: transparent;
  }

  .lg\:placeholder-transparent::placeholder {
    color: transparent;
  }

  .lg\:placeholder-black::-webkit-input-placeholder {
    color: #000;
  }

  .lg\:placeholder-black::-moz-placeholder {
    color: #000;
  }

  .lg\:placeholder-black:-ms-input-placeholder {
    color: #000;
  }

  .lg\:placeholder-black::-ms-input-placeholder {
    color: #000;
  }

  .lg\:placeholder-black::placeholder {
    color: #000;
  }

  .lg\:placeholder-white::-webkit-input-placeholder {
    color: #fff;
  }

  .lg\:placeholder-white::-moz-placeholder {
    color: #fff;
  }

  .lg\:placeholder-white:-ms-input-placeholder {
    color: #fff;
  }

  .lg\:placeholder-white::-ms-input-placeholder {
    color: #fff;
  }

  .lg\:placeholder-white::placeholder {
    color: #fff;
  }

  .lg\:placeholder-gray-100::-webkit-input-placeholder {
    color: #f7fafc;
  }

  .lg\:placeholder-gray-100::-moz-placeholder {
    color: #f7fafc;
  }

  .lg\:placeholder-gray-100:-ms-input-placeholder {
    color: #f7fafc;
  }

  .lg\:placeholder-gray-100::-ms-input-placeholder {
    color: #f7fafc;
  }

  .lg\:placeholder-gray-100::placeholder {
    color: #f7fafc;
  }

  .lg\:placeholder-gray-200::-webkit-input-placeholder {
    color: #edf2f7;
  }

  .lg\:placeholder-gray-200::-moz-placeholder {
    color: #edf2f7;
  }

  .lg\:placeholder-gray-200:-ms-input-placeholder {
    color: #edf2f7;
  }

  .lg\:placeholder-gray-200::-ms-input-placeholder {
    color: #edf2f7;
  }

  .lg\:placeholder-gray-200::placeholder {
    color: #edf2f7;
  }

  .lg\:placeholder-gray-300::-webkit-input-placeholder {
    color: #e2e8f0;
  }

  .lg\:placeholder-gray-300::-moz-placeholder {
    color: #e2e8f0;
  }

  .lg\:placeholder-gray-300:-ms-input-placeholder {
    color: #e2e8f0;
  }

  .lg\:placeholder-gray-300::-ms-input-placeholder {
    color: #e2e8f0;
  }

  .lg\:placeholder-gray-300::placeholder {
    color: #e2e8f0;
  }

  .lg\:placeholder-gray-400::-webkit-input-placeholder {
    color: #cbd5e0;
  }

  .lg\:placeholder-gray-400::-moz-placeholder {
    color: #cbd5e0;
  }

  .lg\:placeholder-gray-400:-ms-input-placeholder {
    color: #cbd5e0;
  }

  .lg\:placeholder-gray-400::-ms-input-placeholder {
    color: #cbd5e0;
  }

  .lg\:placeholder-gray-400::placeholder {
    color: #cbd5e0;
  }

  .lg\:placeholder-gray-500::-webkit-input-placeholder {
    color: #a0aec0;
  }

  .lg\:placeholder-gray-500::-moz-placeholder {
    color: #a0aec0;
  }

  .lg\:placeholder-gray-500:-ms-input-placeholder {
    color: #a0aec0;
  }

  .lg\:placeholder-gray-500::-ms-input-placeholder {
    color: #a0aec0;
  }

  .lg\:placeholder-gray-500::placeholder {
    color: #a0aec0;
  }

  .lg\:placeholder-gray-600::-webkit-input-placeholder {
    color: #718096;
  }

  .lg\:placeholder-gray-600::-moz-placeholder {
    color: #718096;
  }

  .lg\:placeholder-gray-600:-ms-input-placeholder {
    color: #718096;
  }

  .lg\:placeholder-gray-600::-ms-input-placeholder {
    color: #718096;
  }

  .lg\:placeholder-gray-600::placeholder {
    color: #718096;
  }

  .lg\:placeholder-gray-700::-webkit-input-placeholder {
    color: #4a5568;
  }

  .lg\:placeholder-gray-700::-moz-placeholder {
    color: #4a5568;
  }

  .lg\:placeholder-gray-700:-ms-input-placeholder {
    color: #4a5568;
  }

  .lg\:placeholder-gray-700::-ms-input-placeholder {
    color: #4a5568;
  }

  .lg\:placeholder-gray-700::placeholder {
    color: #4a5568;
  }

  .lg\:placeholder-gray-800::-webkit-input-placeholder {
    color: #2d3748;
  }

  .lg\:placeholder-gray-800::-moz-placeholder {
    color: #2d3748;
  }

  .lg\:placeholder-gray-800:-ms-input-placeholder {
    color: #2d3748;
  }

  .lg\:placeholder-gray-800::-ms-input-placeholder {
    color: #2d3748;
  }

  .lg\:placeholder-gray-800::placeholder {
    color: #2d3748;
  }

  .lg\:placeholder-gray-900::-webkit-input-placeholder {
    color: #1a202c;
  }

  .lg\:placeholder-gray-900::-moz-placeholder {
    color: #1a202c;
  }

  .lg\:placeholder-gray-900:-ms-input-placeholder {
    color: #1a202c;
  }

  .lg\:placeholder-gray-900::-ms-input-placeholder {
    color: #1a202c;
  }

  .lg\:placeholder-gray-900::placeholder {
    color: #1a202c;
  }

  .lg\:placeholder-red-100::-webkit-input-placeholder {
    color: #fff5f5;
  }

  .lg\:placeholder-red-100::-moz-placeholder {
    color: #fff5f5;
  }

  .lg\:placeholder-red-100:-ms-input-placeholder {
    color: #fff5f5;
  }

  .lg\:placeholder-red-100::-ms-input-placeholder {
    color: #fff5f5;
  }

  .lg\:placeholder-red-100::placeholder {
    color: #fff5f5;
  }

  .lg\:placeholder-red-200::-webkit-input-placeholder {
    color: #fed7d7;
  }

  .lg\:placeholder-red-200::-moz-placeholder {
    color: #fed7d7;
  }

  .lg\:placeholder-red-200:-ms-input-placeholder {
    color: #fed7d7;
  }

  .lg\:placeholder-red-200::-ms-input-placeholder {
    color: #fed7d7;
  }

  .lg\:placeholder-red-200::placeholder {
    color: #fed7d7;
  }

  .lg\:placeholder-red-300::-webkit-input-placeholder {
    color: #feb2b2;
  }

  .lg\:placeholder-red-300::-moz-placeholder {
    color: #feb2b2;
  }

  .lg\:placeholder-red-300:-ms-input-placeholder {
    color: #feb2b2;
  }

  .lg\:placeholder-red-300::-ms-input-placeholder {
    color: #feb2b2;
  }

  .lg\:placeholder-red-300::placeholder {
    color: #feb2b2;
  }

  .lg\:placeholder-red-400::-webkit-input-placeholder {
    color: #fc8181;
  }

  .lg\:placeholder-red-400::-moz-placeholder {
    color: #fc8181;
  }

  .lg\:placeholder-red-400:-ms-input-placeholder {
    color: #fc8181;
  }

  .lg\:placeholder-red-400::-ms-input-placeholder {
    color: #fc8181;
  }

  .lg\:placeholder-red-400::placeholder {
    color: #fc8181;
  }

  .lg\:placeholder-red-500::-webkit-input-placeholder {
    color: #f56565;
  }

  .lg\:placeholder-red-500::-moz-placeholder {
    color: #f56565;
  }

  .lg\:placeholder-red-500:-ms-input-placeholder {
    color: #f56565;
  }

  .lg\:placeholder-red-500::-ms-input-placeholder {
    color: #f56565;
  }

  .lg\:placeholder-red-500::placeholder {
    color: #f56565;
  }

  .lg\:placeholder-red-600::-webkit-input-placeholder {
    color: #e53e3e;
  }

  .lg\:placeholder-red-600::-moz-placeholder {
    color: #e53e3e;
  }

  .lg\:placeholder-red-600:-ms-input-placeholder {
    color: #e53e3e;
  }

  .lg\:placeholder-red-600::-ms-input-placeholder {
    color: #e53e3e;
  }

  .lg\:placeholder-red-600::placeholder {
    color: #e53e3e;
  }

  .lg\:placeholder-red-700::-webkit-input-placeholder {
    color: #c53030;
  }

  .lg\:placeholder-red-700::-moz-placeholder {
    color: #c53030;
  }

  .lg\:placeholder-red-700:-ms-input-placeholder {
    color: #c53030;
  }

  .lg\:placeholder-red-700::-ms-input-placeholder {
    color: #c53030;
  }

  .lg\:placeholder-red-700::placeholder {
    color: #c53030;
  }

  .lg\:placeholder-red-800::-webkit-input-placeholder {
    color: #9b2c2c;
  }

  .lg\:placeholder-red-800::-moz-placeholder {
    color: #9b2c2c;
  }

  .lg\:placeholder-red-800:-ms-input-placeholder {
    color: #9b2c2c;
  }

  .lg\:placeholder-red-800::-ms-input-placeholder {
    color: #9b2c2c;
  }

  .lg\:placeholder-red-800::placeholder {
    color: #9b2c2c;
  }

  .lg\:placeholder-red-900::-webkit-input-placeholder {
    color: #742a2a;
  }

  .lg\:placeholder-red-900::-moz-placeholder {
    color: #742a2a;
  }

  .lg\:placeholder-red-900:-ms-input-placeholder {
    color: #742a2a;
  }

  .lg\:placeholder-red-900::-ms-input-placeholder {
    color: #742a2a;
  }

  .lg\:placeholder-red-900::placeholder {
    color: #742a2a;
  }

  .lg\:placeholder-orange-100::-webkit-input-placeholder {
    color: #fffaf0;
  }

  .lg\:placeholder-orange-100::-moz-placeholder {
    color: #fffaf0;
  }

  .lg\:placeholder-orange-100:-ms-input-placeholder {
    color: #fffaf0;
  }

  .lg\:placeholder-orange-100::-ms-input-placeholder {
    color: #fffaf0;
  }

  .lg\:placeholder-orange-100::placeholder {
    color: #fffaf0;
  }

  .lg\:placeholder-orange-200::-webkit-input-placeholder {
    color: #feebc8;
  }

  .lg\:placeholder-orange-200::-moz-placeholder {
    color: #feebc8;
  }

  .lg\:placeholder-orange-200:-ms-input-placeholder {
    color: #feebc8;
  }

  .lg\:placeholder-orange-200::-ms-input-placeholder {
    color: #feebc8;
  }

  .lg\:placeholder-orange-200::placeholder {
    color: #feebc8;
  }

  .lg\:placeholder-orange-300::-webkit-input-placeholder {
    color: #fbd38d;
  }

  .lg\:placeholder-orange-300::-moz-placeholder {
    color: #fbd38d;
  }

  .lg\:placeholder-orange-300:-ms-input-placeholder {
    color: #fbd38d;
  }

  .lg\:placeholder-orange-300::-ms-input-placeholder {
    color: #fbd38d;
  }

  .lg\:placeholder-orange-300::placeholder {
    color: #fbd38d;
  }

  .lg\:placeholder-orange-400::-webkit-input-placeholder {
    color: #f6ad55;
  }

  .lg\:placeholder-orange-400::-moz-placeholder {
    color: #f6ad55;
  }

  .lg\:placeholder-orange-400:-ms-input-placeholder {
    color: #f6ad55;
  }

  .lg\:placeholder-orange-400::-ms-input-placeholder {
    color: #f6ad55;
  }

  .lg\:placeholder-orange-400::placeholder {
    color: #f6ad55;
  }

  .lg\:placeholder-orange-500::-webkit-input-placeholder {
    color: #ed8936;
  }

  .lg\:placeholder-orange-500::-moz-placeholder {
    color: #ed8936;
  }

  .lg\:placeholder-orange-500:-ms-input-placeholder {
    color: #ed8936;
  }

  .lg\:placeholder-orange-500::-ms-input-placeholder {
    color: #ed8936;
  }

  .lg\:placeholder-orange-500::placeholder {
    color: #ed8936;
  }

  .lg\:placeholder-orange-600::-webkit-input-placeholder {
    color: #dd6b20;
  }

  .lg\:placeholder-orange-600::-moz-placeholder {
    color: #dd6b20;
  }

  .lg\:placeholder-orange-600:-ms-input-placeholder {
    color: #dd6b20;
  }

  .lg\:placeholder-orange-600::-ms-input-placeholder {
    color: #dd6b20;
  }

  .lg\:placeholder-orange-600::placeholder {
    color: #dd6b20;
  }

  .lg\:placeholder-orange-700::-webkit-input-placeholder {
    color: #c05621;
  }

  .lg\:placeholder-orange-700::-moz-placeholder {
    color: #c05621;
  }

  .lg\:placeholder-orange-700:-ms-input-placeholder {
    color: #c05621;
  }

  .lg\:placeholder-orange-700::-ms-input-placeholder {
    color: #c05621;
  }

  .lg\:placeholder-orange-700::placeholder {
    color: #c05621;
  }

  .lg\:placeholder-orange-800::-webkit-input-placeholder {
    color: #9c4221;
  }

  .lg\:placeholder-orange-800::-moz-placeholder {
    color: #9c4221;
  }

  .lg\:placeholder-orange-800:-ms-input-placeholder {
    color: #9c4221;
  }

  .lg\:placeholder-orange-800::-ms-input-placeholder {
    color: #9c4221;
  }

  .lg\:placeholder-orange-800::placeholder {
    color: #9c4221;
  }

  .lg\:placeholder-orange-900::-webkit-input-placeholder {
    color: #7b341e;
  }

  .lg\:placeholder-orange-900::-moz-placeholder {
    color: #7b341e;
  }

  .lg\:placeholder-orange-900:-ms-input-placeholder {
    color: #7b341e;
  }

  .lg\:placeholder-orange-900::-ms-input-placeholder {
    color: #7b341e;
  }

  .lg\:placeholder-orange-900::placeholder {
    color: #7b341e;
  }

  .lg\:placeholder-yellow-100::-webkit-input-placeholder {
    color: #fffff0;
  }

  .lg\:placeholder-yellow-100::-moz-placeholder {
    color: #fffff0;
  }

  .lg\:placeholder-yellow-100:-ms-input-placeholder {
    color: #fffff0;
  }

  .lg\:placeholder-yellow-100::-ms-input-placeholder {
    color: #fffff0;
  }

  .lg\:placeholder-yellow-100::placeholder {
    color: #fffff0;
  }

  .lg\:placeholder-yellow-200::-webkit-input-placeholder {
    color: #fefcbf;
  }

  .lg\:placeholder-yellow-200::-moz-placeholder {
    color: #fefcbf;
  }

  .lg\:placeholder-yellow-200:-ms-input-placeholder {
    color: #fefcbf;
  }

  .lg\:placeholder-yellow-200::-ms-input-placeholder {
    color: #fefcbf;
  }

  .lg\:placeholder-yellow-200::placeholder {
    color: #fefcbf;
  }

  .lg\:placeholder-yellow-300::-webkit-input-placeholder {
    color: #faf089;
  }

  .lg\:placeholder-yellow-300::-moz-placeholder {
    color: #faf089;
  }

  .lg\:placeholder-yellow-300:-ms-input-placeholder {
    color: #faf089;
  }

  .lg\:placeholder-yellow-300::-ms-input-placeholder {
    color: #faf089;
  }

  .lg\:placeholder-yellow-300::placeholder {
    color: #faf089;
  }

  .lg\:placeholder-yellow-400::-webkit-input-placeholder {
    color: #f6e05e;
  }

  .lg\:placeholder-yellow-400::-moz-placeholder {
    color: #f6e05e;
  }

  .lg\:placeholder-yellow-400:-ms-input-placeholder {
    color: #f6e05e;
  }

  .lg\:placeholder-yellow-400::-ms-input-placeholder {
    color: #f6e05e;
  }

  .lg\:placeholder-yellow-400::placeholder {
    color: #f6e05e;
  }

  .lg\:placeholder-yellow-500::-webkit-input-placeholder {
    color: #ecc94b;
  }

  .lg\:placeholder-yellow-500::-moz-placeholder {
    color: #ecc94b;
  }

  .lg\:placeholder-yellow-500:-ms-input-placeholder {
    color: #ecc94b;
  }

  .lg\:placeholder-yellow-500::-ms-input-placeholder {
    color: #ecc94b;
  }

  .lg\:placeholder-yellow-500::placeholder {
    color: #ecc94b;
  }

  .lg\:placeholder-yellow-600::-webkit-input-placeholder {
    color: #d69e2e;
  }

  .lg\:placeholder-yellow-600::-moz-placeholder {
    color: #d69e2e;
  }

  .lg\:placeholder-yellow-600:-ms-input-placeholder {
    color: #d69e2e;
  }

  .lg\:placeholder-yellow-600::-ms-input-placeholder {
    color: #d69e2e;
  }

  .lg\:placeholder-yellow-600::placeholder {
    color: #d69e2e;
  }

  .lg\:placeholder-yellow-700::-webkit-input-placeholder {
    color: #b7791f;
  }

  .lg\:placeholder-yellow-700::-moz-placeholder {
    color: #b7791f;
  }

  .lg\:placeholder-yellow-700:-ms-input-placeholder {
    color: #b7791f;
  }

  .lg\:placeholder-yellow-700::-ms-input-placeholder {
    color: #b7791f;
  }

  .lg\:placeholder-yellow-700::placeholder {
    color: #b7791f;
  }

  .lg\:placeholder-yellow-800::-webkit-input-placeholder {
    color: #975a16;
  }

  .lg\:placeholder-yellow-800::-moz-placeholder {
    color: #975a16;
  }

  .lg\:placeholder-yellow-800:-ms-input-placeholder {
    color: #975a16;
  }

  .lg\:placeholder-yellow-800::-ms-input-placeholder {
    color: #975a16;
  }

  .lg\:placeholder-yellow-800::placeholder {
    color: #975a16;
  }

  .lg\:placeholder-yellow-900::-webkit-input-placeholder {
    color: #744210;
  }

  .lg\:placeholder-yellow-900::-moz-placeholder {
    color: #744210;
  }

  .lg\:placeholder-yellow-900:-ms-input-placeholder {
    color: #744210;
  }

  .lg\:placeholder-yellow-900::-ms-input-placeholder {
    color: #744210;
  }

  .lg\:placeholder-yellow-900::placeholder {
    color: #744210;
  }

  .lg\:placeholder-green-100::-webkit-input-placeholder {
    color: #f0fff4;
  }

  .lg\:placeholder-green-100::-moz-placeholder {
    color: #f0fff4;
  }

  .lg\:placeholder-green-100:-ms-input-placeholder {
    color: #f0fff4;
  }

  .lg\:placeholder-green-100::-ms-input-placeholder {
    color: #f0fff4;
  }

  .lg\:placeholder-green-100::placeholder {
    color: #f0fff4;
  }

  .lg\:placeholder-green-200::-webkit-input-placeholder {
    color: #c6f6d5;
  }

  .lg\:placeholder-green-200::-moz-placeholder {
    color: #c6f6d5;
  }

  .lg\:placeholder-green-200:-ms-input-placeholder {
    color: #c6f6d5;
  }

  .lg\:placeholder-green-200::-ms-input-placeholder {
    color: #c6f6d5;
  }

  .lg\:placeholder-green-200::placeholder {
    color: #c6f6d5;
  }

  .lg\:placeholder-green-300::-webkit-input-placeholder {
    color: #9ae6b4;
  }

  .lg\:placeholder-green-300::-moz-placeholder {
    color: #9ae6b4;
  }

  .lg\:placeholder-green-300:-ms-input-placeholder {
    color: #9ae6b4;
  }

  .lg\:placeholder-green-300::-ms-input-placeholder {
    color: #9ae6b4;
  }

  .lg\:placeholder-green-300::placeholder {
    color: #9ae6b4;
  }

  .lg\:placeholder-green-400::-webkit-input-placeholder {
    color: #68d391;
  }

  .lg\:placeholder-green-400::-moz-placeholder {
    color: #68d391;
  }

  .lg\:placeholder-green-400:-ms-input-placeholder {
    color: #68d391;
  }

  .lg\:placeholder-green-400::-ms-input-placeholder {
    color: #68d391;
  }

  .lg\:placeholder-green-400::placeholder {
    color: #68d391;
  }

  .lg\:placeholder-green-500::-webkit-input-placeholder {
    color: #48bb78;
  }

  .lg\:placeholder-green-500::-moz-placeholder {
    color: #48bb78;
  }

  .lg\:placeholder-green-500:-ms-input-placeholder {
    color: #48bb78;
  }

  .lg\:placeholder-green-500::-ms-input-placeholder {
    color: #48bb78;
  }

  .lg\:placeholder-green-500::placeholder {
    color: #48bb78;
  }

  .lg\:placeholder-green-600::-webkit-input-placeholder {
    color: #38a169;
  }

  .lg\:placeholder-green-600::-moz-placeholder {
    color: #38a169;
  }

  .lg\:placeholder-green-600:-ms-input-placeholder {
    color: #38a169;
  }

  .lg\:placeholder-green-600::-ms-input-placeholder {
    color: #38a169;
  }

  .lg\:placeholder-green-600::placeholder {
    color: #38a169;
  }

  .lg\:placeholder-green-700::-webkit-input-placeholder {
    color: #2f855a;
  }

  .lg\:placeholder-green-700::-moz-placeholder {
    color: #2f855a;
  }

  .lg\:placeholder-green-700:-ms-input-placeholder {
    color: #2f855a;
  }

  .lg\:placeholder-green-700::-ms-input-placeholder {
    color: #2f855a;
  }

  .lg\:placeholder-green-700::placeholder {
    color: #2f855a;
  }

  .lg\:placeholder-green-800::-webkit-input-placeholder {
    color: #276749;
  }

  .lg\:placeholder-green-800::-moz-placeholder {
    color: #276749;
  }

  .lg\:placeholder-green-800:-ms-input-placeholder {
    color: #276749;
  }

  .lg\:placeholder-green-800::-ms-input-placeholder {
    color: #276749;
  }

  .lg\:placeholder-green-800::placeholder {
    color: #276749;
  }

  .lg\:placeholder-green-900::-webkit-input-placeholder {
    color: #22543d;
  }

  .lg\:placeholder-green-900::-moz-placeholder {
    color: #22543d;
  }

  .lg\:placeholder-green-900:-ms-input-placeholder {
    color: #22543d;
  }

  .lg\:placeholder-green-900::-ms-input-placeholder {
    color: #22543d;
  }

  .lg\:placeholder-green-900::placeholder {
    color: #22543d;
  }

  .lg\:placeholder-teal-100::-webkit-input-placeholder {
    color: #e6fffa;
  }

  .lg\:placeholder-teal-100::-moz-placeholder {
    color: #e6fffa;
  }

  .lg\:placeholder-teal-100:-ms-input-placeholder {
    color: #e6fffa;
  }

  .lg\:placeholder-teal-100::-ms-input-placeholder {
    color: #e6fffa;
  }

  .lg\:placeholder-teal-100::placeholder {
    color: #e6fffa;
  }

  .lg\:placeholder-teal-200::-webkit-input-placeholder {
    color: #b2f5ea;
  }

  .lg\:placeholder-teal-200::-moz-placeholder {
    color: #b2f5ea;
  }

  .lg\:placeholder-teal-200:-ms-input-placeholder {
    color: #b2f5ea;
  }

  .lg\:placeholder-teal-200::-ms-input-placeholder {
    color: #b2f5ea;
  }

  .lg\:placeholder-teal-200::placeholder {
    color: #b2f5ea;
  }

  .lg\:placeholder-teal-300::-webkit-input-placeholder {
    color: #81e6d9;
  }

  .lg\:placeholder-teal-300::-moz-placeholder {
    color: #81e6d9;
  }

  .lg\:placeholder-teal-300:-ms-input-placeholder {
    color: #81e6d9;
  }

  .lg\:placeholder-teal-300::-ms-input-placeholder {
    color: #81e6d9;
  }

  .lg\:placeholder-teal-300::placeholder {
    color: #81e6d9;
  }

  .lg\:placeholder-teal-400::-webkit-input-placeholder {
    color: #4fd1c5;
  }

  .lg\:placeholder-teal-400::-moz-placeholder {
    color: #4fd1c5;
  }

  .lg\:placeholder-teal-400:-ms-input-placeholder {
    color: #4fd1c5;
  }

  .lg\:placeholder-teal-400::-ms-input-placeholder {
    color: #4fd1c5;
  }

  .lg\:placeholder-teal-400::placeholder {
    color: #4fd1c5;
  }

  .lg\:placeholder-teal-500::-webkit-input-placeholder {
    color: #38b2ac;
  }

  .lg\:placeholder-teal-500::-moz-placeholder {
    color: #38b2ac;
  }

  .lg\:placeholder-teal-500:-ms-input-placeholder {
    color: #38b2ac;
  }

  .lg\:placeholder-teal-500::-ms-input-placeholder {
    color: #38b2ac;
  }

  .lg\:placeholder-teal-500::placeholder {
    color: #38b2ac;
  }

  .lg\:placeholder-teal-600::-webkit-input-placeholder {
    color: #319795;
  }

  .lg\:placeholder-teal-600::-moz-placeholder {
    color: #319795;
  }

  .lg\:placeholder-teal-600:-ms-input-placeholder {
    color: #319795;
  }

  .lg\:placeholder-teal-600::-ms-input-placeholder {
    color: #319795;
  }

  .lg\:placeholder-teal-600::placeholder {
    color: #319795;
  }

  .lg\:placeholder-teal-700::-webkit-input-placeholder {
    color: #2c7a7b;
  }

  .lg\:placeholder-teal-700::-moz-placeholder {
    color: #2c7a7b;
  }

  .lg\:placeholder-teal-700:-ms-input-placeholder {
    color: #2c7a7b;
  }

  .lg\:placeholder-teal-700::-ms-input-placeholder {
    color: #2c7a7b;
  }

  .lg\:placeholder-teal-700::placeholder {
    color: #2c7a7b;
  }

  .lg\:placeholder-teal-800::-webkit-input-placeholder {
    color: #285e61;
  }

  .lg\:placeholder-teal-800::-moz-placeholder {
    color: #285e61;
  }

  .lg\:placeholder-teal-800:-ms-input-placeholder {
    color: #285e61;
  }

  .lg\:placeholder-teal-800::-ms-input-placeholder {
    color: #285e61;
  }

  .lg\:placeholder-teal-800::placeholder {
    color: #285e61;
  }

  .lg\:placeholder-teal-900::-webkit-input-placeholder {
    color: #234e52;
  }

  .lg\:placeholder-teal-900::-moz-placeholder {
    color: #234e52;
  }

  .lg\:placeholder-teal-900:-ms-input-placeholder {
    color: #234e52;
  }

  .lg\:placeholder-teal-900::-ms-input-placeholder {
    color: #234e52;
  }

  .lg\:placeholder-teal-900::placeholder {
    color: #234e52;
  }

  .lg\:placeholder-blue-100::-webkit-input-placeholder {
    color: #ebf8ff;
  }

  .lg\:placeholder-blue-100::-moz-placeholder {
    color: #ebf8ff;
  }

  .lg\:placeholder-blue-100:-ms-input-placeholder {
    color: #ebf8ff;
  }

  .lg\:placeholder-blue-100::-ms-input-placeholder {
    color: #ebf8ff;
  }

  .lg\:placeholder-blue-100::placeholder {
    color: #ebf8ff;
  }

  .lg\:placeholder-blue-200::-webkit-input-placeholder {
    color: #bee3f8;
  }

  .lg\:placeholder-blue-200::-moz-placeholder {
    color: #bee3f8;
  }

  .lg\:placeholder-blue-200:-ms-input-placeholder {
    color: #bee3f8;
  }

  .lg\:placeholder-blue-200::-ms-input-placeholder {
    color: #bee3f8;
  }

  .lg\:placeholder-blue-200::placeholder {
    color: #bee3f8;
  }

  .lg\:placeholder-blue-300::-webkit-input-placeholder {
    color: #90cdf4;
  }

  .lg\:placeholder-blue-300::-moz-placeholder {
    color: #90cdf4;
  }

  .lg\:placeholder-blue-300:-ms-input-placeholder {
    color: #90cdf4;
  }

  .lg\:placeholder-blue-300::-ms-input-placeholder {
    color: #90cdf4;
  }

  .lg\:placeholder-blue-300::placeholder {
    color: #90cdf4;
  }

  .lg\:placeholder-blue-400::-webkit-input-placeholder {
    color: #63b3ed;
  }

  .lg\:placeholder-blue-400::-moz-placeholder {
    color: #63b3ed;
  }

  .lg\:placeholder-blue-400:-ms-input-placeholder {
    color: #63b3ed;
  }

  .lg\:placeholder-blue-400::-ms-input-placeholder {
    color: #63b3ed;
  }

  .lg\:placeholder-blue-400::placeholder {
    color: #63b3ed;
  }

  .lg\:placeholder-blue-500::-webkit-input-placeholder {
    color: #4299e1;
  }

  .lg\:placeholder-blue-500::-moz-placeholder {
    color: #4299e1;
  }

  .lg\:placeholder-blue-500:-ms-input-placeholder {
    color: #4299e1;
  }

  .lg\:placeholder-blue-500::-ms-input-placeholder {
    color: #4299e1;
  }

  .lg\:placeholder-blue-500::placeholder {
    color: #4299e1;
  }

  .lg\:placeholder-blue-600::-webkit-input-placeholder {
    color: #3182ce;
  }

  .lg\:placeholder-blue-600::-moz-placeholder {
    color: #3182ce;
  }

  .lg\:placeholder-blue-600:-ms-input-placeholder {
    color: #3182ce;
  }

  .lg\:placeholder-blue-600::-ms-input-placeholder {
    color: #3182ce;
  }

  .lg\:placeholder-blue-600::placeholder {
    color: #3182ce;
  }

  .lg\:placeholder-blue-700::-webkit-input-placeholder {
    color: #2b6cb0;
  }

  .lg\:placeholder-blue-700::-moz-placeholder {
    color: #2b6cb0;
  }

  .lg\:placeholder-blue-700:-ms-input-placeholder {
    color: #2b6cb0;
  }

  .lg\:placeholder-blue-700::-ms-input-placeholder {
    color: #2b6cb0;
  }

  .lg\:placeholder-blue-700::placeholder {
    color: #2b6cb0;
  }

  .lg\:placeholder-blue-800::-webkit-input-placeholder {
    color: #2c5282;
  }

  .lg\:placeholder-blue-800::-moz-placeholder {
    color: #2c5282;
  }

  .lg\:placeholder-blue-800:-ms-input-placeholder {
    color: #2c5282;
  }

  .lg\:placeholder-blue-800::-ms-input-placeholder {
    color: #2c5282;
  }

  .lg\:placeholder-blue-800::placeholder {
    color: #2c5282;
  }

  .lg\:placeholder-blue-900::-webkit-input-placeholder {
    color: #2a4365;
  }

  .lg\:placeholder-blue-900::-moz-placeholder {
    color: #2a4365;
  }

  .lg\:placeholder-blue-900:-ms-input-placeholder {
    color: #2a4365;
  }

  .lg\:placeholder-blue-900::-ms-input-placeholder {
    color: #2a4365;
  }

  .lg\:placeholder-blue-900::placeholder {
    color: #2a4365;
  }

  .lg\:placeholder-indigo-100::-webkit-input-placeholder {
    color: #ebf4ff;
  }

  .lg\:placeholder-indigo-100::-moz-placeholder {
    color: #ebf4ff;
  }

  .lg\:placeholder-indigo-100:-ms-input-placeholder {
    color: #ebf4ff;
  }

  .lg\:placeholder-indigo-100::-ms-input-placeholder {
    color: #ebf4ff;
  }

  .lg\:placeholder-indigo-100::placeholder {
    color: #ebf4ff;
  }

  .lg\:placeholder-indigo-200::-webkit-input-placeholder {
    color: #c3dafe;
  }

  .lg\:placeholder-indigo-200::-moz-placeholder {
    color: #c3dafe;
  }

  .lg\:placeholder-indigo-200:-ms-input-placeholder {
    color: #c3dafe;
  }

  .lg\:placeholder-indigo-200::-ms-input-placeholder {
    color: #c3dafe;
  }

  .lg\:placeholder-indigo-200::placeholder {
    color: #c3dafe;
  }

  .lg\:placeholder-indigo-300::-webkit-input-placeholder {
    color: #a3bffa;
  }

  .lg\:placeholder-indigo-300::-moz-placeholder {
    color: #a3bffa;
  }

  .lg\:placeholder-indigo-300:-ms-input-placeholder {
    color: #a3bffa;
  }

  .lg\:placeholder-indigo-300::-ms-input-placeholder {
    color: #a3bffa;
  }

  .lg\:placeholder-indigo-300::placeholder {
    color: #a3bffa;
  }

  .lg\:placeholder-indigo-400::-webkit-input-placeholder {
    color: #7f9cf5;
  }

  .lg\:placeholder-indigo-400::-moz-placeholder {
    color: #7f9cf5;
  }

  .lg\:placeholder-indigo-400:-ms-input-placeholder {
    color: #7f9cf5;
  }

  .lg\:placeholder-indigo-400::-ms-input-placeholder {
    color: #7f9cf5;
  }

  .lg\:placeholder-indigo-400::placeholder {
    color: #7f9cf5;
  }

  .lg\:placeholder-indigo-500::-webkit-input-placeholder {
    color: #667eea;
  }

  .lg\:placeholder-indigo-500::-moz-placeholder {
    color: #667eea;
  }

  .lg\:placeholder-indigo-500:-ms-input-placeholder {
    color: #667eea;
  }

  .lg\:placeholder-indigo-500::-ms-input-placeholder {
    color: #667eea;
  }

  .lg\:placeholder-indigo-500::placeholder {
    color: #667eea;
  }

  .lg\:placeholder-indigo-600::-webkit-input-placeholder {
    color: #5a67d8;
  }

  .lg\:placeholder-indigo-600::-moz-placeholder {
    color: #5a67d8;
  }

  .lg\:placeholder-indigo-600:-ms-input-placeholder {
    color: #5a67d8;
  }

  .lg\:placeholder-indigo-600::-ms-input-placeholder {
    color: #5a67d8;
  }

  .lg\:placeholder-indigo-600::placeholder {
    color: #5a67d8;
  }

  .lg\:placeholder-indigo-700::-webkit-input-placeholder {
    color: #4c51bf;
  }

  .lg\:placeholder-indigo-700::-moz-placeholder {
    color: #4c51bf;
  }

  .lg\:placeholder-indigo-700:-ms-input-placeholder {
    color: #4c51bf;
  }

  .lg\:placeholder-indigo-700::-ms-input-placeholder {
    color: #4c51bf;
  }

  .lg\:placeholder-indigo-700::placeholder {
    color: #4c51bf;
  }

  .lg\:placeholder-indigo-800::-webkit-input-placeholder {
    color: #434190;
  }

  .lg\:placeholder-indigo-800::-moz-placeholder {
    color: #434190;
  }

  .lg\:placeholder-indigo-800:-ms-input-placeholder {
    color: #434190;
  }

  .lg\:placeholder-indigo-800::-ms-input-placeholder {
    color: #434190;
  }

  .lg\:placeholder-indigo-800::placeholder {
    color: #434190;
  }

  .lg\:placeholder-indigo-900::-webkit-input-placeholder {
    color: #3c366b;
  }

  .lg\:placeholder-indigo-900::-moz-placeholder {
    color: #3c366b;
  }

  .lg\:placeholder-indigo-900:-ms-input-placeholder {
    color: #3c366b;
  }

  .lg\:placeholder-indigo-900::-ms-input-placeholder {
    color: #3c366b;
  }

  .lg\:placeholder-indigo-900::placeholder {
    color: #3c366b;
  }

  .lg\:placeholder-purple-100::-webkit-input-placeholder {
    color: #faf5ff;
  }

  .lg\:placeholder-purple-100::-moz-placeholder {
    color: #faf5ff;
  }

  .lg\:placeholder-purple-100:-ms-input-placeholder {
    color: #faf5ff;
  }

  .lg\:placeholder-purple-100::-ms-input-placeholder {
    color: #faf5ff;
  }

  .lg\:placeholder-purple-100::placeholder {
    color: #faf5ff;
  }

  .lg\:placeholder-purple-200::-webkit-input-placeholder {
    color: #e9d8fd;
  }

  .lg\:placeholder-purple-200::-moz-placeholder {
    color: #e9d8fd;
  }

  .lg\:placeholder-purple-200:-ms-input-placeholder {
    color: #e9d8fd;
  }

  .lg\:placeholder-purple-200::-ms-input-placeholder {
    color: #e9d8fd;
  }

  .lg\:placeholder-purple-200::placeholder {
    color: #e9d8fd;
  }

  .lg\:placeholder-purple-300::-webkit-input-placeholder {
    color: #d6bcfa;
  }

  .lg\:placeholder-purple-300::-moz-placeholder {
    color: #d6bcfa;
  }

  .lg\:placeholder-purple-300:-ms-input-placeholder {
    color: #d6bcfa;
  }

  .lg\:placeholder-purple-300::-ms-input-placeholder {
    color: #d6bcfa;
  }

  .lg\:placeholder-purple-300::placeholder {
    color: #d6bcfa;
  }

  .lg\:placeholder-purple-400::-webkit-input-placeholder {
    color: #b794f4;
  }

  .lg\:placeholder-purple-400::-moz-placeholder {
    color: #b794f4;
  }

  .lg\:placeholder-purple-400:-ms-input-placeholder {
    color: #b794f4;
  }

  .lg\:placeholder-purple-400::-ms-input-placeholder {
    color: #b794f4;
  }

  .lg\:placeholder-purple-400::placeholder {
    color: #b794f4;
  }

  .lg\:placeholder-purple-500::-webkit-input-placeholder {
    color: #9f7aea;
  }

  .lg\:placeholder-purple-500::-moz-placeholder {
    color: #9f7aea;
  }

  .lg\:placeholder-purple-500:-ms-input-placeholder {
    color: #9f7aea;
  }

  .lg\:placeholder-purple-500::-ms-input-placeholder {
    color: #9f7aea;
  }

  .lg\:placeholder-purple-500::placeholder {
    color: #9f7aea;
  }

  .lg\:placeholder-purple-600::-webkit-input-placeholder {
    color: #805ad5;
  }

  .lg\:placeholder-purple-600::-moz-placeholder {
    color: #805ad5;
  }

  .lg\:placeholder-purple-600:-ms-input-placeholder {
    color: #805ad5;
  }

  .lg\:placeholder-purple-600::-ms-input-placeholder {
    color: #805ad5;
  }

  .lg\:placeholder-purple-600::placeholder {
    color: #805ad5;
  }

  .lg\:placeholder-purple-700::-webkit-input-placeholder {
    color: #6b46c1;
  }

  .lg\:placeholder-purple-700::-moz-placeholder {
    color: #6b46c1;
  }

  .lg\:placeholder-purple-700:-ms-input-placeholder {
    color: #6b46c1;
  }

  .lg\:placeholder-purple-700::-ms-input-placeholder {
    color: #6b46c1;
  }

  .lg\:placeholder-purple-700::placeholder {
    color: #6b46c1;
  }

  .lg\:placeholder-purple-800::-webkit-input-placeholder {
    color: #553c9a;
  }

  .lg\:placeholder-purple-800::-moz-placeholder {
    color: #553c9a;
  }

  .lg\:placeholder-purple-800:-ms-input-placeholder {
    color: #553c9a;
  }

  .lg\:placeholder-purple-800::-ms-input-placeholder {
    color: #553c9a;
  }

  .lg\:placeholder-purple-800::placeholder {
    color: #553c9a;
  }

  .lg\:placeholder-purple-900::-webkit-input-placeholder {
    color: #44337a;
  }

  .lg\:placeholder-purple-900::-moz-placeholder {
    color: #44337a;
  }

  .lg\:placeholder-purple-900:-ms-input-placeholder {
    color: #44337a;
  }

  .lg\:placeholder-purple-900::-ms-input-placeholder {
    color: #44337a;
  }

  .lg\:placeholder-purple-900::placeholder {
    color: #44337a;
  }

  .lg\:placeholder-pink-100::-webkit-input-placeholder {
    color: #fff5f7;
  }

  .lg\:placeholder-pink-100::-moz-placeholder {
    color: #fff5f7;
  }

  .lg\:placeholder-pink-100:-ms-input-placeholder {
    color: #fff5f7;
  }

  .lg\:placeholder-pink-100::-ms-input-placeholder {
    color: #fff5f7;
  }

  .lg\:placeholder-pink-100::placeholder {
    color: #fff5f7;
  }

  .lg\:placeholder-pink-200::-webkit-input-placeholder {
    color: #fed7e2;
  }

  .lg\:placeholder-pink-200::-moz-placeholder {
    color: #fed7e2;
  }

  .lg\:placeholder-pink-200:-ms-input-placeholder {
    color: #fed7e2;
  }

  .lg\:placeholder-pink-200::-ms-input-placeholder {
    color: #fed7e2;
  }

  .lg\:placeholder-pink-200::placeholder {
    color: #fed7e2;
  }

  .lg\:placeholder-pink-300::-webkit-input-placeholder {
    color: #fbb6ce;
  }

  .lg\:placeholder-pink-300::-moz-placeholder {
    color: #fbb6ce;
  }

  .lg\:placeholder-pink-300:-ms-input-placeholder {
    color: #fbb6ce;
  }

  .lg\:placeholder-pink-300::-ms-input-placeholder {
    color: #fbb6ce;
  }

  .lg\:placeholder-pink-300::placeholder {
    color: #fbb6ce;
  }

  .lg\:placeholder-pink-400::-webkit-input-placeholder {
    color: #f687b3;
  }

  .lg\:placeholder-pink-400::-moz-placeholder {
    color: #f687b3;
  }

  .lg\:placeholder-pink-400:-ms-input-placeholder {
    color: #f687b3;
  }

  .lg\:placeholder-pink-400::-ms-input-placeholder {
    color: #f687b3;
  }

  .lg\:placeholder-pink-400::placeholder {
    color: #f687b3;
  }

  .lg\:placeholder-pink-500::-webkit-input-placeholder {
    color: #ed64a6;
  }

  .lg\:placeholder-pink-500::-moz-placeholder {
    color: #ed64a6;
  }

  .lg\:placeholder-pink-500:-ms-input-placeholder {
    color: #ed64a6;
  }

  .lg\:placeholder-pink-500::-ms-input-placeholder {
    color: #ed64a6;
  }

  .lg\:placeholder-pink-500::placeholder {
    color: #ed64a6;
  }

  .lg\:placeholder-pink-600::-webkit-input-placeholder {
    color: #d53f8c;
  }

  .lg\:placeholder-pink-600::-moz-placeholder {
    color: #d53f8c;
  }

  .lg\:placeholder-pink-600:-ms-input-placeholder {
    color: #d53f8c;
  }

  .lg\:placeholder-pink-600::-ms-input-placeholder {
    color: #d53f8c;
  }

  .lg\:placeholder-pink-600::placeholder {
    color: #d53f8c;
  }

  .lg\:placeholder-pink-700::-webkit-input-placeholder {
    color: #b83280;
  }

  .lg\:placeholder-pink-700::-moz-placeholder {
    color: #b83280;
  }

  .lg\:placeholder-pink-700:-ms-input-placeholder {
    color: #b83280;
  }

  .lg\:placeholder-pink-700::-ms-input-placeholder {
    color: #b83280;
  }

  .lg\:placeholder-pink-700::placeholder {
    color: #b83280;
  }

  .lg\:placeholder-pink-800::-webkit-input-placeholder {
    color: #97266d;
  }

  .lg\:placeholder-pink-800::-moz-placeholder {
    color: #97266d;
  }

  .lg\:placeholder-pink-800:-ms-input-placeholder {
    color: #97266d;
  }

  .lg\:placeholder-pink-800::-ms-input-placeholder {
    color: #97266d;
  }

  .lg\:placeholder-pink-800::placeholder {
    color: #97266d;
  }

  .lg\:placeholder-pink-900::-webkit-input-placeholder {
    color: #702459;
  }

  .lg\:placeholder-pink-900::-moz-placeholder {
    color: #702459;
  }

  .lg\:placeholder-pink-900:-ms-input-placeholder {
    color: #702459;
  }

  .lg\:placeholder-pink-900::-ms-input-placeholder {
    color: #702459;
  }

  .lg\:placeholder-pink-900::placeholder {
    color: #702459;
  }

  .lg\:focus\:placeholder-transparent:focus::-webkit-input-placeholder {
    color: transparent;
  }

  .lg\:focus\:placeholder-transparent:focus::-moz-placeholder {
    color: transparent;
  }

  .lg\:focus\:placeholder-transparent:focus:-ms-input-placeholder {
    color: transparent;
  }

  .lg\:focus\:placeholder-transparent:focus::-ms-input-placeholder {
    color: transparent;
  }

  .lg\:focus\:placeholder-transparent:focus::placeholder {
    color: transparent;
  }

  .lg\:focus\:placeholder-black:focus::-webkit-input-placeholder {
    color: #000;
  }

  .lg\:focus\:placeholder-black:focus::-moz-placeholder {
    color: #000;
  }

  .lg\:focus\:placeholder-black:focus:-ms-input-placeholder {
    color: #000;
  }

  .lg\:focus\:placeholder-black:focus::-ms-input-placeholder {
    color: #000;
  }

  .lg\:focus\:placeholder-black:focus::placeholder {
    color: #000;
  }

  .lg\:focus\:placeholder-white:focus::-webkit-input-placeholder {
    color: #fff;
  }

  .lg\:focus\:placeholder-white:focus::-moz-placeholder {
    color: #fff;
  }

  .lg\:focus\:placeholder-white:focus:-ms-input-placeholder {
    color: #fff;
  }

  .lg\:focus\:placeholder-white:focus::-ms-input-placeholder {
    color: #fff;
  }

  .lg\:focus\:placeholder-white:focus::placeholder {
    color: #fff;
  }

  .lg\:focus\:placeholder-gray-100:focus::-webkit-input-placeholder {
    color: #f7fafc;
  }

  .lg\:focus\:placeholder-gray-100:focus::-moz-placeholder {
    color: #f7fafc;
  }

  .lg\:focus\:placeholder-gray-100:focus:-ms-input-placeholder {
    color: #f7fafc;
  }

  .lg\:focus\:placeholder-gray-100:focus::-ms-input-placeholder {
    color: #f7fafc;
  }

  .lg\:focus\:placeholder-gray-100:focus::placeholder {
    color: #f7fafc;
  }

  .lg\:focus\:placeholder-gray-200:focus::-webkit-input-placeholder {
    color: #edf2f7;
  }

  .lg\:focus\:placeholder-gray-200:focus::-moz-placeholder {
    color: #edf2f7;
  }

  .lg\:focus\:placeholder-gray-200:focus:-ms-input-placeholder {
    color: #edf2f7;
  }

  .lg\:focus\:placeholder-gray-200:focus::-ms-input-placeholder {
    color: #edf2f7;
  }

  .lg\:focus\:placeholder-gray-200:focus::placeholder {
    color: #edf2f7;
  }

  .lg\:focus\:placeholder-gray-300:focus::-webkit-input-placeholder {
    color: #e2e8f0;
  }

  .lg\:focus\:placeholder-gray-300:focus::-moz-placeholder {
    color: #e2e8f0;
  }

  .lg\:focus\:placeholder-gray-300:focus:-ms-input-placeholder {
    color: #e2e8f0;
  }

  .lg\:focus\:placeholder-gray-300:focus::-ms-input-placeholder {
    color: #e2e8f0;
  }

  .lg\:focus\:placeholder-gray-300:focus::placeholder {
    color: #e2e8f0;
  }

  .lg\:focus\:placeholder-gray-400:focus::-webkit-input-placeholder {
    color: #cbd5e0;
  }

  .lg\:focus\:placeholder-gray-400:focus::-moz-placeholder {
    color: #cbd5e0;
  }

  .lg\:focus\:placeholder-gray-400:focus:-ms-input-placeholder {
    color: #cbd5e0;
  }

  .lg\:focus\:placeholder-gray-400:focus::-ms-input-placeholder {
    color: #cbd5e0;
  }

  .lg\:focus\:placeholder-gray-400:focus::placeholder {
    color: #cbd5e0;
  }

  .lg\:focus\:placeholder-gray-500:focus::-webkit-input-placeholder {
    color: #a0aec0;
  }

  .lg\:focus\:placeholder-gray-500:focus::-moz-placeholder {
    color: #a0aec0;
  }

  .lg\:focus\:placeholder-gray-500:focus:-ms-input-placeholder {
    color: #a0aec0;
  }

  .lg\:focus\:placeholder-gray-500:focus::-ms-input-placeholder {
    color: #a0aec0;
  }

  .lg\:focus\:placeholder-gray-500:focus::placeholder {
    color: #a0aec0;
  }

  .lg\:focus\:placeholder-gray-600:focus::-webkit-input-placeholder {
    color: #718096;
  }

  .lg\:focus\:placeholder-gray-600:focus::-moz-placeholder {
    color: #718096;
  }

  .lg\:focus\:placeholder-gray-600:focus:-ms-input-placeholder {
    color: #718096;
  }

  .lg\:focus\:placeholder-gray-600:focus::-ms-input-placeholder {
    color: #718096;
  }

  .lg\:focus\:placeholder-gray-600:focus::placeholder {
    color: #718096;
  }

  .lg\:focus\:placeholder-gray-700:focus::-webkit-input-placeholder {
    color: #4a5568;
  }

  .lg\:focus\:placeholder-gray-700:focus::-moz-placeholder {
    color: #4a5568;
  }

  .lg\:focus\:placeholder-gray-700:focus:-ms-input-placeholder {
    color: #4a5568;
  }

  .lg\:focus\:placeholder-gray-700:focus::-ms-input-placeholder {
    color: #4a5568;
  }

  .lg\:focus\:placeholder-gray-700:focus::placeholder {
    color: #4a5568;
  }

  .lg\:focus\:placeholder-gray-800:focus::-webkit-input-placeholder {
    color: #2d3748;
  }

  .lg\:focus\:placeholder-gray-800:focus::-moz-placeholder {
    color: #2d3748;
  }

  .lg\:focus\:placeholder-gray-800:focus:-ms-input-placeholder {
    color: #2d3748;
  }

  .lg\:focus\:placeholder-gray-800:focus::-ms-input-placeholder {
    color: #2d3748;
  }

  .lg\:focus\:placeholder-gray-800:focus::placeholder {
    color: #2d3748;
  }

  .lg\:focus\:placeholder-gray-900:focus::-webkit-input-placeholder {
    color: #1a202c;
  }

  .lg\:focus\:placeholder-gray-900:focus::-moz-placeholder {
    color: #1a202c;
  }

  .lg\:focus\:placeholder-gray-900:focus:-ms-input-placeholder {
    color: #1a202c;
  }

  .lg\:focus\:placeholder-gray-900:focus::-ms-input-placeholder {
    color: #1a202c;
  }

  .lg\:focus\:placeholder-gray-900:focus::placeholder {
    color: #1a202c;
  }

  .lg\:focus\:placeholder-red-100:focus::-webkit-input-placeholder {
    color: #fff5f5;
  }

  .lg\:focus\:placeholder-red-100:focus::-moz-placeholder {
    color: #fff5f5;
  }

  .lg\:focus\:placeholder-red-100:focus:-ms-input-placeholder {
    color: #fff5f5;
  }

  .lg\:focus\:placeholder-red-100:focus::-ms-input-placeholder {
    color: #fff5f5;
  }

  .lg\:focus\:placeholder-red-100:focus::placeholder {
    color: #fff5f5;
  }

  .lg\:focus\:placeholder-red-200:focus::-webkit-input-placeholder {
    color: #fed7d7;
  }

  .lg\:focus\:placeholder-red-200:focus::-moz-placeholder {
    color: #fed7d7;
  }

  .lg\:focus\:placeholder-red-200:focus:-ms-input-placeholder {
    color: #fed7d7;
  }

  .lg\:focus\:placeholder-red-200:focus::-ms-input-placeholder {
    color: #fed7d7;
  }

  .lg\:focus\:placeholder-red-200:focus::placeholder {
    color: #fed7d7;
  }

  .lg\:focus\:placeholder-red-300:focus::-webkit-input-placeholder {
    color: #feb2b2;
  }

  .lg\:focus\:placeholder-red-300:focus::-moz-placeholder {
    color: #feb2b2;
  }

  .lg\:focus\:placeholder-red-300:focus:-ms-input-placeholder {
    color: #feb2b2;
  }

  .lg\:focus\:placeholder-red-300:focus::-ms-input-placeholder {
    color: #feb2b2;
  }

  .lg\:focus\:placeholder-red-300:focus::placeholder {
    color: #feb2b2;
  }

  .lg\:focus\:placeholder-red-400:focus::-webkit-input-placeholder {
    color: #fc8181;
  }

  .lg\:focus\:placeholder-red-400:focus::-moz-placeholder {
    color: #fc8181;
  }

  .lg\:focus\:placeholder-red-400:focus:-ms-input-placeholder {
    color: #fc8181;
  }

  .lg\:focus\:placeholder-red-400:focus::-ms-input-placeholder {
    color: #fc8181;
  }

  .lg\:focus\:placeholder-red-400:focus::placeholder {
    color: #fc8181;
  }

  .lg\:focus\:placeholder-red-500:focus::-webkit-input-placeholder {
    color: #f56565;
  }

  .lg\:focus\:placeholder-red-500:focus::-moz-placeholder {
    color: #f56565;
  }

  .lg\:focus\:placeholder-red-500:focus:-ms-input-placeholder {
    color: #f56565;
  }

  .lg\:focus\:placeholder-red-500:focus::-ms-input-placeholder {
    color: #f56565;
  }

  .lg\:focus\:placeholder-red-500:focus::placeholder {
    color: #f56565;
  }

  .lg\:focus\:placeholder-red-600:focus::-webkit-input-placeholder {
    color: #e53e3e;
  }

  .lg\:focus\:placeholder-red-600:focus::-moz-placeholder {
    color: #e53e3e;
  }

  .lg\:focus\:placeholder-red-600:focus:-ms-input-placeholder {
    color: #e53e3e;
  }

  .lg\:focus\:placeholder-red-600:focus::-ms-input-placeholder {
    color: #e53e3e;
  }

  .lg\:focus\:placeholder-red-600:focus::placeholder {
    color: #e53e3e;
  }

  .lg\:focus\:placeholder-red-700:focus::-webkit-input-placeholder {
    color: #c53030;
  }

  .lg\:focus\:placeholder-red-700:focus::-moz-placeholder {
    color: #c53030;
  }

  .lg\:focus\:placeholder-red-700:focus:-ms-input-placeholder {
    color: #c53030;
  }

  .lg\:focus\:placeholder-red-700:focus::-ms-input-placeholder {
    color: #c53030;
  }

  .lg\:focus\:placeholder-red-700:focus::placeholder {
    color: #c53030;
  }

  .lg\:focus\:placeholder-red-800:focus::-webkit-input-placeholder {
    color: #9b2c2c;
  }

  .lg\:focus\:placeholder-red-800:focus::-moz-placeholder {
    color: #9b2c2c;
  }

  .lg\:focus\:placeholder-red-800:focus:-ms-input-placeholder {
    color: #9b2c2c;
  }

  .lg\:focus\:placeholder-red-800:focus::-ms-input-placeholder {
    color: #9b2c2c;
  }

  .lg\:focus\:placeholder-red-800:focus::placeholder {
    color: #9b2c2c;
  }

  .lg\:focus\:placeholder-red-900:focus::-webkit-input-placeholder {
    color: #742a2a;
  }

  .lg\:focus\:placeholder-red-900:focus::-moz-placeholder {
    color: #742a2a;
  }

  .lg\:focus\:placeholder-red-900:focus:-ms-input-placeholder {
    color: #742a2a;
  }

  .lg\:focus\:placeholder-red-900:focus::-ms-input-placeholder {
    color: #742a2a;
  }

  .lg\:focus\:placeholder-red-900:focus::placeholder {
    color: #742a2a;
  }

  .lg\:focus\:placeholder-orange-100:focus::-webkit-input-placeholder {
    color: #fffaf0;
  }

  .lg\:focus\:placeholder-orange-100:focus::-moz-placeholder {
    color: #fffaf0;
  }

  .lg\:focus\:placeholder-orange-100:focus:-ms-input-placeholder {
    color: #fffaf0;
  }

  .lg\:focus\:placeholder-orange-100:focus::-ms-input-placeholder {
    color: #fffaf0;
  }

  .lg\:focus\:placeholder-orange-100:focus::placeholder {
    color: #fffaf0;
  }

  .lg\:focus\:placeholder-orange-200:focus::-webkit-input-placeholder {
    color: #feebc8;
  }

  .lg\:focus\:placeholder-orange-200:focus::-moz-placeholder {
    color: #feebc8;
  }

  .lg\:focus\:placeholder-orange-200:focus:-ms-input-placeholder {
    color: #feebc8;
  }

  .lg\:focus\:placeholder-orange-200:focus::-ms-input-placeholder {
    color: #feebc8;
  }

  .lg\:focus\:placeholder-orange-200:focus::placeholder {
    color: #feebc8;
  }

  .lg\:focus\:placeholder-orange-300:focus::-webkit-input-placeholder {
    color: #fbd38d;
  }

  .lg\:focus\:placeholder-orange-300:focus::-moz-placeholder {
    color: #fbd38d;
  }

  .lg\:focus\:placeholder-orange-300:focus:-ms-input-placeholder {
    color: #fbd38d;
  }

  .lg\:focus\:placeholder-orange-300:focus::-ms-input-placeholder {
    color: #fbd38d;
  }

  .lg\:focus\:placeholder-orange-300:focus::placeholder {
    color: #fbd38d;
  }

  .lg\:focus\:placeholder-orange-400:focus::-webkit-input-placeholder {
    color: #f6ad55;
  }

  .lg\:focus\:placeholder-orange-400:focus::-moz-placeholder {
    color: #f6ad55;
  }

  .lg\:focus\:placeholder-orange-400:focus:-ms-input-placeholder {
    color: #f6ad55;
  }

  .lg\:focus\:placeholder-orange-400:focus::-ms-input-placeholder {
    color: #f6ad55;
  }

  .lg\:focus\:placeholder-orange-400:focus::placeholder {
    color: #f6ad55;
  }

  .lg\:focus\:placeholder-orange-500:focus::-webkit-input-placeholder {
    color: #ed8936;
  }

  .lg\:focus\:placeholder-orange-500:focus::-moz-placeholder {
    color: #ed8936;
  }

  .lg\:focus\:placeholder-orange-500:focus:-ms-input-placeholder {
    color: #ed8936;
  }

  .lg\:focus\:placeholder-orange-500:focus::-ms-input-placeholder {
    color: #ed8936;
  }

  .lg\:focus\:placeholder-orange-500:focus::placeholder {
    color: #ed8936;
  }

  .lg\:focus\:placeholder-orange-600:focus::-webkit-input-placeholder {
    color: #dd6b20;
  }

  .lg\:focus\:placeholder-orange-600:focus::-moz-placeholder {
    color: #dd6b20;
  }

  .lg\:focus\:placeholder-orange-600:focus:-ms-input-placeholder {
    color: #dd6b20;
  }

  .lg\:focus\:placeholder-orange-600:focus::-ms-input-placeholder {
    color: #dd6b20;
  }

  .lg\:focus\:placeholder-orange-600:focus::placeholder {
    color: #dd6b20;
  }

  .lg\:focus\:placeholder-orange-700:focus::-webkit-input-placeholder {
    color: #c05621;
  }

  .lg\:focus\:placeholder-orange-700:focus::-moz-placeholder {
    color: #c05621;
  }

  .lg\:focus\:placeholder-orange-700:focus:-ms-input-placeholder {
    color: #c05621;
  }

  .lg\:focus\:placeholder-orange-700:focus::-ms-input-placeholder {
    color: #c05621;
  }

  .lg\:focus\:placeholder-orange-700:focus::placeholder {
    color: #c05621;
  }

  .lg\:focus\:placeholder-orange-800:focus::-webkit-input-placeholder {
    color: #9c4221;
  }

  .lg\:focus\:placeholder-orange-800:focus::-moz-placeholder {
    color: #9c4221;
  }

  .lg\:focus\:placeholder-orange-800:focus:-ms-input-placeholder {
    color: #9c4221;
  }

  .lg\:focus\:placeholder-orange-800:focus::-ms-input-placeholder {
    color: #9c4221;
  }

  .lg\:focus\:placeholder-orange-800:focus::placeholder {
    color: #9c4221;
  }

  .lg\:focus\:placeholder-orange-900:focus::-webkit-input-placeholder {
    color: #7b341e;
  }

  .lg\:focus\:placeholder-orange-900:focus::-moz-placeholder {
    color: #7b341e;
  }

  .lg\:focus\:placeholder-orange-900:focus:-ms-input-placeholder {
    color: #7b341e;
  }

  .lg\:focus\:placeholder-orange-900:focus::-ms-input-placeholder {
    color: #7b341e;
  }

  .lg\:focus\:placeholder-orange-900:focus::placeholder {
    color: #7b341e;
  }

  .lg\:focus\:placeholder-yellow-100:focus::-webkit-input-placeholder {
    color: #fffff0;
  }

  .lg\:focus\:placeholder-yellow-100:focus::-moz-placeholder {
    color: #fffff0;
  }

  .lg\:focus\:placeholder-yellow-100:focus:-ms-input-placeholder {
    color: #fffff0;
  }

  .lg\:focus\:placeholder-yellow-100:focus::-ms-input-placeholder {
    color: #fffff0;
  }

  .lg\:focus\:placeholder-yellow-100:focus::placeholder {
    color: #fffff0;
  }

  .lg\:focus\:placeholder-yellow-200:focus::-webkit-input-placeholder {
    color: #fefcbf;
  }

  .lg\:focus\:placeholder-yellow-200:focus::-moz-placeholder {
    color: #fefcbf;
  }

  .lg\:focus\:placeholder-yellow-200:focus:-ms-input-placeholder {
    color: #fefcbf;
  }

  .lg\:focus\:placeholder-yellow-200:focus::-ms-input-placeholder {
    color: #fefcbf;
  }

  .lg\:focus\:placeholder-yellow-200:focus::placeholder {
    color: #fefcbf;
  }

  .lg\:focus\:placeholder-yellow-300:focus::-webkit-input-placeholder {
    color: #faf089;
  }

  .lg\:focus\:placeholder-yellow-300:focus::-moz-placeholder {
    color: #faf089;
  }

  .lg\:focus\:placeholder-yellow-300:focus:-ms-input-placeholder {
    color: #faf089;
  }

  .lg\:focus\:placeholder-yellow-300:focus::-ms-input-placeholder {
    color: #faf089;
  }

  .lg\:focus\:placeholder-yellow-300:focus::placeholder {
    color: #faf089;
  }

  .lg\:focus\:placeholder-yellow-400:focus::-webkit-input-placeholder {
    color: #f6e05e;
  }

  .lg\:focus\:placeholder-yellow-400:focus::-moz-placeholder {
    color: #f6e05e;
  }

  .lg\:focus\:placeholder-yellow-400:focus:-ms-input-placeholder {
    color: #f6e05e;
  }

  .lg\:focus\:placeholder-yellow-400:focus::-ms-input-placeholder {
    color: #f6e05e;
  }

  .lg\:focus\:placeholder-yellow-400:focus::placeholder {
    color: #f6e05e;
  }

  .lg\:focus\:placeholder-yellow-500:focus::-webkit-input-placeholder {
    color: #ecc94b;
  }

  .lg\:focus\:placeholder-yellow-500:focus::-moz-placeholder {
    color: #ecc94b;
  }

  .lg\:focus\:placeholder-yellow-500:focus:-ms-input-placeholder {
    color: #ecc94b;
  }

  .lg\:focus\:placeholder-yellow-500:focus::-ms-input-placeholder {
    color: #ecc94b;
  }

  .lg\:focus\:placeholder-yellow-500:focus::placeholder {
    color: #ecc94b;
  }

  .lg\:focus\:placeholder-yellow-600:focus::-webkit-input-placeholder {
    color: #d69e2e;
  }

  .lg\:focus\:placeholder-yellow-600:focus::-moz-placeholder {
    color: #d69e2e;
  }

  .lg\:focus\:placeholder-yellow-600:focus:-ms-input-placeholder {
    color: #d69e2e;
  }

  .lg\:focus\:placeholder-yellow-600:focus::-ms-input-placeholder {
    color: #d69e2e;
  }

  .lg\:focus\:placeholder-yellow-600:focus::placeholder {
    color: #d69e2e;
  }

  .lg\:focus\:placeholder-yellow-700:focus::-webkit-input-placeholder {
    color: #b7791f;
  }

  .lg\:focus\:placeholder-yellow-700:focus::-moz-placeholder {
    color: #b7791f;
  }

  .lg\:focus\:placeholder-yellow-700:focus:-ms-input-placeholder {
    color: #b7791f;
  }

  .lg\:focus\:placeholder-yellow-700:focus::-ms-input-placeholder {
    color: #b7791f;
  }

  .lg\:focus\:placeholder-yellow-700:focus::placeholder {
    color: #b7791f;
  }

  .lg\:focus\:placeholder-yellow-800:focus::-webkit-input-placeholder {
    color: #975a16;
  }

  .lg\:focus\:placeholder-yellow-800:focus::-moz-placeholder {
    color: #975a16;
  }

  .lg\:focus\:placeholder-yellow-800:focus:-ms-input-placeholder {
    color: #975a16;
  }

  .lg\:focus\:placeholder-yellow-800:focus::-ms-input-placeholder {
    color: #975a16;
  }

  .lg\:focus\:placeholder-yellow-800:focus::placeholder {
    color: #975a16;
  }

  .lg\:focus\:placeholder-yellow-900:focus::-webkit-input-placeholder {
    color: #744210;
  }

  .lg\:focus\:placeholder-yellow-900:focus::-moz-placeholder {
    color: #744210;
  }

  .lg\:focus\:placeholder-yellow-900:focus:-ms-input-placeholder {
    color: #744210;
  }

  .lg\:focus\:placeholder-yellow-900:focus::-ms-input-placeholder {
    color: #744210;
  }

  .lg\:focus\:placeholder-yellow-900:focus::placeholder {
    color: #744210;
  }

  .lg\:focus\:placeholder-green-100:focus::-webkit-input-placeholder {
    color: #f0fff4;
  }

  .lg\:focus\:placeholder-green-100:focus::-moz-placeholder {
    color: #f0fff4;
  }

  .lg\:focus\:placeholder-green-100:focus:-ms-input-placeholder {
    color: #f0fff4;
  }

  .lg\:focus\:placeholder-green-100:focus::-ms-input-placeholder {
    color: #f0fff4;
  }

  .lg\:focus\:placeholder-green-100:focus::placeholder {
    color: #f0fff4;
  }

  .lg\:focus\:placeholder-green-200:focus::-webkit-input-placeholder {
    color: #c6f6d5;
  }

  .lg\:focus\:placeholder-green-200:focus::-moz-placeholder {
    color: #c6f6d5;
  }

  .lg\:focus\:placeholder-green-200:focus:-ms-input-placeholder {
    color: #c6f6d5;
  }

  .lg\:focus\:placeholder-green-200:focus::-ms-input-placeholder {
    color: #c6f6d5;
  }

  .lg\:focus\:placeholder-green-200:focus::placeholder {
    color: #c6f6d5;
  }

  .lg\:focus\:placeholder-green-300:focus::-webkit-input-placeholder {
    color: #9ae6b4;
  }

  .lg\:focus\:placeholder-green-300:focus::-moz-placeholder {
    color: #9ae6b4;
  }

  .lg\:focus\:placeholder-green-300:focus:-ms-input-placeholder {
    color: #9ae6b4;
  }

  .lg\:focus\:placeholder-green-300:focus::-ms-input-placeholder {
    color: #9ae6b4;
  }

  .lg\:focus\:placeholder-green-300:focus::placeholder {
    color: #9ae6b4;
  }

  .lg\:focus\:placeholder-green-400:focus::-webkit-input-placeholder {
    color: #68d391;
  }

  .lg\:focus\:placeholder-green-400:focus::-moz-placeholder {
    color: #68d391;
  }

  .lg\:focus\:placeholder-green-400:focus:-ms-input-placeholder {
    color: #68d391;
  }

  .lg\:focus\:placeholder-green-400:focus::-ms-input-placeholder {
    color: #68d391;
  }

  .lg\:focus\:placeholder-green-400:focus::placeholder {
    color: #68d391;
  }

  .lg\:focus\:placeholder-green-500:focus::-webkit-input-placeholder {
    color: #48bb78;
  }

  .lg\:focus\:placeholder-green-500:focus::-moz-placeholder {
    color: #48bb78;
  }

  .lg\:focus\:placeholder-green-500:focus:-ms-input-placeholder {
    color: #48bb78;
  }

  .lg\:focus\:placeholder-green-500:focus::-ms-input-placeholder {
    color: #48bb78;
  }

  .lg\:focus\:placeholder-green-500:focus::placeholder {
    color: #48bb78;
  }

  .lg\:focus\:placeholder-green-600:focus::-webkit-input-placeholder {
    color: #38a169;
  }

  .lg\:focus\:placeholder-green-600:focus::-moz-placeholder {
    color: #38a169;
  }

  .lg\:focus\:placeholder-green-600:focus:-ms-input-placeholder {
    color: #38a169;
  }

  .lg\:focus\:placeholder-green-600:focus::-ms-input-placeholder {
    color: #38a169;
  }

  .lg\:focus\:placeholder-green-600:focus::placeholder {
    color: #38a169;
  }

  .lg\:focus\:placeholder-green-700:focus::-webkit-input-placeholder {
    color: #2f855a;
  }

  .lg\:focus\:placeholder-green-700:focus::-moz-placeholder {
    color: #2f855a;
  }

  .lg\:focus\:placeholder-green-700:focus:-ms-input-placeholder {
    color: #2f855a;
  }

  .lg\:focus\:placeholder-green-700:focus::-ms-input-placeholder {
    color: #2f855a;
  }

  .lg\:focus\:placeholder-green-700:focus::placeholder {
    color: #2f855a;
  }

  .lg\:focus\:placeholder-green-800:focus::-webkit-input-placeholder {
    color: #276749;
  }

  .lg\:focus\:placeholder-green-800:focus::-moz-placeholder {
    color: #276749;
  }

  .lg\:focus\:placeholder-green-800:focus:-ms-input-placeholder {
    color: #276749;
  }

  .lg\:focus\:placeholder-green-800:focus::-ms-input-placeholder {
    color: #276749;
  }

  .lg\:focus\:placeholder-green-800:focus::placeholder {
    color: #276749;
  }

  .lg\:focus\:placeholder-green-900:focus::-webkit-input-placeholder {
    color: #22543d;
  }

  .lg\:focus\:placeholder-green-900:focus::-moz-placeholder {
    color: #22543d;
  }

  .lg\:focus\:placeholder-green-900:focus:-ms-input-placeholder {
    color: #22543d;
  }

  .lg\:focus\:placeholder-green-900:focus::-ms-input-placeholder {
    color: #22543d;
  }

  .lg\:focus\:placeholder-green-900:focus::placeholder {
    color: #22543d;
  }

  .lg\:focus\:placeholder-teal-100:focus::-webkit-input-placeholder {
    color: #e6fffa;
  }

  .lg\:focus\:placeholder-teal-100:focus::-moz-placeholder {
    color: #e6fffa;
  }

  .lg\:focus\:placeholder-teal-100:focus:-ms-input-placeholder {
    color: #e6fffa;
  }

  .lg\:focus\:placeholder-teal-100:focus::-ms-input-placeholder {
    color: #e6fffa;
  }

  .lg\:focus\:placeholder-teal-100:focus::placeholder {
    color: #e6fffa;
  }

  .lg\:focus\:placeholder-teal-200:focus::-webkit-input-placeholder {
    color: #b2f5ea;
  }

  .lg\:focus\:placeholder-teal-200:focus::-moz-placeholder {
    color: #b2f5ea;
  }

  .lg\:focus\:placeholder-teal-200:focus:-ms-input-placeholder {
    color: #b2f5ea;
  }

  .lg\:focus\:placeholder-teal-200:focus::-ms-input-placeholder {
    color: #b2f5ea;
  }

  .lg\:focus\:placeholder-teal-200:focus::placeholder {
    color: #b2f5ea;
  }

  .lg\:focus\:placeholder-teal-300:focus::-webkit-input-placeholder {
    color: #81e6d9;
  }

  .lg\:focus\:placeholder-teal-300:focus::-moz-placeholder {
    color: #81e6d9;
  }

  .lg\:focus\:placeholder-teal-300:focus:-ms-input-placeholder {
    color: #81e6d9;
  }

  .lg\:focus\:placeholder-teal-300:focus::-ms-input-placeholder {
    color: #81e6d9;
  }

  .lg\:focus\:placeholder-teal-300:focus::placeholder {
    color: #81e6d9;
  }

  .lg\:focus\:placeholder-teal-400:focus::-webkit-input-placeholder {
    color: #4fd1c5;
  }

  .lg\:focus\:placeholder-teal-400:focus::-moz-placeholder {
    color: #4fd1c5;
  }

  .lg\:focus\:placeholder-teal-400:focus:-ms-input-placeholder {
    color: #4fd1c5;
  }

  .lg\:focus\:placeholder-teal-400:focus::-ms-input-placeholder {
    color: #4fd1c5;
  }

  .lg\:focus\:placeholder-teal-400:focus::placeholder {
    color: #4fd1c5;
  }

  .lg\:focus\:placeholder-teal-500:focus::-webkit-input-placeholder {
    color: #38b2ac;
  }

  .lg\:focus\:placeholder-teal-500:focus::-moz-placeholder {
    color: #38b2ac;
  }

  .lg\:focus\:placeholder-teal-500:focus:-ms-input-placeholder {
    color: #38b2ac;
  }

  .lg\:focus\:placeholder-teal-500:focus::-ms-input-placeholder {
    color: #38b2ac;
  }

  .lg\:focus\:placeholder-teal-500:focus::placeholder {
    color: #38b2ac;
  }

  .lg\:focus\:placeholder-teal-600:focus::-webkit-input-placeholder {
    color: #319795;
  }

  .lg\:focus\:placeholder-teal-600:focus::-moz-placeholder {
    color: #319795;
  }

  .lg\:focus\:placeholder-teal-600:focus:-ms-input-placeholder {
    color: #319795;
  }

  .lg\:focus\:placeholder-teal-600:focus::-ms-input-placeholder {
    color: #319795;
  }

  .lg\:focus\:placeholder-teal-600:focus::placeholder {
    color: #319795;
  }

  .lg\:focus\:placeholder-teal-700:focus::-webkit-input-placeholder {
    color: #2c7a7b;
  }

  .lg\:focus\:placeholder-teal-700:focus::-moz-placeholder {
    color: #2c7a7b;
  }

  .lg\:focus\:placeholder-teal-700:focus:-ms-input-placeholder {
    color: #2c7a7b;
  }

  .lg\:focus\:placeholder-teal-700:focus::-ms-input-placeholder {
    color: #2c7a7b;
  }

  .lg\:focus\:placeholder-teal-700:focus::placeholder {
    color: #2c7a7b;
  }

  .lg\:focus\:placeholder-teal-800:focus::-webkit-input-placeholder {
    color: #285e61;
  }

  .lg\:focus\:placeholder-teal-800:focus::-moz-placeholder {
    color: #285e61;
  }

  .lg\:focus\:placeholder-teal-800:focus:-ms-input-placeholder {
    color: #285e61;
  }

  .lg\:focus\:placeholder-teal-800:focus::-ms-input-placeholder {
    color: #285e61;
  }

  .lg\:focus\:placeholder-teal-800:focus::placeholder {
    color: #285e61;
  }

  .lg\:focus\:placeholder-teal-900:focus::-webkit-input-placeholder {
    color: #234e52;
  }

  .lg\:focus\:placeholder-teal-900:focus::-moz-placeholder {
    color: #234e52;
  }

  .lg\:focus\:placeholder-teal-900:focus:-ms-input-placeholder {
    color: #234e52;
  }

  .lg\:focus\:placeholder-teal-900:focus::-ms-input-placeholder {
    color: #234e52;
  }

  .lg\:focus\:placeholder-teal-900:focus::placeholder {
    color: #234e52;
  }

  .lg\:focus\:placeholder-blue-100:focus::-webkit-input-placeholder {
    color: #ebf8ff;
  }

  .lg\:focus\:placeholder-blue-100:focus::-moz-placeholder {
    color: #ebf8ff;
  }

  .lg\:focus\:placeholder-blue-100:focus:-ms-input-placeholder {
    color: #ebf8ff;
  }

  .lg\:focus\:placeholder-blue-100:focus::-ms-input-placeholder {
    color: #ebf8ff;
  }

  .lg\:focus\:placeholder-blue-100:focus::placeholder {
    color: #ebf8ff;
  }

  .lg\:focus\:placeholder-blue-200:focus::-webkit-input-placeholder {
    color: #bee3f8;
  }

  .lg\:focus\:placeholder-blue-200:focus::-moz-placeholder {
    color: #bee3f8;
  }

  .lg\:focus\:placeholder-blue-200:focus:-ms-input-placeholder {
    color: #bee3f8;
  }

  .lg\:focus\:placeholder-blue-200:focus::-ms-input-placeholder {
    color: #bee3f8;
  }

  .lg\:focus\:placeholder-blue-200:focus::placeholder {
    color: #bee3f8;
  }

  .lg\:focus\:placeholder-blue-300:focus::-webkit-input-placeholder {
    color: #90cdf4;
  }

  .lg\:focus\:placeholder-blue-300:focus::-moz-placeholder {
    color: #90cdf4;
  }

  .lg\:focus\:placeholder-blue-300:focus:-ms-input-placeholder {
    color: #90cdf4;
  }

  .lg\:focus\:placeholder-blue-300:focus::-ms-input-placeholder {
    color: #90cdf4;
  }

  .lg\:focus\:placeholder-blue-300:focus::placeholder {
    color: #90cdf4;
  }

  .lg\:focus\:placeholder-blue-400:focus::-webkit-input-placeholder {
    color: #63b3ed;
  }

  .lg\:focus\:placeholder-blue-400:focus::-moz-placeholder {
    color: #63b3ed;
  }

  .lg\:focus\:placeholder-blue-400:focus:-ms-input-placeholder {
    color: #63b3ed;
  }

  .lg\:focus\:placeholder-blue-400:focus::-ms-input-placeholder {
    color: #63b3ed;
  }

  .lg\:focus\:placeholder-blue-400:focus::placeholder {
    color: #63b3ed;
  }

  .lg\:focus\:placeholder-blue-500:focus::-webkit-input-placeholder {
    color: #4299e1;
  }

  .lg\:focus\:placeholder-blue-500:focus::-moz-placeholder {
    color: #4299e1;
  }

  .lg\:focus\:placeholder-blue-500:focus:-ms-input-placeholder {
    color: #4299e1;
  }

  .lg\:focus\:placeholder-blue-500:focus::-ms-input-placeholder {
    color: #4299e1;
  }

  .lg\:focus\:placeholder-blue-500:focus::placeholder {
    color: #4299e1;
  }

  .lg\:focus\:placeholder-blue-600:focus::-webkit-input-placeholder {
    color: #3182ce;
  }

  .lg\:focus\:placeholder-blue-600:focus::-moz-placeholder {
    color: #3182ce;
  }

  .lg\:focus\:placeholder-blue-600:focus:-ms-input-placeholder {
    color: #3182ce;
  }

  .lg\:focus\:placeholder-blue-600:focus::-ms-input-placeholder {
    color: #3182ce;
  }

  .lg\:focus\:placeholder-blue-600:focus::placeholder {
    color: #3182ce;
  }

  .lg\:focus\:placeholder-blue-700:focus::-webkit-input-placeholder {
    color: #2b6cb0;
  }

  .lg\:focus\:placeholder-blue-700:focus::-moz-placeholder {
    color: #2b6cb0;
  }

  .lg\:focus\:placeholder-blue-700:focus:-ms-input-placeholder {
    color: #2b6cb0;
  }

  .lg\:focus\:placeholder-blue-700:focus::-ms-input-placeholder {
    color: #2b6cb0;
  }

  .lg\:focus\:placeholder-blue-700:focus::placeholder {
    color: #2b6cb0;
  }

  .lg\:focus\:placeholder-blue-800:focus::-webkit-input-placeholder {
    color: #2c5282;
  }

  .lg\:focus\:placeholder-blue-800:focus::-moz-placeholder {
    color: #2c5282;
  }

  .lg\:focus\:placeholder-blue-800:focus:-ms-input-placeholder {
    color: #2c5282;
  }

  .lg\:focus\:placeholder-blue-800:focus::-ms-input-placeholder {
    color: #2c5282;
  }

  .lg\:focus\:placeholder-blue-800:focus::placeholder {
    color: #2c5282;
  }

  .lg\:focus\:placeholder-blue-900:focus::-webkit-input-placeholder {
    color: #2a4365;
  }

  .lg\:focus\:placeholder-blue-900:focus::-moz-placeholder {
    color: #2a4365;
  }

  .lg\:focus\:placeholder-blue-900:focus:-ms-input-placeholder {
    color: #2a4365;
  }

  .lg\:focus\:placeholder-blue-900:focus::-ms-input-placeholder {
    color: #2a4365;
  }

  .lg\:focus\:placeholder-blue-900:focus::placeholder {
    color: #2a4365;
  }

  .lg\:focus\:placeholder-indigo-100:focus::-webkit-input-placeholder {
    color: #ebf4ff;
  }

  .lg\:focus\:placeholder-indigo-100:focus::-moz-placeholder {
    color: #ebf4ff;
  }

  .lg\:focus\:placeholder-indigo-100:focus:-ms-input-placeholder {
    color: #ebf4ff;
  }

  .lg\:focus\:placeholder-indigo-100:focus::-ms-input-placeholder {
    color: #ebf4ff;
  }

  .lg\:focus\:placeholder-indigo-100:focus::placeholder {
    color: #ebf4ff;
  }

  .lg\:focus\:placeholder-indigo-200:focus::-webkit-input-placeholder {
    color: #c3dafe;
  }

  .lg\:focus\:placeholder-indigo-200:focus::-moz-placeholder {
    color: #c3dafe;
  }

  .lg\:focus\:placeholder-indigo-200:focus:-ms-input-placeholder {
    color: #c3dafe;
  }

  .lg\:focus\:placeholder-indigo-200:focus::-ms-input-placeholder {
    color: #c3dafe;
  }

  .lg\:focus\:placeholder-indigo-200:focus::placeholder {
    color: #c3dafe;
  }

  .lg\:focus\:placeholder-indigo-300:focus::-webkit-input-placeholder {
    color: #a3bffa;
  }

  .lg\:focus\:placeholder-indigo-300:focus::-moz-placeholder {
    color: #a3bffa;
  }

  .lg\:focus\:placeholder-indigo-300:focus:-ms-input-placeholder {
    color: #a3bffa;
  }

  .lg\:focus\:placeholder-indigo-300:focus::-ms-input-placeholder {
    color: #a3bffa;
  }

  .lg\:focus\:placeholder-indigo-300:focus::placeholder {
    color: #a3bffa;
  }

  .lg\:focus\:placeholder-indigo-400:focus::-webkit-input-placeholder {
    color: #7f9cf5;
  }

  .lg\:focus\:placeholder-indigo-400:focus::-moz-placeholder {
    color: #7f9cf5;
  }

  .lg\:focus\:placeholder-indigo-400:focus:-ms-input-placeholder {
    color: #7f9cf5;
  }

  .lg\:focus\:placeholder-indigo-400:focus::-ms-input-placeholder {
    color: #7f9cf5;
  }

  .lg\:focus\:placeholder-indigo-400:focus::placeholder {
    color: #7f9cf5;
  }

  .lg\:focus\:placeholder-indigo-500:focus::-webkit-input-placeholder {
    color: #667eea;
  }

  .lg\:focus\:placeholder-indigo-500:focus::-moz-placeholder {
    color: #667eea;
  }

  .lg\:focus\:placeholder-indigo-500:focus:-ms-input-placeholder {
    color: #667eea;
  }

  .lg\:focus\:placeholder-indigo-500:focus::-ms-input-placeholder {
    color: #667eea;
  }

  .lg\:focus\:placeholder-indigo-500:focus::placeholder {
    color: #667eea;
  }

  .lg\:focus\:placeholder-indigo-600:focus::-webkit-input-placeholder {
    color: #5a67d8;
  }

  .lg\:focus\:placeholder-indigo-600:focus::-moz-placeholder {
    color: #5a67d8;
  }

  .lg\:focus\:placeholder-indigo-600:focus:-ms-input-placeholder {
    color: #5a67d8;
  }

  .lg\:focus\:placeholder-indigo-600:focus::-ms-input-placeholder {
    color: #5a67d8;
  }

  .lg\:focus\:placeholder-indigo-600:focus::placeholder {
    color: #5a67d8;
  }

  .lg\:focus\:placeholder-indigo-700:focus::-webkit-input-placeholder {
    color: #4c51bf;
  }

  .lg\:focus\:placeholder-indigo-700:focus::-moz-placeholder {
    color: #4c51bf;
  }

  .lg\:focus\:placeholder-indigo-700:focus:-ms-input-placeholder {
    color: #4c51bf;
  }

  .lg\:focus\:placeholder-indigo-700:focus::-ms-input-placeholder {
    color: #4c51bf;
  }

  .lg\:focus\:placeholder-indigo-700:focus::placeholder {
    color: #4c51bf;
  }

  .lg\:focus\:placeholder-indigo-800:focus::-webkit-input-placeholder {
    color: #434190;
  }

  .lg\:focus\:placeholder-indigo-800:focus::-moz-placeholder {
    color: #434190;
  }

  .lg\:focus\:placeholder-indigo-800:focus:-ms-input-placeholder {
    color: #434190;
  }

  .lg\:focus\:placeholder-indigo-800:focus::-ms-input-placeholder {
    color: #434190;
  }

  .lg\:focus\:placeholder-indigo-800:focus::placeholder {
    color: #434190;
  }

  .lg\:focus\:placeholder-indigo-900:focus::-webkit-input-placeholder {
    color: #3c366b;
  }

  .lg\:focus\:placeholder-indigo-900:focus::-moz-placeholder {
    color: #3c366b;
  }

  .lg\:focus\:placeholder-indigo-900:focus:-ms-input-placeholder {
    color: #3c366b;
  }

  .lg\:focus\:placeholder-indigo-900:focus::-ms-input-placeholder {
    color: #3c366b;
  }

  .lg\:focus\:placeholder-indigo-900:focus::placeholder {
    color: #3c366b;
  }

  .lg\:focus\:placeholder-purple-100:focus::-webkit-input-placeholder {
    color: #faf5ff;
  }

  .lg\:focus\:placeholder-purple-100:focus::-moz-placeholder {
    color: #faf5ff;
  }

  .lg\:focus\:placeholder-purple-100:focus:-ms-input-placeholder {
    color: #faf5ff;
  }

  .lg\:focus\:placeholder-purple-100:focus::-ms-input-placeholder {
    color: #faf5ff;
  }

  .lg\:focus\:placeholder-purple-100:focus::placeholder {
    color: #faf5ff;
  }

  .lg\:focus\:placeholder-purple-200:focus::-webkit-input-placeholder {
    color: #e9d8fd;
  }

  .lg\:focus\:placeholder-purple-200:focus::-moz-placeholder {
    color: #e9d8fd;
  }

  .lg\:focus\:placeholder-purple-200:focus:-ms-input-placeholder {
    color: #e9d8fd;
  }

  .lg\:focus\:placeholder-purple-200:focus::-ms-input-placeholder {
    color: #e9d8fd;
  }

  .lg\:focus\:placeholder-purple-200:focus::placeholder {
    color: #e9d8fd;
  }

  .lg\:focus\:placeholder-purple-300:focus::-webkit-input-placeholder {
    color: #d6bcfa;
  }

  .lg\:focus\:placeholder-purple-300:focus::-moz-placeholder {
    color: #d6bcfa;
  }

  .lg\:focus\:placeholder-purple-300:focus:-ms-input-placeholder {
    color: #d6bcfa;
  }

  .lg\:focus\:placeholder-purple-300:focus::-ms-input-placeholder {
    color: #d6bcfa;
  }

  .lg\:focus\:placeholder-purple-300:focus::placeholder {
    color: #d6bcfa;
  }

  .lg\:focus\:placeholder-purple-400:focus::-webkit-input-placeholder {
    color: #b794f4;
  }

  .lg\:focus\:placeholder-purple-400:focus::-moz-placeholder {
    color: #b794f4;
  }

  .lg\:focus\:placeholder-purple-400:focus:-ms-input-placeholder {
    color: #b794f4;
  }

  .lg\:focus\:placeholder-purple-400:focus::-ms-input-placeholder {
    color: #b794f4;
  }

  .lg\:focus\:placeholder-purple-400:focus::placeholder {
    color: #b794f4;
  }

  .lg\:focus\:placeholder-purple-500:focus::-webkit-input-placeholder {
    color: #9f7aea;
  }

  .lg\:focus\:placeholder-purple-500:focus::-moz-placeholder {
    color: #9f7aea;
  }

  .lg\:focus\:placeholder-purple-500:focus:-ms-input-placeholder {
    color: #9f7aea;
  }

  .lg\:focus\:placeholder-purple-500:focus::-ms-input-placeholder {
    color: #9f7aea;
  }

  .lg\:focus\:placeholder-purple-500:focus::placeholder {
    color: #9f7aea;
  }

  .lg\:focus\:placeholder-purple-600:focus::-webkit-input-placeholder {
    color: #805ad5;
  }

  .lg\:focus\:placeholder-purple-600:focus::-moz-placeholder {
    color: #805ad5;
  }

  .lg\:focus\:placeholder-purple-600:focus:-ms-input-placeholder {
    color: #805ad5;
  }

  .lg\:focus\:placeholder-purple-600:focus::-ms-input-placeholder {
    color: #805ad5;
  }

  .lg\:focus\:placeholder-purple-600:focus::placeholder {
    color: #805ad5;
  }

  .lg\:focus\:placeholder-purple-700:focus::-webkit-input-placeholder {
    color: #6b46c1;
  }

  .lg\:focus\:placeholder-purple-700:focus::-moz-placeholder {
    color: #6b46c1;
  }

  .lg\:focus\:placeholder-purple-700:focus:-ms-input-placeholder {
    color: #6b46c1;
  }

  .lg\:focus\:placeholder-purple-700:focus::-ms-input-placeholder {
    color: #6b46c1;
  }

  .lg\:focus\:placeholder-purple-700:focus::placeholder {
    color: #6b46c1;
  }

  .lg\:focus\:placeholder-purple-800:focus::-webkit-input-placeholder {
    color: #553c9a;
  }

  .lg\:focus\:placeholder-purple-800:focus::-moz-placeholder {
    color: #553c9a;
  }

  .lg\:focus\:placeholder-purple-800:focus:-ms-input-placeholder {
    color: #553c9a;
  }

  .lg\:focus\:placeholder-purple-800:focus::-ms-input-placeholder {
    color: #553c9a;
  }

  .lg\:focus\:placeholder-purple-800:focus::placeholder {
    color: #553c9a;
  }

  .lg\:focus\:placeholder-purple-900:focus::-webkit-input-placeholder {
    color: #44337a;
  }

  .lg\:focus\:placeholder-purple-900:focus::-moz-placeholder {
    color: #44337a;
  }

  .lg\:focus\:placeholder-purple-900:focus:-ms-input-placeholder {
    color: #44337a;
  }

  .lg\:focus\:placeholder-purple-900:focus::-ms-input-placeholder {
    color: #44337a;
  }

  .lg\:focus\:placeholder-purple-900:focus::placeholder {
    color: #44337a;
  }

  .lg\:focus\:placeholder-pink-100:focus::-webkit-input-placeholder {
    color: #fff5f7;
  }

  .lg\:focus\:placeholder-pink-100:focus::-moz-placeholder {
    color: #fff5f7;
  }

  .lg\:focus\:placeholder-pink-100:focus:-ms-input-placeholder {
    color: #fff5f7;
  }

  .lg\:focus\:placeholder-pink-100:focus::-ms-input-placeholder {
    color: #fff5f7;
  }

  .lg\:focus\:placeholder-pink-100:focus::placeholder {
    color: #fff5f7;
  }

  .lg\:focus\:placeholder-pink-200:focus::-webkit-input-placeholder {
    color: #fed7e2;
  }

  .lg\:focus\:placeholder-pink-200:focus::-moz-placeholder {
    color: #fed7e2;
  }

  .lg\:focus\:placeholder-pink-200:focus:-ms-input-placeholder {
    color: #fed7e2;
  }

  .lg\:focus\:placeholder-pink-200:focus::-ms-input-placeholder {
    color: #fed7e2;
  }

  .lg\:focus\:placeholder-pink-200:focus::placeholder {
    color: #fed7e2;
  }

  .lg\:focus\:placeholder-pink-300:focus::-webkit-input-placeholder {
    color: #fbb6ce;
  }

  .lg\:focus\:placeholder-pink-300:focus::-moz-placeholder {
    color: #fbb6ce;
  }

  .lg\:focus\:placeholder-pink-300:focus:-ms-input-placeholder {
    color: #fbb6ce;
  }

  .lg\:focus\:placeholder-pink-300:focus::-ms-input-placeholder {
    color: #fbb6ce;
  }

  .lg\:focus\:placeholder-pink-300:focus::placeholder {
    color: #fbb6ce;
  }

  .lg\:focus\:placeholder-pink-400:focus::-webkit-input-placeholder {
    color: #f687b3;
  }

  .lg\:focus\:placeholder-pink-400:focus::-moz-placeholder {
    color: #f687b3;
  }

  .lg\:focus\:placeholder-pink-400:focus:-ms-input-placeholder {
    color: #f687b3;
  }

  .lg\:focus\:placeholder-pink-400:focus::-ms-input-placeholder {
    color: #f687b3;
  }

  .lg\:focus\:placeholder-pink-400:focus::placeholder {
    color: #f687b3;
  }

  .lg\:focus\:placeholder-pink-500:focus::-webkit-input-placeholder {
    color: #ed64a6;
  }

  .lg\:focus\:placeholder-pink-500:focus::-moz-placeholder {
    color: #ed64a6;
  }

  .lg\:focus\:placeholder-pink-500:focus:-ms-input-placeholder {
    color: #ed64a6;
  }

  .lg\:focus\:placeholder-pink-500:focus::-ms-input-placeholder {
    color: #ed64a6;
  }

  .lg\:focus\:placeholder-pink-500:focus::placeholder {
    color: #ed64a6;
  }

  .lg\:focus\:placeholder-pink-600:focus::-webkit-input-placeholder {
    color: #d53f8c;
  }

  .lg\:focus\:placeholder-pink-600:focus::-moz-placeholder {
    color: #d53f8c;
  }

  .lg\:focus\:placeholder-pink-600:focus:-ms-input-placeholder {
    color: #d53f8c;
  }

  .lg\:focus\:placeholder-pink-600:focus::-ms-input-placeholder {
    color: #d53f8c;
  }

  .lg\:focus\:placeholder-pink-600:focus::placeholder {
    color: #d53f8c;
  }

  .lg\:focus\:placeholder-pink-700:focus::-webkit-input-placeholder {
    color: #b83280;
  }

  .lg\:focus\:placeholder-pink-700:focus::-moz-placeholder {
    color: #b83280;
  }

  .lg\:focus\:placeholder-pink-700:focus:-ms-input-placeholder {
    color: #b83280;
  }

  .lg\:focus\:placeholder-pink-700:focus::-ms-input-placeholder {
    color: #b83280;
  }

  .lg\:focus\:placeholder-pink-700:focus::placeholder {
    color: #b83280;
  }

  .lg\:focus\:placeholder-pink-800:focus::-webkit-input-placeholder {
    color: #97266d;
  }

  .lg\:focus\:placeholder-pink-800:focus::-moz-placeholder {
    color: #97266d;
  }

  .lg\:focus\:placeholder-pink-800:focus:-ms-input-placeholder {
    color: #97266d;
  }

  .lg\:focus\:placeholder-pink-800:focus::-ms-input-placeholder {
    color: #97266d;
  }

  .lg\:focus\:placeholder-pink-800:focus::placeholder {
    color: #97266d;
  }

  .lg\:focus\:placeholder-pink-900:focus::-webkit-input-placeholder {
    color: #702459;
  }

  .lg\:focus\:placeholder-pink-900:focus::-moz-placeholder {
    color: #702459;
  }

  .lg\:focus\:placeholder-pink-900:focus:-ms-input-placeholder {
    color: #702459;
  }

  .lg\:focus\:placeholder-pink-900:focus::-ms-input-placeholder {
    color: #702459;
  }

  .lg\:focus\:placeholder-pink-900:focus::placeholder {
    color: #702459;
  }

  .lg\:pointer-events-none {
    pointer-events: none;
  }

  .lg\:pointer-events-auto {
    pointer-events: auto;
  }

  .lg\:static {
    position: static;
  }

  .lg\:fixed {
    position: fixed;
  }

  .lg\:absolute {
    position: absolute;
  }

  .lg\:relative {
    position: relative;
  }

  .lg\:sticky {
    position: -webkit-sticky;
    position: sticky;
  }

  .lg\:inset-0 {
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
  }

  .lg\:inset-auto {
    top: auto;
    right: auto;
    bottom: auto;
    left: auto;
  }

  .lg\:inset-y-0 {
    top: 0;
    bottom: 0;
  }

  .lg\:inset-x-0 {
    right: 0;
    left: 0;
  }

  .lg\:inset-y-auto {
    top: auto;
    bottom: auto;
  }

  .lg\:inset-x-auto {
    right: auto;
    left: auto;
  }

  .lg\:top-0 {
    top: 0;
  }

  .lg\:right-0 {
    right: 0;
  }

  .lg\:bottom-0 {
    bottom: 0;
  }

  .lg\:left-0 {
    left: 0;
  }

  .lg\:top-auto {
    top: auto;
  }

  .lg\:right-auto {
    right: auto;
  }

  .lg\:bottom-auto {
    bottom: auto;
  }

  .lg\:left-auto {
    left: auto;
  }

  .lg\:resize-none {
    resize: none;
  }

  .lg\:resize-y {
    resize: vertical;
  }

  .lg\:resize-x {
    resize: horizontal;
  }

  .lg\:resize {
    resize: both;
  }

  .lg\:shadow {
    box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06);
  }

  .lg\:shadow-md {
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
  }

  .lg\:shadow-lg {
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
  }

  .lg\:shadow-xl {
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
  }

  .lg\:shadow-2xl {
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
  }

  .lg\:shadow-inner {
    box-shadow: inset 0 2px 4px 0 rgba(0, 0, 0, 0.06);
  }

  .lg\:shadow-outline {
    box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.5);
  }

  .lg\:shadow-none {
    box-shadow: none;
  }

  .lg\:hover\:shadow:hover {
    box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06);
  }

  .lg\:hover\:shadow-md:hover {
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
  }

  .lg\:hover\:shadow-lg:hover {
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
  }

  .lg\:hover\:shadow-xl:hover {
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
  }

  .lg\:hover\:shadow-2xl:hover {
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
  }

  .lg\:hover\:shadow-inner:hover {
    box-shadow: inset 0 2px 4px 0 rgba(0, 0, 0, 0.06);
  }

  .lg\:hover\:shadow-outline:hover {
    box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.5);
  }

  .lg\:hover\:shadow-none:hover {
    box-shadow: none;
  }

  .lg\:focus\:shadow:focus {
    box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06);
  }

  .lg\:focus\:shadow-md:focus {
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
  }

  .lg\:focus\:shadow-lg:focus {
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
  }

  .lg\:focus\:shadow-xl:focus {
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
  }

  .lg\:focus\:shadow-2xl:focus {
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
  }

  .lg\:focus\:shadow-inner:focus {
    box-shadow: inset 0 2px 4px 0 rgba(0, 0, 0, 0.06);
  }

  .lg\:focus\:shadow-outline:focus {
    box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.5);
  }

  .lg\:focus\:shadow-none:focus {
    box-shadow: none;
  }

  .lg\:fill-current {
    fill: currentColor;
  }

  .lg\:stroke-current {
    stroke: currentColor;
  }

  .lg\:table-auto {
    table-layout: auto;
  }

  .lg\:table-fixed {
    table-layout: fixed;
  }

  .lg\:text-left {
    text-align: left;
  }

  .lg\:text-center {
    text-align: center;
  }

  .lg\:text-right {
    text-align: right;
  }

  .lg\:text-justify {
    text-align: justify;
  }

  .lg\:text-transparent {
    color: transparent;
  }

  .lg\:text-black {
    color: #000;
  }

  .lg\:text-white {
    color: #fff;
  }

  .lg\:text-gray-100 {
    color: #f7fafc;
  }

  .lg\:text-gray-200 {
    color: #edf2f7;
  }

  .lg\:text-gray-300 {
    color: #e2e8f0;
  }

  .lg\:text-gray-400 {
    color: #cbd5e0;
  }

  .lg\:text-gray-500 {
    color: #a0aec0;
  }

  .lg\:text-gray-600 {
    color: #718096;
  }

  .lg\:text-gray-700 {
    color: #4a5568;
  }

  .lg\:text-gray-800 {
    color: #2d3748;
  }

  .lg\:text-gray-900 {
    color: #1a202c;
  }

  .lg\:text-red-100 {
    color: #fff5f5;
  }

  .lg\:text-red-200 {
    color: #fed7d7;
  }

  .lg\:text-red-300 {
    color: #feb2b2;
  }

  .lg\:text-red-400 {
    color: #fc8181;
  }

  .lg\:text-red-500 {
    color: #f56565;
  }

  .lg\:text-red-600 {
    color: #e53e3e;
  }

  .lg\:text-red-700 {
    color: #c53030;
  }

  .lg\:text-red-800 {
    color: #9b2c2c;
  }

  .lg\:text-red-900 {
    color: #742a2a;
  }

  .lg\:text-orange-100 {
    color: #fffaf0;
  }

  .lg\:text-orange-200 {
    color: #feebc8;
  }

  .lg\:text-orange-300 {
    color: #fbd38d;
  }

  .lg\:text-orange-400 {
    color: #f6ad55;
  }

  .lg\:text-orange-500 {
    color: #ed8936;
  }

  .lg\:text-orange-600 {
    color: #dd6b20;
  }

  .lg\:text-orange-700 {
    color: #c05621;
  }

  .lg\:text-orange-800 {
    color: #9c4221;
  }

  .lg\:text-orange-900 {
    color: #7b341e;
  }

  .lg\:text-yellow-100 {
    color: #fffff0;
  }

  .lg\:text-yellow-200 {
    color: #fefcbf;
  }

  .lg\:text-yellow-300 {
    color: #faf089;
  }

  .lg\:text-yellow-400 {
    color: #f6e05e;
  }

  .lg\:text-yellow-500 {
    color: #ecc94b;
  }

  .lg\:text-yellow-600 {
    color: #d69e2e;
  }

  .lg\:text-yellow-700 {
    color: #b7791f;
  }

  .lg\:text-yellow-800 {
    color: #975a16;
  }

  .lg\:text-yellow-900 {
    color: #744210;
  }

  .lg\:text-green-100 {
    color: #f0fff4;
  }

  .lg\:text-green-200 {
    color: #c6f6d5;
  }

  .lg\:text-green-300 {
    color: #9ae6b4;
  }

  .lg\:text-green-400 {
    color: #68d391;
  }

  .lg\:text-green-500 {
    color: #48bb78;
  }

  .lg\:text-green-600 {
    color: #38a169;
  }

  .lg\:text-green-700 {
    color: #2f855a;
  }

  .lg\:text-green-800 {
    color: #276749;
  }

  .lg\:text-green-900 {
    color: #22543d;
  }

  .lg\:text-teal-100 {
    color: #e6fffa;
  }

  .lg\:text-teal-200 {
    color: #b2f5ea;
  }

  .lg\:text-teal-300 {
    color: #81e6d9;
  }

  .lg\:text-teal-400 {
    color: #4fd1c5;
  }

  .lg\:text-teal-500 {
    color: #38b2ac;
  }

  .lg\:text-teal-600 {
    color: #319795;
  }

  .lg\:text-teal-700 {
    color: #2c7a7b;
  }

  .lg\:text-teal-800 {
    color: #285e61;
  }

  .lg\:text-teal-900 {
    color: #234e52;
  }

  .lg\:text-blue-100 {
    color: #ebf8ff;
  }

  .lg\:text-blue-200 {
    color: #bee3f8;
  }

  .lg\:text-blue-300 {
    color: #90cdf4;
  }

  .lg\:text-blue-400 {
    color: #63b3ed;
  }

  .lg\:text-blue-500 {
    color: #4299e1;
  }

  .lg\:text-blue-600 {
    color: #3182ce;
  }

  .lg\:text-blue-700 {
    color: #2b6cb0;
  }

  .lg\:text-blue-800 {
    color: #2c5282;
  }

  .lg\:text-blue-900 {
    color: #2a4365;
  }

  .lg\:text-indigo-100 {
    color: #ebf4ff;
  }

  .lg\:text-indigo-200 {
    color: #c3dafe;
  }

  .lg\:text-indigo-300 {
    color: #a3bffa;
  }

  .lg\:text-indigo-400 {
    color: #7f9cf5;
  }

  .lg\:text-indigo-500 {
    color: #667eea;
  }

  .lg\:text-indigo-600 {
    color: #5a67d8;
  }

  .lg\:text-indigo-700 {
    color: #4c51bf;
  }

  .lg\:text-indigo-800 {
    color: #434190;
  }

  .lg\:text-indigo-900 {
    color: #3c366b;
  }

  .lg\:text-purple-100 {
    color: #faf5ff;
  }

  .lg\:text-purple-200 {
    color: #e9d8fd;
  }

  .lg\:text-purple-300 {
    color: #d6bcfa;
  }

  .lg\:text-purple-400 {
    color: #b794f4;
  }

  .lg\:text-purple-500 {
    color: #9f7aea;
  }

  .lg\:text-purple-600 {
    color: #805ad5;
  }

  .lg\:text-purple-700 {
    color: #6b46c1;
  }

  .lg\:text-purple-800 {
    color: #553c9a;
  }

  .lg\:text-purple-900 {
    color: #44337a;
  }

  .lg\:text-pink-100 {
    color: #fff5f7;
  }

  .lg\:text-pink-200 {
    color: #fed7e2;
  }

  .lg\:text-pink-300 {
    color: #fbb6ce;
  }

  .lg\:text-pink-400 {
    color: #f687b3;
  }

  .lg\:text-pink-500 {
    color: #ed64a6;
  }

  .lg\:text-pink-600 {
    color: #d53f8c;
  }

  .lg\:text-pink-700 {
    color: #b83280;
  }

  .lg\:text-pink-800 {
    color: #97266d;
  }

  .lg\:text-pink-900 {
    color: #702459;
  }

  .lg\:hover\:text-transparent:hover {
    color: transparent;
  }

  .lg\:hover\:text-black:hover {
    color: #000;
  }

  .lg\:hover\:text-white:hover {
    color: #fff;
  }

  .lg\:hover\:text-gray-100:hover {
    color: #f7fafc;
  }

  .lg\:hover\:text-gray-200:hover {
    color: #edf2f7;
  }

  .lg\:hover\:text-gray-300:hover {
    color: #e2e8f0;
  }

  .lg\:hover\:text-gray-400:hover {
    color: #cbd5e0;
  }

  .lg\:hover\:text-gray-500:hover {
    color: #a0aec0;
  }

  .lg\:hover\:text-gray-600:hover {
    color: #718096;
  }

  .lg\:hover\:text-gray-700:hover {
    color: #4a5568;
  }

  .lg\:hover\:text-gray-800:hover {
    color: #2d3748;
  }

  .lg\:hover\:text-gray-900:hover {
    color: #1a202c;
  }

  .lg\:hover\:text-red-100:hover {
    color: #fff5f5;
  }

  .lg\:hover\:text-red-200:hover {
    color: #fed7d7;
  }

  .lg\:hover\:text-red-300:hover {
    color: #feb2b2;
  }

  .lg\:hover\:text-red-400:hover {
    color: #fc8181;
  }

  .lg\:hover\:text-red-500:hover {
    color: #f56565;
  }

  .lg\:hover\:text-red-600:hover {
    color: #e53e3e;
  }

  .lg\:hover\:text-red-700:hover {
    color: #c53030;
  }

  .lg\:hover\:text-red-800:hover {
    color: #9b2c2c;
  }

  .lg\:hover\:text-red-900:hover {
    color: #742a2a;
  }

  .lg\:hover\:text-orange-100:hover {
    color: #fffaf0;
  }

  .lg\:hover\:text-orange-200:hover {
    color: #feebc8;
  }

  .lg\:hover\:text-orange-300:hover {
    color: #fbd38d;
  }

  .lg\:hover\:text-orange-400:hover {
    color: #f6ad55;
  }

  .lg\:hover\:text-orange-500:hover {
    color: #ed8936;
  }

  .lg\:hover\:text-orange-600:hover {
    color: #dd6b20;
  }

  .lg\:hover\:text-orange-700:hover {
    color: #c05621;
  }

  .lg\:hover\:text-orange-800:hover {
    color: #9c4221;
  }

  .lg\:hover\:text-orange-900:hover {
    color: #7b341e;
  }

  .lg\:hover\:text-yellow-100:hover {
    color: #fffff0;
  }

  .lg\:hover\:text-yellow-200:hover {
    color: #fefcbf;
  }

  .lg\:hover\:text-yellow-300:hover {
    color: #faf089;
  }

  .lg\:hover\:text-yellow-400:hover {
    color: #f6e05e;
  }

  .lg\:hover\:text-yellow-500:hover {
    color: #ecc94b;
  }

  .lg\:hover\:text-yellow-600:hover {
    color: #d69e2e;
  }

  .lg\:hover\:text-yellow-700:hover {
    color: #b7791f;
  }

  .lg\:hover\:text-yellow-800:hover {
    color: #975a16;
  }

  .lg\:hover\:text-yellow-900:hover {
    color: #744210;
  }

  .lg\:hover\:text-green-100:hover {
    color: #f0fff4;
  }

  .lg\:hover\:text-green-200:hover {
    color: #c6f6d5;
  }

  .lg\:hover\:text-green-300:hover {
    color: #9ae6b4;
  }

  .lg\:hover\:text-green-400:hover {
    color: #68d391;
  }

  .lg\:hover\:text-green-500:hover {
    color: #48bb78;
  }

  .lg\:hover\:text-green-600:hover {
    color: #38a169;
  }

  .lg\:hover\:text-green-700:hover {
    color: #2f855a;
  }

  .lg\:hover\:text-green-800:hover {
    color: #276749;
  }

  .lg\:hover\:text-green-900:hover {
    color: #22543d;
  }

  .lg\:hover\:text-teal-100:hover {
    color: #e6fffa;
  }

  .lg\:hover\:text-teal-200:hover {
    color: #b2f5ea;
  }

  .lg\:hover\:text-teal-300:hover {
    color: #81e6d9;
  }

  .lg\:hover\:text-teal-400:hover {
    color: #4fd1c5;
  }

  .lg\:hover\:text-teal-500:hover {
    color: #38b2ac;
  }

  .lg\:hover\:text-teal-600:hover {
    color: #319795;
  }

  .lg\:hover\:text-teal-700:hover {
    color: #2c7a7b;
  }

  .lg\:hover\:text-teal-800:hover {
    color: #285e61;
  }

  .lg\:hover\:text-teal-900:hover {
    color: #234e52;
  }

  .lg\:hover\:text-blue-100:hover {
    color: #ebf8ff;
  }

  .lg\:hover\:text-blue-200:hover {
    color: #bee3f8;
  }

  .lg\:hover\:text-blue-300:hover {
    color: #90cdf4;
  }

  .lg\:hover\:text-blue-400:hover {
    color: #63b3ed;
  }

  .lg\:hover\:text-blue-500:hover {
    color: #4299e1;
  }

  .lg\:hover\:text-blue-600:hover {
    color: #3182ce;
  }

  .lg\:hover\:text-blue-700:hover {
    color: #2b6cb0;
  }

  .lg\:hover\:text-blue-800:hover {
    color: #2c5282;
  }

  .lg\:hover\:text-blue-900:hover {
    color: #2a4365;
  }

  .lg\:hover\:text-indigo-100:hover {
    color: #ebf4ff;
  }

  .lg\:hover\:text-indigo-200:hover {
    color: #c3dafe;
  }

  .lg\:hover\:text-indigo-300:hover {
    color: #a3bffa;
  }

  .lg\:hover\:text-indigo-400:hover {
    color: #7f9cf5;
  }

  .lg\:hover\:text-indigo-500:hover {
    color: #667eea;
  }

  .lg\:hover\:text-indigo-600:hover {
    color: #5a67d8;
  }

  .lg\:hover\:text-indigo-700:hover {
    color: #4c51bf;
  }

  .lg\:hover\:text-indigo-800:hover {
    color: #434190;
  }

  .lg\:hover\:text-indigo-900:hover {
    color: #3c366b;
  }

  .lg\:hover\:text-purple-100:hover {
    color: #faf5ff;
  }

  .lg\:hover\:text-purple-200:hover {
    color: #e9d8fd;
  }

  .lg\:hover\:text-purple-300:hover {
    color: #d6bcfa;
  }

  .lg\:hover\:text-purple-400:hover {
    color: #b794f4;
  }

  .lg\:hover\:text-purple-500:hover {
    color: #9f7aea;
  }

  .lg\:hover\:text-purple-600:hover {
    color: #805ad5;
  }

  .lg\:hover\:text-purple-700:hover {
    color: #6b46c1;
  }

  .lg\:hover\:text-purple-800:hover {
    color: #553c9a;
  }

  .lg\:hover\:text-purple-900:hover {
    color: #44337a;
  }

  .lg\:hover\:text-pink-100:hover {
    color: #fff5f7;
  }

  .lg\:hover\:text-pink-200:hover {
    color: #fed7e2;
  }

  .lg\:hover\:text-pink-300:hover {
    color: #fbb6ce;
  }

  .lg\:hover\:text-pink-400:hover {
    color: #f687b3;
  }

  .lg\:hover\:text-pink-500:hover {
    color: #ed64a6;
  }

  .lg\:hover\:text-pink-600:hover {
    color: #d53f8c;
  }

  .lg\:hover\:text-pink-700:hover {
    color: #b83280;
  }

  .lg\:hover\:text-pink-800:hover {
    color: #97266d;
  }

  .lg\:hover\:text-pink-900:hover {
    color: #702459;
  }

  .lg\:focus\:text-transparent:focus {
    color: transparent;
  }

  .lg\:focus\:text-black:focus {
    color: #000;
  }

  .lg\:focus\:text-white:focus {
    color: #fff;
  }

  .lg\:focus\:text-gray-100:focus {
    color: #f7fafc;
  }

  .lg\:focus\:text-gray-200:focus {
    color: #edf2f7;
  }

  .lg\:focus\:text-gray-300:focus {
    color: #e2e8f0;
  }

  .lg\:focus\:text-gray-400:focus {
    color: #cbd5e0;
  }

  .lg\:focus\:text-gray-500:focus {
    color: #a0aec0;
  }

  .lg\:focus\:text-gray-600:focus {
    color: #718096;
  }

  .lg\:focus\:text-gray-700:focus {
    color: #4a5568;
  }

  .lg\:focus\:text-gray-800:focus {
    color: #2d3748;
  }

  .lg\:focus\:text-gray-900:focus {
    color: #1a202c;
  }

  .lg\:focus\:text-red-100:focus {
    color: #fff5f5;
  }

  .lg\:focus\:text-red-200:focus {
    color: #fed7d7;
  }

  .lg\:focus\:text-red-300:focus {
    color: #feb2b2;
  }

  .lg\:focus\:text-red-400:focus {
    color: #fc8181;
  }

  .lg\:focus\:text-red-500:focus {
    color: #f56565;
  }

  .lg\:focus\:text-red-600:focus {
    color: #e53e3e;
  }

  .lg\:focus\:text-red-700:focus {
    color: #c53030;
  }

  .lg\:focus\:text-red-800:focus {
    color: #9b2c2c;
  }

  .lg\:focus\:text-red-900:focus {
    color: #742a2a;
  }

  .lg\:focus\:text-orange-100:focus {
    color: #fffaf0;
  }

  .lg\:focus\:text-orange-200:focus {
    color: #feebc8;
  }

  .lg\:focus\:text-orange-300:focus {
    color: #fbd38d;
  }

  .lg\:focus\:text-orange-400:focus {
    color: #f6ad55;
  }

  .lg\:focus\:text-orange-500:focus {
    color: #ed8936;
  }

  .lg\:focus\:text-orange-600:focus {
    color: #dd6b20;
  }

  .lg\:focus\:text-orange-700:focus {
    color: #c05621;
  }

  .lg\:focus\:text-orange-800:focus {
    color: #9c4221;
  }

  .lg\:focus\:text-orange-900:focus {
    color: #7b341e;
  }

  .lg\:focus\:text-yellow-100:focus {
    color: #fffff0;
  }

  .lg\:focus\:text-yellow-200:focus {
    color: #fefcbf;
  }

  .lg\:focus\:text-yellow-300:focus {
    color: #faf089;
  }

  .lg\:focus\:text-yellow-400:focus {
    color: #f6e05e;
  }

  .lg\:focus\:text-yellow-500:focus {
    color: #ecc94b;
  }

  .lg\:focus\:text-yellow-600:focus {
    color: #d69e2e;
  }

  .lg\:focus\:text-yellow-700:focus {
    color: #b7791f;
  }

  .lg\:focus\:text-yellow-800:focus {
    color: #975a16;
  }

  .lg\:focus\:text-yellow-900:focus {
    color: #744210;
  }

  .lg\:focus\:text-green-100:focus {
    color: #f0fff4;
  }

  .lg\:focus\:text-green-200:focus {
    color: #c6f6d5;
  }

  .lg\:focus\:text-green-300:focus {
    color: #9ae6b4;
  }

  .lg\:focus\:text-green-400:focus {
    color: #68d391;
  }

  .lg\:focus\:text-green-500:focus {
    color: #48bb78;
  }

  .lg\:focus\:text-green-600:focus {
    color: #38a169;
  }

  .lg\:focus\:text-green-700:focus {
    color: #2f855a;
  }

  .lg\:focus\:text-green-800:focus {
    color: #276749;
  }

  .lg\:focus\:text-green-900:focus {
    color: #22543d;
  }

  .lg\:focus\:text-teal-100:focus {
    color: #e6fffa;
  }

  .lg\:focus\:text-teal-200:focus {
    color: #b2f5ea;
  }

  .lg\:focus\:text-teal-300:focus {
    color: #81e6d9;
  }

  .lg\:focus\:text-teal-400:focus {
    color: #4fd1c5;
  }

  .lg\:focus\:text-teal-500:focus {
    color: #38b2ac;
  }

  .lg\:focus\:text-teal-600:focus {
    color: #319795;
  }

  .lg\:focus\:text-teal-700:focus {
    color: #2c7a7b;
  }

  .lg\:focus\:text-teal-800:focus {
    color: #285e61;
  }

  .lg\:focus\:text-teal-900:focus {
    color: #234e52;
  }

  .lg\:focus\:text-blue-100:focus {
    color: #ebf8ff;
  }

  .lg\:focus\:text-blue-200:focus {
    color: #bee3f8;
  }

  .lg\:focus\:text-blue-300:focus {
    color: #90cdf4;
  }

  .lg\:focus\:text-blue-400:focus {
    color: #63b3ed;
  }

  .lg\:focus\:text-blue-500:focus {
    color: #4299e1;
  }

  .lg\:focus\:text-blue-600:focus {
    color: #3182ce;
  }

  .lg\:focus\:text-blue-700:focus {
    color: #2b6cb0;
  }

  .lg\:focus\:text-blue-800:focus {
    color: #2c5282;
  }

  .lg\:focus\:text-blue-900:focus {
    color: #2a4365;
  }

  .lg\:focus\:text-indigo-100:focus {
    color: #ebf4ff;
  }

  .lg\:focus\:text-indigo-200:focus {
    color: #c3dafe;
  }

  .lg\:focus\:text-indigo-300:focus {
    color: #a3bffa;
  }

  .lg\:focus\:text-indigo-400:focus {
    color: #7f9cf5;
  }

  .lg\:focus\:text-indigo-500:focus {
    color: #667eea;
  }

  .lg\:focus\:text-indigo-600:focus {
    color: #5a67d8;
  }

  .lg\:focus\:text-indigo-700:focus {
    color: #4c51bf;
  }

  .lg\:focus\:text-indigo-800:focus {
    color: #434190;
  }

  .lg\:focus\:text-indigo-900:focus {
    color: #3c366b;
  }

  .lg\:focus\:text-purple-100:focus {
    color: #faf5ff;
  }

  .lg\:focus\:text-purple-200:focus {
    color: #e9d8fd;
  }

  .lg\:focus\:text-purple-300:focus {
    color: #d6bcfa;
  }

  .lg\:focus\:text-purple-400:focus {
    color: #b794f4;
  }

  .lg\:focus\:text-purple-500:focus {
    color: #9f7aea;
  }

  .lg\:focus\:text-purple-600:focus {
    color: #805ad5;
  }

  .lg\:focus\:text-purple-700:focus {
    color: #6b46c1;
  }

  .lg\:focus\:text-purple-800:focus {
    color: #553c9a;
  }

  .lg\:focus\:text-purple-900:focus {
    color: #44337a;
  }

  .lg\:focus\:text-pink-100:focus {
    color: #fff5f7;
  }

  .lg\:focus\:text-pink-200:focus {
    color: #fed7e2;
  }

  .lg\:focus\:text-pink-300:focus {
    color: #fbb6ce;
  }

  .lg\:focus\:text-pink-400:focus {
    color: #f687b3;
  }

  .lg\:focus\:text-pink-500:focus {
    color: #ed64a6;
  }

  .lg\:focus\:text-pink-600:focus {
    color: #d53f8c;
  }

  .lg\:focus\:text-pink-700:focus {
    color: #b83280;
  }

  .lg\:focus\:text-pink-800:focus {
    color: #97266d;
  }

  .lg\:focus\:text-pink-900:focus {
    color: #702459;
  }

  .lg\:text-xs {
    font-size: 0.75rem;
  }

  .lg\:text-sm {
    font-size: 0.875rem;
  }

  .lg\:text-base {
    font-size: 1rem;
  }

  .lg\:text-lg {
    font-size: 1.125rem;
  }

  .lg\:text-xl {
    font-size: 1.25rem;
  }

  .lg\:text-2xl {
    font-size: 1.5rem;
  }

  .lg\:text-3xl {
    font-size: 1.875rem;
  }

  .lg\:text-4xl {
    font-size: 2.25rem;
  }

  .lg\:text-5xl {
    font-size: 3rem;
  }

  .lg\:text-6xl {
    font-size: 4rem;
  }

  .lg\:italic {
    font-style: italic;
  }

  .lg\:not-italic {
    font-style: normal;
  }

  .lg\:uppercase {
    text-transform: uppercase;
  }

  .lg\:lowercase {
    text-transform: lowercase;
  }

  .lg\:capitalize {
    text-transform: capitalize;
  }

  .lg\:normal-case {
    text-transform: none;
  }

  .lg\:underline {
    text-decoration: underline;
  }

  .lg\:line-through {
    text-decoration: line-through;
  }

  .lg\:no-underline {
    text-decoration: none;
  }

  .lg\:hover\:underline:hover {
    text-decoration: underline;
  }

  .lg\:hover\:line-through:hover {
    text-decoration: line-through;
  }

  .lg\:hover\:no-underline:hover {
    text-decoration: none;
  }

  .lg\:focus\:underline:focus {
    text-decoration: underline;
  }

  .lg\:focus\:line-through:focus {
    text-decoration: line-through;
  }

  .lg\:focus\:no-underline:focus {
    text-decoration: none;
  }

  .lg\:antialiased {
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
  }

  .lg\:subpixel-antialiased {
    -webkit-font-smoothing: auto;
    -moz-osx-font-smoothing: auto;
  }

  .lg\:tracking-tighter {
    letter-spacing: -0.05em;
  }

  .lg\:tracking-tight {
    letter-spacing: -0.025em;
  }

  .lg\:tracking-normal {
    letter-spacing: 0;
  }

  .lg\:tracking-wide {
    letter-spacing: 0.025em;
  }

  .lg\:tracking-wider {
    letter-spacing: 0.05em;
  }

  .lg\:tracking-widest {
    letter-spacing: 0.1em;
  }

  .lg\:select-none {
    -webkit-user-select: none;
       -moz-user-select: none;
        -ms-user-select: none;
            user-select: none;
  }

  .lg\:select-text {
    -webkit-user-select: text;
       -moz-user-select: text;
        -ms-user-select: text;
            user-select: text;
  }

  .lg\:select-all {
    -webkit-user-select: all;
       -moz-user-select: all;
        -ms-user-select: all;
            user-select: all;
  }

  .lg\:select-auto {
    -webkit-user-select: auto;
       -moz-user-select: auto;
        -ms-user-select: auto;
            user-select: auto;
  }

  .lg\:align-baseline {
    vertical-align: baseline;
  }

  .lg\:align-top {
    vertical-align: top;
  }

  .lg\:align-middle {
    vertical-align: middle;
  }

  .lg\:align-bottom {
    vertical-align: bottom;
  }

  .lg\:align-text-top {
    vertical-align: text-top;
  }

  .lg\:align-text-bottom {
    vertical-align: text-bottom;
  }

  .lg\:visible {
    visibility: visible;
  }

  .lg\:invisible {
    visibility: hidden;
  }

  .lg\:whitespace-normal {
    white-space: normal;
  }

  .lg\:whitespace-no-wrap {
    white-space: nowrap;
  }

  .lg\:whitespace-pre {
    white-space: pre;
  }

  .lg\:whitespace-pre-line {
    white-space: pre-line;
  }

  .lg\:whitespace-pre-wrap {
    white-space: pre-wrap;
  }

  .lg\:break-normal {
    overflow-wrap: normal;
    word-break: normal;
  }

  .lg\:break-words {
    overflow-wrap: break-word;
  }

  .lg\:break-all {
    word-break: break-all;
  }

  .lg\:truncate {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .lg\:w-0 {
    width: 0;
  }

  .lg\:w-1 {
    width: 0.25rem;
  }

  .lg\:w-2 {
    width: 0.5rem;
  }

  .lg\:w-3 {
    width: 0.75rem;
  }

  .lg\:w-4 {
    width: 1rem;
  }

  .lg\:w-5 {
    width: 1.25rem;
  }

  .lg\:w-6 {
    width: 1.5rem;
  }

  .lg\:w-8 {
    width: 2rem;
  }

  .lg\:w-10 {
    width: 2.5rem;
  }

  .lg\:w-12 {
    width: 3rem;
  }

  .lg\:w-16 {
    width: 4rem;
  }

  .lg\:w-20 {
    width: 5rem;
  }

  .lg\:w-24 {
    width: 6rem;
  }

  .lg\:w-32 {
    width: 8rem;
  }

  .lg\:w-40 {
    width: 10rem;
  }

  .lg\:w-48 {
    width: 12rem;
  }

  .lg\:w-56 {
    width: 14rem;
  }

  .lg\:w-64 {
    width: 16rem;
  }

  .lg\:w-auto {
    width: auto;
  }

  .lg\:w-px {
    width: 1px;
  }

  .lg\:w-1\/2 {
    width: 50%;
  }

  .lg\:w-1\/3 {
    width: 33.333333%;
  }

  .lg\:w-2\/3 {
    width: 66.666667%;
  }

  .lg\:w-1\/4 {
    width: 25%;
  }

  .lg\:w-2\/4 {
    width: 50%;
  }

  .lg\:w-3\/4 {
    width: 75%;
  }

  .lg\:w-1\/5 {
    width: 20%;
  }

  .lg\:w-2\/5 {
    width: 40%;
  }

  .lg\:w-3\/5 {
    width: 60%;
  }

  .lg\:w-4\/5 {
    width: 80%;
  }

  .lg\:w-1\/6 {
    width: 16.666667%;
  }

  .lg\:w-2\/6 {
    width: 33.333333%;
  }

  .lg\:w-3\/6 {
    width: 50%;
  }

  .lg\:w-4\/6 {
    width: 66.666667%;
  }

  .lg\:w-5\/6 {
    width: 83.333333%;
  }

  .lg\:w-1\/12 {
    width: 8.333333%;
  }

  .lg\:w-2\/12 {
    width: 16.666667%;
  }

  .lg\:w-3\/12 {
    width: 25%;
  }

  .lg\:w-4\/12 {
    width: 33.333333%;
  }

  .lg\:w-5\/12 {
    width: 41.666667%;
  }

  .lg\:w-6\/12 {
    width: 50%;
  }

  .lg\:w-7\/12 {
    width: 58.333333%;
  }

  .lg\:w-8\/12 {
    width: 66.666667%;
  }

  .lg\:w-9\/12 {
    width: 75%;
  }

  .lg\:w-10\/12 {
    width: 83.333333%;
  }

  .lg\:w-11\/12 {
    width: 91.666667%;
  }

  .lg\:w-full {
    width: 100%;
  }

  .lg\:w-screen {
    width: 100vw;
  }

  .lg\:z-0 {
    z-index: 0;
  }

  .lg\:z-10 {
    z-index: 10;
  }

  .lg\:z-20 {
    z-index: 20;
  }

  .lg\:z-30 {
    z-index: 30;
  }

  .lg\:z-40 {
    z-index: 40;
  }

  .lg\:z-50 {
    z-index: 50;
  }

  .lg\:z-auto {
    z-index: auto;
  }
}

@media (min-width: 1280px) {
  .xl\:sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border-width: 0;
  }

  .xl\:not-sr-only {
    position: static;
    width: auto;
    height: auto;
    padding: 0;
    margin: 0;
    overflow: visible;
    clip: auto;
    white-space: normal;
  }

  .xl\:focus\:sr-only:focus {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border-width: 0;
  }

  .xl\:focus\:not-sr-only:focus {
    position: static;
    width: auto;
    height: auto;
    padding: 0;
    margin: 0;
    overflow: visible;
    clip: auto;
    white-space: normal;
  }

  .xl\:appearance-none {
    -webkit-appearance: none;
       -moz-appearance: none;
            appearance: none;
  }

  .xl\:bg-fixed {
    background-attachment: fixed;
  }

  .xl\:bg-local {
    background-attachment: local;
  }

  .xl\:bg-scroll {
    background-attachment: scroll;
  }

  .xl\:bg-transparent {
    background-color: transparent;
  }

  .xl\:bg-black {
    background-color: #000;
  }

  .xl\:bg-white {
    background-color: #fff;
  }

  .xl\:bg-gray-100 {
    background-color: #f7fafc;
  }

  .xl\:bg-gray-200 {
    background-color: #edf2f7;
  }

  .xl\:bg-gray-300 {
    background-color: #e2e8f0;
  }

  .xl\:bg-gray-400 {
    background-color: #cbd5e0;
  }

  .xl\:bg-gray-500 {
    background-color: #a0aec0;
  }

  .xl\:bg-gray-600 {
    background-color: #718096;
  }

  .xl\:bg-gray-700 {
    background-color: #4a5568;
  }

  .xl\:bg-gray-800 {
    background-color: #2d3748;
  }

  .xl\:bg-gray-900 {
    background-color: #1a202c;
  }

  .xl\:bg-red-100 {
    background-color: #fff5f5;
  }

  .xl\:bg-red-200 {
    background-color: #fed7d7;
  }

  .xl\:bg-red-300 {
    background-color: #feb2b2;
  }

  .xl\:bg-red-400 {
    background-color: #fc8181;
  }

  .xl\:bg-red-500 {
    background-color: #f56565;
  }

  .xl\:bg-red-600 {
    background-color: #e53e3e;
  }

  .xl\:bg-red-700 {
    background-color: #c53030;
  }

  .xl\:bg-red-800 {
    background-color: #9b2c2c;
  }

  .xl\:bg-red-900 {
    background-color: #742a2a;
  }

  .xl\:bg-orange-100 {
    background-color: #fffaf0;
  }

  .xl\:bg-orange-200 {
    background-color: #feebc8;
  }

  .xl\:bg-orange-300 {
    background-color: #fbd38d;
  }

  .xl\:bg-orange-400 {
    background-color: #f6ad55;
  }

  .xl\:bg-orange-500 {
    background-color: #ed8936;
  }

  .xl\:bg-orange-600 {
    background-color: #dd6b20;
  }

  .xl\:bg-orange-700 {
    background-color: #c05621;
  }

  .xl\:bg-orange-800 {
    background-color: #9c4221;
  }

  .xl\:bg-orange-900 {
    background-color: #7b341e;
  }

  .xl\:bg-yellow-100 {
    background-color: #fffff0;
  }

  .xl\:bg-yellow-200 {
    background-color: #fefcbf;
  }

  .xl\:bg-yellow-300 {
    background-color: #faf089;
  }

  .xl\:bg-yellow-400 {
    background-color: #f6e05e;
  }

  .xl\:bg-yellow-500 {
    background-color: #ecc94b;
  }

  .xl\:bg-yellow-600 {
    background-color: #d69e2e;
  }

  .xl\:bg-yellow-700 {
    background-color: #b7791f;
  }

  .xl\:bg-yellow-800 {
    background-color: #975a16;
  }

  .xl\:bg-yellow-900 {
    background-color: #744210;
  }

  .xl\:bg-green-100 {
    background-color: #f0fff4;
  }

  .xl\:bg-green-200 {
    background-color: #c6f6d5;
  }

  .xl\:bg-green-300 {
    background-color: #9ae6b4;
  }

  .xl\:bg-green-400 {
    background-color: #68d391;
  }

  .xl\:bg-green-500 {
    background-color: #48bb78;
  }

  .xl\:bg-green-600 {
    background-color: #38a169;
  }

  .xl\:bg-green-700 {
    background-color: #2f855a;
  }

  .xl\:bg-green-800 {
    background-color: #276749;
  }

  .xl\:bg-green-900 {
    background-color: #22543d;
  }

  .xl\:bg-teal-100 {
    background-color: #e6fffa;
  }

  .xl\:bg-teal-200 {
    background-color: #b2f5ea;
  }

  .xl\:bg-teal-300 {
    background-color: #81e6d9;
  }

  .xl\:bg-teal-400 {
    background-color: #4fd1c5;
  }

  .xl\:bg-teal-500 {
    background-color: #38b2ac;
  }

  .xl\:bg-teal-600 {
    background-color: #319795;
  }

  .xl\:bg-teal-700 {
    background-color: #2c7a7b;
  }

  .xl\:bg-teal-800 {
    background-color: #285e61;
  }

  .xl\:bg-teal-900 {
    background-color: #234e52;
  }

  .xl\:bg-blue-100 {
    background-color: #ebf8ff;
  }

  .xl\:bg-blue-200 {
    background-color: #bee3f8;
  }

  .xl\:bg-blue-300 {
    background-color: #90cdf4;
  }

  .xl\:bg-blue-400 {
    background-color: #63b3ed;
  }

  .xl\:bg-blue-500 {
    background-color: #4299e1;
  }

  .xl\:bg-blue-600 {
    background-color: #3182ce;
  }

  .xl\:bg-blue-700 {
    background-color: #2b6cb0;
  }

  .xl\:bg-blue-800 {
    background-color: #2c5282;
  }

  .xl\:bg-blue-900 {
    background-color: #2a4365;
  }

  .xl\:bg-indigo-100 {
    background-color: #ebf4ff;
  }

  .xl\:bg-indigo-200 {
    background-color: #c3dafe;
  }

  .xl\:bg-indigo-300 {
    background-color: #a3bffa;
  }

  .xl\:bg-indigo-400 {
    background-color: #7f9cf5;
  }

  .xl\:bg-indigo-500 {
    background-color: #667eea;
  }

  .xl\:bg-indigo-600 {
    background-color: #5a67d8;
  }

  .xl\:bg-indigo-700 {
    background-color: #4c51bf;
  }

  .xl\:bg-indigo-800 {
    background-color: #434190;
  }

  .xl\:bg-indigo-900 {
    background-color: #3c366b;
  }

  .xl\:bg-purple-100 {
    background-color: #faf5ff;
  }

  .xl\:bg-purple-200 {
    background-color: #e9d8fd;
  }

  .xl\:bg-purple-300 {
    background-color: #d6bcfa;
  }

  .xl\:bg-purple-400 {
    background-color: #b794f4;
  }

  .xl\:bg-purple-500 {
    background-color: #9f7aea;
  }

  .xl\:bg-purple-600 {
    background-color: #805ad5;
  }

  .xl\:bg-purple-700 {
    background-color: #6b46c1;
  }

  .xl\:bg-purple-800 {
    background-color: #553c9a;
  }

  .xl\:bg-purple-900 {
    background-color: #44337a;
  }

  .xl\:bg-pink-100 {
    background-color: #fff5f7;
  }

  .xl\:bg-pink-200 {
    background-color: #fed7e2;
  }

  .xl\:bg-pink-300 {
    background-color: #fbb6ce;
  }

  .xl\:bg-pink-400 {
    background-color: #f687b3;
  }

  .xl\:bg-pink-500 {
    background-color: #ed64a6;
  }

  .xl\:bg-pink-600 {
    background-color: #d53f8c;
  }

  .xl\:bg-pink-700 {
    background-color: #b83280;
  }

  .xl\:bg-pink-800 {
    background-color: #97266d;
  }

  .xl\:bg-pink-900 {
    background-color: #702459;
  }

  .xl\:hover\:bg-transparent:hover {
    background-color: transparent;
  }

  .xl\:hover\:bg-black:hover {
    background-color: #000;
  }

  .xl\:hover\:bg-white:hover {
    background-color: #fff;
  }

  .xl\:hover\:bg-gray-100:hover {
    background-color: #f7fafc;
  }

  .xl\:hover\:bg-gray-200:hover {
    background-color: #edf2f7;
  }

  .xl\:hover\:bg-gray-300:hover {
    background-color: #e2e8f0;
  }

  .xl\:hover\:bg-gray-400:hover {
    background-color: #cbd5e0;
  }

  .xl\:hover\:bg-gray-500:hover {
    background-color: #a0aec0;
  }

  .xl\:hover\:bg-gray-600:hover {
    background-color: #718096;
  }

  .xl\:hover\:bg-gray-700:hover {
    background-color: #4a5568;
  }

  .xl\:hover\:bg-gray-800:hover {
    background-color: #2d3748;
  }

  .xl\:hover\:bg-gray-900:hover {
    background-color: #1a202c;
  }

  .xl\:hover\:bg-red-100:hover {
    background-color: #fff5f5;
  }

  .xl\:hover\:bg-red-200:hover {
    background-color: #fed7d7;
  }

  .xl\:hover\:bg-red-300:hover {
    background-color: #feb2b2;
  }

  .xl\:hover\:bg-red-400:hover {
    background-color: #fc8181;
  }

  .xl\:hover\:bg-red-500:hover {
    background-color: #f56565;
  }

  .xl\:hover\:bg-red-600:hover {
    background-color: #e53e3e;
  }

  .xl\:hover\:bg-red-700:hover {
    background-color: #c53030;
  }

  .xl\:hover\:bg-red-800:hover {
    background-color: #9b2c2c;
  }

  .xl\:hover\:bg-red-900:hover {
    background-color: #742a2a;
  }

  .xl\:hover\:bg-orange-100:hover {
    background-color: #fffaf0;
  }

  .xl\:hover\:bg-orange-200:hover {
    background-color: #feebc8;
  }

  .xl\:hover\:bg-orange-300:hover {
    background-color: #fbd38d;
  }

  .xl\:hover\:bg-orange-400:hover {
    background-color: #f6ad55;
  }

  .xl\:hover\:bg-orange-500:hover {
    background-color: #ed8936;
  }

  .xl\:hover\:bg-orange-600:hover {
    background-color: #dd6b20;
  }

  .xl\:hover\:bg-orange-700:hover {
    background-color: #c05621;
  }

  .xl\:hover\:bg-orange-800:hover {
    background-color: #9c4221;
  }

  .xl\:hover\:bg-orange-900:hover {
    background-color: #7b341e;
  }

  .xl\:hover\:bg-yellow-100:hover {
    background-color: #fffff0;
  }

  .xl\:hover\:bg-yellow-200:hover {
    background-color: #fefcbf;
  }

  .xl\:hover\:bg-yellow-300:hover {
    background-color: #faf089;
  }

  .xl\:hover\:bg-yellow-400:hover {
    background-color: #f6e05e;
  }

  .xl\:hover\:bg-yellow-500:hover {
    background-color: #ecc94b;
  }

  .xl\:hover\:bg-yellow-600:hover {
    background-color: #d69e2e;
  }

  .xl\:hover\:bg-yellow-700:hover {
    background-color: #b7791f;
  }

  .xl\:hover\:bg-yellow-800:hover {
    background-color: #975a16;
  }

  .xl\:hover\:bg-yellow-900:hover {
    background-color: #744210;
  }

  .xl\:hover\:bg-green-100:hover {
    background-color: #f0fff4;
  }

  .xl\:hover\:bg-green-200:hover {
    background-color: #c6f6d5;
  }

  .xl\:hover\:bg-green-300:hover {
    background-color: #9ae6b4;
  }

  .xl\:hover\:bg-green-400:hover {
    background-color: #68d391;
  }

  .xl\:hover\:bg-green-500:hover {
    background-color: #48bb78;
  }

  .xl\:hover\:bg-green-600:hover {
    background-color: #38a169;
  }

  .xl\:hover\:bg-green-700:hover {
    background-color: #2f855a;
  }

  .xl\:hover\:bg-green-800:hover {
    background-color: #276749;
  }

  .xl\:hover\:bg-green-900:hover {
    background-color: #22543d;
  }

  .xl\:hover\:bg-teal-100:hover {
    background-color: #e6fffa;
  }

  .xl\:hover\:bg-teal-200:hover {
    background-color: #b2f5ea;
  }

  .xl\:hover\:bg-teal-300:hover {
    background-color: #81e6d9;
  }

  .xl\:hover\:bg-teal-400:hover {
    background-color: #4fd1c5;
  }

  .xl\:hover\:bg-teal-500:hover {
    background-color: #38b2ac;
  }

  .xl\:hover\:bg-teal-600:hover {
    background-color: #319795;
  }

  .xl\:hover\:bg-teal-700:hover {
    background-color: #2c7a7b;
  }

  .xl\:hover\:bg-teal-800:hover {
    background-color: #285e61;
  }

  .xl\:hover\:bg-teal-900:hover {
    background-color: #234e52;
  }

  .xl\:hover\:bg-blue-100:hover {
    background-color: #ebf8ff;
  }

  .xl\:hover\:bg-blue-200:hover {
    background-color: #bee3f8;
  }

  .xl\:hover\:bg-blue-300:hover {
    background-color: #90cdf4;
  }

  .xl\:hover\:bg-blue-400:hover {
    background-color: #63b3ed;
  }

  .xl\:hover\:bg-blue-500:hover {
    background-color: #4299e1;
  }

  .xl\:hover\:bg-blue-600:hover {
    background-color: #3182ce;
  }

  .xl\:hover\:bg-blue-700:hover {
    background-color: #2b6cb0;
  }

  .xl\:hover\:bg-blue-800:hover {
    background-color: #2c5282;
  }

  .xl\:hover\:bg-blue-900:hover {
    background-color: #2a4365;
  }

  .xl\:hover\:bg-indigo-100:hover {
    background-color: #ebf4ff;
  }

  .xl\:hover\:bg-indigo-200:hover {
    background-color: #c3dafe;
  }

  .xl\:hover\:bg-indigo-300:hover {
    background-color: #a3bffa;
  }

  .xl\:hover\:bg-indigo-400:hover {
    background-color: #7f9cf5;
  }

  .xl\:hover\:bg-indigo-500:hover {
    background-color: #667eea;
  }

  .xl\:hover\:bg-indigo-600:hover {
    background-color: #5a67d8;
  }

  .xl\:hover\:bg-indigo-700:hover {
    background-color: #4c51bf;
  }

  .xl\:hover\:bg-indigo-800:hover {
    background-color: #434190;
  }

  .xl\:hover\:bg-indigo-900:hover {
    background-color: #3c366b;
  }

  .xl\:hover\:bg-purple-100:hover {
    background-color: #faf5ff;
  }

  .xl\:hover\:bg-purple-200:hover {
    background-color: #e9d8fd;
  }

  .xl\:hover\:bg-purple-300:hover {
    background-color: #d6bcfa;
  }

  .xl\:hover\:bg-purple-400:hover {
    background-color: #b794f4;
  }

  .xl\:hover\:bg-purple-500:hover {
    background-color: #9f7aea;
  }

  .xl\:hover\:bg-purple-600:hover {
    background-color: #805ad5;
  }

  .xl\:hover\:bg-purple-700:hover {
    background-color: #6b46c1;
  }

  .xl\:hover\:bg-purple-800:hover {
    background-color: #553c9a;
  }

  .xl\:hover\:bg-purple-900:hover {
    background-color: #44337a;
  }

  .xl\:hover\:bg-pink-100:hover {
    background-color: #fff5f7;
  }

  .xl\:hover\:bg-pink-200:hover {
    background-color: #fed7e2;
  }

  .xl\:hover\:bg-pink-300:hover {
    background-color: #fbb6ce;
  }

  .xl\:hover\:bg-pink-400:hover {
    background-color: #f687b3;
  }

  .xl\:hover\:bg-pink-500:hover {
    background-color: #ed64a6;
  }

  .xl\:hover\:bg-pink-600:hover {
    background-color: #d53f8c;
  }

  .xl\:hover\:bg-pink-700:hover {
    background-color: #b83280;
  }

  .xl\:hover\:bg-pink-800:hover {
    background-color: #97266d;
  }

  .xl\:hover\:bg-pink-900:hover {
    background-color: #702459;
  }

  .xl\:focus\:bg-transparent:focus {
    background-color: transparent;
  }

  .xl\:focus\:bg-black:focus {
    background-color: #000;
  }

  .xl\:focus\:bg-white:focus {
    background-color: #fff;
  }

  .xl\:focus\:bg-gray-100:focus {
    background-color: #f7fafc;
  }

  .xl\:focus\:bg-gray-200:focus {
    background-color: #edf2f7;
  }

  .xl\:focus\:bg-gray-300:focus {
    background-color: #e2e8f0;
  }

  .xl\:focus\:bg-gray-400:focus {
    background-color: #cbd5e0;
  }

  .xl\:focus\:bg-gray-500:focus {
    background-color: #a0aec0;
  }

  .xl\:focus\:bg-gray-600:focus {
    background-color: #718096;
  }

  .xl\:focus\:bg-gray-700:focus {
    background-color: #4a5568;
  }

  .xl\:focus\:bg-gray-800:focus {
    background-color: #2d3748;
  }

  .xl\:focus\:bg-gray-900:focus {
    background-color: #1a202c;
  }

  .xl\:focus\:bg-red-100:focus {
    background-color: #fff5f5;
  }

  .xl\:focus\:bg-red-200:focus {
    background-color: #fed7d7;
  }

  .xl\:focus\:bg-red-300:focus {
    background-color: #feb2b2;
  }

  .xl\:focus\:bg-red-400:focus {
    background-color: #fc8181;
  }

  .xl\:focus\:bg-red-500:focus {
    background-color: #f56565;
  }

  .xl\:focus\:bg-red-600:focus {
    background-color: #e53e3e;
  }

  .xl\:focus\:bg-red-700:focus {
    background-color: #c53030;
  }

  .xl\:focus\:bg-red-800:focus {
    background-color: #9b2c2c;
  }

  .xl\:focus\:bg-red-900:focus {
    background-color: #742a2a;
  }

  .xl\:focus\:bg-orange-100:focus {
    background-color: #fffaf0;
  }

  .xl\:focus\:bg-orange-200:focus {
    background-color: #feebc8;
  }

  .xl\:focus\:bg-orange-300:focus {
    background-color: #fbd38d;
  }

  .xl\:focus\:bg-orange-400:focus {
    background-color: #f6ad55;
  }

  .xl\:focus\:bg-orange-500:focus {
    background-color: #ed8936;
  }

  .xl\:focus\:bg-orange-600:focus {
    background-color: #dd6b20;
  }

  .xl\:focus\:bg-orange-700:focus {
    background-color: #c05621;
  }

  .xl\:focus\:bg-orange-800:focus {
    background-color: #9c4221;
  }

  .xl\:focus\:bg-orange-900:focus {
    background-color: #7b341e;
  }

  .xl\:focus\:bg-yellow-100:focus {
    background-color: #fffff0;
  }

  .xl\:focus\:bg-yellow-200:focus {
    background-color: #fefcbf;
  }

  .xl\:focus\:bg-yellow-300:focus {
    background-color: #faf089;
  }

  .xl\:focus\:bg-yellow-400:focus {
    background-color: #f6e05e;
  }

  .xl\:focus\:bg-yellow-500:focus {
    background-color: #ecc94b;
  }

  .xl\:focus\:bg-yellow-600:focus {
    background-color: #d69e2e;
  }

  .xl\:focus\:bg-yellow-700:focus {
    background-color: #b7791f;
  }

  .xl\:focus\:bg-yellow-800:focus {
    background-color: #975a16;
  }

  .xl\:focus\:bg-yellow-900:focus {
    background-color: #744210;
  }

  .xl\:focus\:bg-green-100:focus {
    background-color: #f0fff4;
  }

  .xl\:focus\:bg-green-200:focus {
    background-color: #c6f6d5;
  }

  .xl\:focus\:bg-green-300:focus {
    background-color: #9ae6b4;
  }

  .xl\:focus\:bg-green-400:focus {
    background-color: #68d391;
  }

  .xl\:focus\:bg-green-500:focus {
    background-color: #48bb78;
  }

  .xl\:focus\:bg-green-600:focus {
    background-color: #38a169;
  }

  .xl\:focus\:bg-green-700:focus {
    background-color: #2f855a;
  }

  .xl\:focus\:bg-green-800:focus {
    background-color: #276749;
  }

  .xl\:focus\:bg-green-900:focus {
    background-color: #22543d;
  }

  .xl\:focus\:bg-teal-100:focus {
    background-color: #e6fffa;
  }

  .xl\:focus\:bg-teal-200:focus {
    background-color: #b2f5ea;
  }

  .xl\:focus\:bg-teal-300:focus {
    background-color: #81e6d9;
  }

  .xl\:focus\:bg-teal-400:focus {
    background-color: #4fd1c5;
  }

  .xl\:focus\:bg-teal-500:focus {
    background-color: #38b2ac;
  }

  .xl\:focus\:bg-teal-600:focus {
    background-color: #319795;
  }

  .xl\:focus\:bg-teal-700:focus {
    background-color: #2c7a7b;
  }

  .xl\:focus\:bg-teal-800:focus {
    background-color: #285e61;
  }

  .xl\:focus\:bg-teal-900:focus {
    background-color: #234e52;
  }

  .xl\:focus\:bg-blue-100:focus {
    background-color: #ebf8ff;
  }

  .xl\:focus\:bg-blue-200:focus {
    background-color: #bee3f8;
  }

  .xl\:focus\:bg-blue-300:focus {
    background-color: #90cdf4;
  }

  .xl\:focus\:bg-blue-400:focus {
    background-color: #63b3ed;
  }

  .xl\:focus\:bg-blue-500:focus {
    background-color: #4299e1;
  }

  .xl\:focus\:bg-blue-600:focus {
    background-color: #3182ce;
  }

  .xl\:focus\:bg-blue-700:focus {
    background-color: #2b6cb0;
  }

  .xl\:focus\:bg-blue-800:focus {
    background-color: #2c5282;
  }

  .xl\:focus\:bg-blue-900:focus {
    background-color: #2a4365;
  }

  .xl\:focus\:bg-indigo-100:focus {
    background-color: #ebf4ff;
  }

  .xl\:focus\:bg-indigo-200:focus {
    background-color: #c3dafe;
  }

  .xl\:focus\:bg-indigo-300:focus {
    background-color: #a3bffa;
  }

  .xl\:focus\:bg-indigo-400:focus {
    background-color: #7f9cf5;
  }

  .xl\:focus\:bg-indigo-500:focus {
    background-color: #667eea;
  }

  .xl\:focus\:bg-indigo-600:focus {
    background-color: #5a67d8;
  }

  .xl\:focus\:bg-indigo-700:focus {
    background-color: #4c51bf;
  }

  .xl\:focus\:bg-indigo-800:focus {
    background-color: #434190;
  }

  .xl\:focus\:bg-indigo-900:focus {
    background-color: #3c366b;
  }

  .xl\:focus\:bg-purple-100:focus {
    background-color: #faf5ff;
  }

  .xl\:focus\:bg-purple-200:focus {
    background-color: #e9d8fd;
  }

  .xl\:focus\:bg-purple-300:focus {
    background-color: #d6bcfa;
  }

  .xl\:focus\:bg-purple-400:focus {
    background-color: #b794f4;
  }

  .xl\:focus\:bg-purple-500:focus {
    background-color: #9f7aea;
  }

  .xl\:focus\:bg-purple-600:focus {
    background-color: #805ad5;
  }

  .xl\:focus\:bg-purple-700:focus {
    background-color: #6b46c1;
  }

  .xl\:focus\:bg-purple-800:focus {
    background-color: #553c9a;
  }

  .xl\:focus\:bg-purple-900:focus {
    background-color: #44337a;
  }

  .xl\:focus\:bg-pink-100:focus {
    background-color: #fff5f7;
  }

  .xl\:focus\:bg-pink-200:focus {
    background-color: #fed7e2;
  }

  .xl\:focus\:bg-pink-300:focus {
    background-color: #fbb6ce;
  }

  .xl\:focus\:bg-pink-400:focus {
    background-color: #f687b3;
  }

  .xl\:focus\:bg-pink-500:focus {
    background-color: #ed64a6;
  }

  .xl\:focus\:bg-pink-600:focus {
    background-color: #d53f8c;
  }

  .xl\:focus\:bg-pink-700:focus {
    background-color: #b83280;
  }

  .xl\:focus\:bg-pink-800:focus {
    background-color: #97266d;
  }

  .xl\:focus\:bg-pink-900:focus {
    background-color: #702459;
  }

  .xl\:bg-bottom {
    background-position: bottom;
  }

  .xl\:bg-center {
    background-position: center;
  }

  .xl\:bg-left {
    background-position: left;
  }

  .xl\:bg-left-bottom {
    background-position: left bottom;
  }

  .xl\:bg-left-top {
    background-position: left top;
  }

  .xl\:bg-right {
    background-position: right;
  }

  .xl\:bg-right-bottom {
    background-position: right bottom;
  }

  .xl\:bg-right-top {
    background-position: right top;
  }

  .xl\:bg-top {
    background-position: top;
  }

  .xl\:bg-repeat {
    background-repeat: repeat;
  }

  .xl\:bg-no-repeat {
    background-repeat: no-repeat;
  }

  .xl\:bg-repeat-x {
    background-repeat: repeat-x;
  }

  .xl\:bg-repeat-y {
    background-repeat: repeat-y;
  }

  .xl\:bg-repeat-round {
    background-repeat: round;
  }

  .xl\:bg-repeat-space {
    background-repeat: space;
  }

  .xl\:bg-auto {
    background-size: auto;
  }

  .xl\:bg-cover {
    background-size: cover;
  }

  .xl\:bg-contain {
    background-size: contain;
  }

  .xl\:border-collapse {
    border-collapse: collapse;
  }

  .xl\:border-separate {
    border-collapse: separate;
  }

  .xl\:border-transparent {
    border-color: transparent;
  }

  .xl\:border-black {
    border-color: #000;
  }

  .xl\:border-white {
    border-color: #fff;
  }

  .xl\:border-gray-100 {
    border-color: #f7fafc;
  }

  .xl\:border-gray-200 {
    border-color: #edf2f7;
  }

  .xl\:border-gray-300 {
    border-color: #e2e8f0;
  }

  .xl\:border-gray-400 {
    border-color: #cbd5e0;
  }

  .xl\:border-gray-500 {
    border-color: #a0aec0;
  }

  .xl\:border-gray-600 {
    border-color: #718096;
  }

  .xl\:border-gray-700 {
    border-color: #4a5568;
  }

  .xl\:border-gray-800 {
    border-color: #2d3748;
  }

  .xl\:border-gray-900 {
    border-color: #1a202c;
  }

  .xl\:border-red-100 {
    border-color: #fff5f5;
  }

  .xl\:border-red-200 {
    border-color: #fed7d7;
  }

  .xl\:border-red-300 {
    border-color: #feb2b2;
  }

  .xl\:border-red-400 {
    border-color: #fc8181;
  }

  .xl\:border-red-500 {
    border-color: #f56565;
  }

  .xl\:border-red-600 {
    border-color: #e53e3e;
  }

  .xl\:border-red-700 {
    border-color: #c53030;
  }

  .xl\:border-red-800 {
    border-color: #9b2c2c;
  }

  .xl\:border-red-900 {
    border-color: #742a2a;
  }

  .xl\:border-orange-100 {
    border-color: #fffaf0;
  }

  .xl\:border-orange-200 {
    border-color: #feebc8;
  }

  .xl\:border-orange-300 {
    border-color: #fbd38d;
  }

  .xl\:border-orange-400 {
    border-color: #f6ad55;
  }

  .xl\:border-orange-500 {
    border-color: #ed8936;
  }

  .xl\:border-orange-600 {
    border-color: #dd6b20;
  }

  .xl\:border-orange-700 {
    border-color: #c05621;
  }

  .xl\:border-orange-800 {
    border-color: #9c4221;
  }

  .xl\:border-orange-900 {
    border-color: #7b341e;
  }

  .xl\:border-yellow-100 {
    border-color: #fffff0;
  }

  .xl\:border-yellow-200 {
    border-color: #fefcbf;
  }

  .xl\:border-yellow-300 {
    border-color: #faf089;
  }

  .xl\:border-yellow-400 {
    border-color: #f6e05e;
  }

  .xl\:border-yellow-500 {
    border-color: #ecc94b;
  }

  .xl\:border-yellow-600 {
    border-color: #d69e2e;
  }

  .xl\:border-yellow-700 {
    border-color: #b7791f;
  }

  .xl\:border-yellow-800 {
    border-color: #975a16;
  }

  .xl\:border-yellow-900 {
    border-color: #744210;
  }

  .xl\:border-green-100 {
    border-color: #f0fff4;
  }

  .xl\:border-green-200 {
    border-color: #c6f6d5;
  }

  .xl\:border-green-300 {
    border-color: #9ae6b4;
  }

  .xl\:border-green-400 {
    border-color: #68d391;
  }

  .xl\:border-green-500 {
    border-color: #48bb78;
  }

  .xl\:border-green-600 {
    border-color: #38a169;
  }

  .xl\:border-green-700 {
    border-color: #2f855a;
  }

  .xl\:border-green-800 {
    border-color: #276749;
  }

  .xl\:border-green-900 {
    border-color: #22543d;
  }

  .xl\:border-teal-100 {
    border-color: #e6fffa;
  }

  .xl\:border-teal-200 {
    border-color: #b2f5ea;
  }

  .xl\:border-teal-300 {
    border-color: #81e6d9;
  }

  .xl\:border-teal-400 {
    border-color: #4fd1c5;
  }

  .xl\:border-teal-500 {
    border-color: #38b2ac;
  }

  .xl\:border-teal-600 {
    border-color: #319795;
  }

  .xl\:border-teal-700 {
    border-color: #2c7a7b;
  }

  .xl\:border-teal-800 {
    border-color: #285e61;
  }

  .xl\:border-teal-900 {
    border-color: #234e52;
  }

  .xl\:border-blue-100 {
    border-color: #ebf8ff;
  }

  .xl\:border-blue-200 {
    border-color: #bee3f8;
  }

  .xl\:border-blue-300 {
    border-color: #90cdf4;
  }

  .xl\:border-blue-400 {
    border-color: #63b3ed;
  }

  .xl\:border-blue-500 {
    border-color: #4299e1;
  }

  .xl\:border-blue-600 {
    border-color: #3182ce;
  }

  .xl\:border-blue-700 {
    border-color: #2b6cb0;
  }

  .xl\:border-blue-800 {
    border-color: #2c5282;
  }

  .xl\:border-blue-900 {
    border-color: #2a4365;
  }

  .xl\:border-indigo-100 {
    border-color: #ebf4ff;
  }

  .xl\:border-indigo-200 {
    border-color: #c3dafe;
  }

  .xl\:border-indigo-300 {
    border-color: #a3bffa;
  }

  .xl\:border-indigo-400 {
    border-color: #7f9cf5;
  }

  .xl\:border-indigo-500 {
    border-color: #667eea;
  }

  .xl\:border-indigo-600 {
    border-color: #5a67d8;
  }

  .xl\:border-indigo-700 {
    border-color: #4c51bf;
  }

  .xl\:border-indigo-800 {
    border-color: #434190;
  }

  .xl\:border-indigo-900 {
    border-color: #3c366b;
  }

  .xl\:border-purple-100 {
    border-color: #faf5ff;
  }

  .xl\:border-purple-200 {
    border-color: #e9d8fd;
  }

  .xl\:border-purple-300 {
    border-color: #d6bcfa;
  }

  .xl\:border-purple-400 {
    border-color: #b794f4;
  }

  .xl\:border-purple-500 {
    border-color: #9f7aea;
  }

  .xl\:border-purple-600 {
    border-color: #805ad5;
  }

  .xl\:border-purple-700 {
    border-color: #6b46c1;
  }

  .xl\:border-purple-800 {
    border-color: #553c9a;
  }

  .xl\:border-purple-900 {
    border-color: #44337a;
  }

  .xl\:border-pink-100 {
    border-color: #fff5f7;
  }

  .xl\:border-pink-200 {
    border-color: #fed7e2;
  }

  .xl\:border-pink-300 {
    border-color: #fbb6ce;
  }

  .xl\:border-pink-400 {
    border-color: #f687b3;
  }

  .xl\:border-pink-500 {
    border-color: #ed64a6;
  }

  .xl\:border-pink-600 {
    border-color: #d53f8c;
  }

  .xl\:border-pink-700 {
    border-color: #b83280;
  }

  .xl\:border-pink-800 {
    border-color: #97266d;
  }

  .xl\:border-pink-900 {
    border-color: #702459;
  }

  .xl\:hover\:border-transparent:hover {
    border-color: transparent;
  }

  .xl\:hover\:border-black:hover {
    border-color: #000;
  }

  .xl\:hover\:border-white:hover {
    border-color: #fff;
  }

  .xl\:hover\:border-gray-100:hover {
    border-color: #f7fafc;
  }

  .xl\:hover\:border-gray-200:hover {
    border-color: #edf2f7;
  }

  .xl\:hover\:border-gray-300:hover {
    border-color: #e2e8f0;
  }

  .xl\:hover\:border-gray-400:hover {
    border-color: #cbd5e0;
  }

  .xl\:hover\:border-gray-500:hover {
    border-color: #a0aec0;
  }

  .xl\:hover\:border-gray-600:hover {
    border-color: #718096;
  }

  .xl\:hover\:border-gray-700:hover {
    border-color: #4a5568;
  }

  .xl\:hover\:border-gray-800:hover {
    border-color: #2d3748;
  }

  .xl\:hover\:border-gray-900:hover {
    border-color: #1a202c;
  }

  .xl\:hover\:border-red-100:hover {
    border-color: #fff5f5;
  }

  .xl\:hover\:border-red-200:hover {
    border-color: #fed7d7;
  }

  .xl\:hover\:border-red-300:hover {
    border-color: #feb2b2;
  }

  .xl\:hover\:border-red-400:hover {
    border-color: #fc8181;
  }

  .xl\:hover\:border-red-500:hover {
    border-color: #f56565;
  }

  .xl\:hover\:border-red-600:hover {
    border-color: #e53e3e;
  }

  .xl\:hover\:border-red-700:hover {
    border-color: #c53030;
  }

  .xl\:hover\:border-red-800:hover {
    border-color: #9b2c2c;
  }

  .xl\:hover\:border-red-900:hover {
    border-color: #742a2a;
  }

  .xl\:hover\:border-orange-100:hover {
    border-color: #fffaf0;
  }

  .xl\:hover\:border-orange-200:hover {
    border-color: #feebc8;
  }

  .xl\:hover\:border-orange-300:hover {
    border-color: #fbd38d;
  }

  .xl\:hover\:border-orange-400:hover {
    border-color: #f6ad55;
  }

  .xl\:hover\:border-orange-500:hover {
    border-color: #ed8936;
  }

  .xl\:hover\:border-orange-600:hover {
    border-color: #dd6b20;
  }

  .xl\:hover\:border-orange-700:hover {
    border-color: #c05621;
  }

  .xl\:hover\:border-orange-800:hover {
    border-color: #9c4221;
  }

  .xl\:hover\:border-orange-900:hover {
    border-color: #7b341e;
  }

  .xl\:hover\:border-yellow-100:hover {
    border-color: #fffff0;
  }

  .xl\:hover\:border-yellow-200:hover {
    border-color: #fefcbf;
  }

  .xl\:hover\:border-yellow-300:hover {
    border-color: #faf089;
  }

  .xl\:hover\:border-yellow-400:hover {
    border-color: #f6e05e;
  }

  .xl\:hover\:border-yellow-500:hover {
    border-color: #ecc94b;
  }

  .xl\:hover\:border-yellow-600:hover {
    border-color: #d69e2e;
  }

  .xl\:hover\:border-yellow-700:hover {
    border-color: #b7791f;
  }

  .xl\:hover\:border-yellow-800:hover {
    border-color: #975a16;
  }

  .xl\:hover\:border-yellow-900:hover {
    border-color: #744210;
  }

  .xl\:hover\:border-green-100:hover {
    border-color: #f0fff4;
  }

  .xl\:hover\:border-green-200:hover {
    border-color: #c6f6d5;
  }

  .xl\:hover\:border-green-300:hover {
    border-color: #9ae6b4;
  }

  .xl\:hover\:border-green-400:hover {
    border-color: #68d391;
  }

  .xl\:hover\:border-green-500:hover {
    border-color: #48bb78;
  }

  .xl\:hover\:border-green-600:hover {
    border-color: #38a169;
  }

  .xl\:hover\:border-green-700:hover {
    border-color: #2f855a;
  }

  .xl\:hover\:border-green-800:hover {
    border-color: #276749;
  }

  .xl\:hover\:border-green-900:hover {
    border-color: #22543d;
  }

  .xl\:hover\:border-teal-100:hover {
    border-color: #e6fffa;
  }

  .xl\:hover\:border-teal-200:hover {
    border-color: #b2f5ea;
  }

  .xl\:hover\:border-teal-300:hover {
    border-color: #81e6d9;
  }

  .xl\:hover\:border-teal-400:hover {
    border-color: #4fd1c5;
  }

  .xl\:hover\:border-teal-500:hover {
    border-color: #38b2ac;
  }

  .xl\:hover\:border-teal-600:hover {
    border-color: #319795;
  }

  .xl\:hover\:border-teal-700:hover {
    border-color: #2c7a7b;
  }

  .xl\:hover\:border-teal-800:hover {
    border-color: #285e61;
  }

  .xl\:hover\:border-teal-900:hover {
    border-color: #234e52;
  }

  .xl\:hover\:border-blue-100:hover {
    border-color: #ebf8ff;
  }

  .xl\:hover\:border-blue-200:hover {
    border-color: #bee3f8;
  }

  .xl\:hover\:border-blue-300:hover {
    border-color: #90cdf4;
  }

  .xl\:hover\:border-blue-400:hover {
    border-color: #63b3ed;
  }

  .xl\:hover\:border-blue-500:hover {
    border-color: #4299e1;
  }

  .xl\:hover\:border-blue-600:hover {
    border-color: #3182ce;
  }

  .xl\:hover\:border-blue-700:hover {
    border-color: #2b6cb0;
  }

  .xl\:hover\:border-blue-800:hover {
    border-color: #2c5282;
  }

  .xl\:hover\:border-blue-900:hover {
    border-color: #2a4365;
  }

  .xl\:hover\:border-indigo-100:hover {
    border-color: #ebf4ff;
  }

  .xl\:hover\:border-indigo-200:hover {
    border-color: #c3dafe;
  }

  .xl\:hover\:border-indigo-300:hover {
    border-color: #a3bffa;
  }

  .xl\:hover\:border-indigo-400:hover {
    border-color: #7f9cf5;
  }

  .xl\:hover\:border-indigo-500:hover {
    border-color: #667eea;
  }

  .xl\:hover\:border-indigo-600:hover {
    border-color: #5a67d8;
  }

  .xl\:hover\:border-indigo-700:hover {
    border-color: #4c51bf;
  }

  .xl\:hover\:border-indigo-800:hover {
    border-color: #434190;
  }

  .xl\:hover\:border-indigo-900:hover {
    border-color: #3c366b;
  }

  .xl\:hover\:border-purple-100:hover {
    border-color: #faf5ff;
  }

  .xl\:hover\:border-purple-200:hover {
    border-color: #e9d8fd;
  }

  .xl\:hover\:border-purple-300:hover {
    border-color: #d6bcfa;
  }

  .xl\:hover\:border-purple-400:hover {
    border-color: #b794f4;
  }

  .xl\:hover\:border-purple-500:hover {
    border-color: #9f7aea;
  }

  .xl\:hover\:border-purple-600:hover {
    border-color: #805ad5;
  }

  .xl\:hover\:border-purple-700:hover {
    border-color: #6b46c1;
  }

  .xl\:hover\:border-purple-800:hover {
    border-color: #553c9a;
  }

  .xl\:hover\:border-purple-900:hover {
    border-color: #44337a;
  }

  .xl\:hover\:border-pink-100:hover {
    border-color: #fff5f7;
  }

  .xl\:hover\:border-pink-200:hover {
    border-color: #fed7e2;
  }

  .xl\:hover\:border-pink-300:hover {
    border-color: #fbb6ce;
  }

  .xl\:hover\:border-pink-400:hover {
    border-color: #f687b3;
  }

  .xl\:hover\:border-pink-500:hover {
    border-color: #ed64a6;
  }

  .xl\:hover\:border-pink-600:hover {
    border-color: #d53f8c;
  }

  .xl\:hover\:border-pink-700:hover {
    border-color: #b83280;
  }

  .xl\:hover\:border-pink-800:hover {
    border-color: #97266d;
  }

  .xl\:hover\:border-pink-900:hover {
    border-color: #702459;
  }

  .xl\:focus\:border-transparent:focus {
    border-color: transparent;
  }

  .xl\:focus\:border-black:focus {
    border-color: #000;
  }

  .xl\:focus\:border-white:focus {
    border-color: #fff;
  }

  .xl\:focus\:border-gray-100:focus {
    border-color: #f7fafc;
  }

  .xl\:focus\:border-gray-200:focus {
    border-color: #edf2f7;
  }

  .xl\:focus\:border-gray-300:focus {
    border-color: #e2e8f0;
  }

  .xl\:focus\:border-gray-400:focus {
    border-color: #cbd5e0;
  }

  .xl\:focus\:border-gray-500:focus {
    border-color: #a0aec0;
  }

  .xl\:focus\:border-gray-600:focus {
    border-color: #718096;
  }

  .xl\:focus\:border-gray-700:focus {
    border-color: #4a5568;
  }

  .xl\:focus\:border-gray-800:focus {
    border-color: #2d3748;
  }

  .xl\:focus\:border-gray-900:focus {
    border-color: #1a202c;
  }

  .xl\:focus\:border-red-100:focus {
    border-color: #fff5f5;
  }

  .xl\:focus\:border-red-200:focus {
    border-color: #fed7d7;
  }

  .xl\:focus\:border-red-300:focus {
    border-color: #feb2b2;
  }

  .xl\:focus\:border-red-400:focus {
    border-color: #fc8181;
  }

  .xl\:focus\:border-red-500:focus {
    border-color: #f56565;
  }

  .xl\:focus\:border-red-600:focus {
    border-color: #e53e3e;
  }

  .xl\:focus\:border-red-700:focus {
    border-color: #c53030;
  }

  .xl\:focus\:border-red-800:focus {
    border-color: #9b2c2c;
  }

  .xl\:focus\:border-red-900:focus {
    border-color: #742a2a;
  }

  .xl\:focus\:border-orange-100:focus {
    border-color: #fffaf0;
  }

  .xl\:focus\:border-orange-200:focus {
    border-color: #feebc8;
  }

  .xl\:focus\:border-orange-300:focus {
    border-color: #fbd38d;
  }

  .xl\:focus\:border-orange-400:focus {
    border-color: #f6ad55;
  }

  .xl\:focus\:border-orange-500:focus {
    border-color: #ed8936;
  }

  .xl\:focus\:border-orange-600:focus {
    border-color: #dd6b20;
  }

  .xl\:focus\:border-orange-700:focus {
    border-color: #c05621;
  }

  .xl\:focus\:border-orange-800:focus {
    border-color: #9c4221;
  }

  .xl\:focus\:border-orange-900:focus {
    border-color: #7b341e;
  }

  .xl\:focus\:border-yellow-100:focus {
    border-color: #fffff0;
  }

  .xl\:focus\:border-yellow-200:focus {
    border-color: #fefcbf;
  }

  .xl\:focus\:border-yellow-300:focus {
    border-color: #faf089;
  }

  .xl\:focus\:border-yellow-400:focus {
    border-color: #f6e05e;
  }

  .xl\:focus\:border-yellow-500:focus {
    border-color: #ecc94b;
  }

  .xl\:focus\:border-yellow-600:focus {
    border-color: #d69e2e;
  }

  .xl\:focus\:border-yellow-700:focus {
    border-color: #b7791f;
  }

  .xl\:focus\:border-yellow-800:focus {
    border-color: #975a16;
  }

  .xl\:focus\:border-yellow-900:focus {
    border-color: #744210;
  }

  .xl\:focus\:border-green-100:focus {
    border-color: #f0fff4;
  }

  .xl\:focus\:border-green-200:focus {
    border-color: #c6f6d5;
  }

  .xl\:focus\:border-green-300:focus {
    border-color: #9ae6b4;
  }

  .xl\:focus\:border-green-400:focus {
    border-color: #68d391;
  }

  .xl\:focus\:border-green-500:focus {
    border-color: #48bb78;
  }

  .xl\:focus\:border-green-600:focus {
    border-color: #38a169;
  }

  .xl\:focus\:border-green-700:focus {
    border-color: #2f855a;
  }

  .xl\:focus\:border-green-800:focus {
    border-color: #276749;
  }

  .xl\:focus\:border-green-900:focus {
    border-color: #22543d;
  }

  .xl\:focus\:border-teal-100:focus {
    border-color: #e6fffa;
  }

  .xl\:focus\:border-teal-200:focus {
    border-color: #b2f5ea;
  }

  .xl\:focus\:border-teal-300:focus {
    border-color: #81e6d9;
  }

  .xl\:focus\:border-teal-400:focus {
    border-color: #4fd1c5;
  }

  .xl\:focus\:border-teal-500:focus {
    border-color: #38b2ac;
  }

  .xl\:focus\:border-teal-600:focus {
    border-color: #319795;
  }

  .xl\:focus\:border-teal-700:focus {
    border-color: #2c7a7b;
  }

  .xl\:focus\:border-teal-800:focus {
    border-color: #285e61;
  }

  .xl\:focus\:border-teal-900:focus {
    border-color: #234e52;
  }

  .xl\:focus\:border-blue-100:focus {
    border-color: #ebf8ff;
  }

  .xl\:focus\:border-blue-200:focus {
    border-color: #bee3f8;
  }

  .xl\:focus\:border-blue-300:focus {
    border-color: #90cdf4;
  }

  .xl\:focus\:border-blue-400:focus {
    border-color: #63b3ed;
  }

  .xl\:focus\:border-blue-500:focus {
    border-color: #4299e1;
  }

  .xl\:focus\:border-blue-600:focus {
    border-color: #3182ce;
  }

  .xl\:focus\:border-blue-700:focus {
    border-color: #2b6cb0;
  }

  .xl\:focus\:border-blue-800:focus {
    border-color: #2c5282;
  }

  .xl\:focus\:border-blue-900:focus {
    border-color: #2a4365;
  }

  .xl\:focus\:border-indigo-100:focus {
    border-color: #ebf4ff;
  }

  .xl\:focus\:border-indigo-200:focus {
    border-color: #c3dafe;
  }

  .xl\:focus\:border-indigo-300:focus {
    border-color: #a3bffa;
  }

  .xl\:focus\:border-indigo-400:focus {
    border-color: #7f9cf5;
  }

  .xl\:focus\:border-indigo-500:focus {
    border-color: #667eea;
  }

  .xl\:focus\:border-indigo-600:focus {
    border-color: #5a67d8;
  }

  .xl\:focus\:border-indigo-700:focus {
    border-color: #4c51bf;
  }

  .xl\:focus\:border-indigo-800:focus {
    border-color: #434190;
  }

  .xl\:focus\:border-indigo-900:focus {
    border-color: #3c366b;
  }

  .xl\:focus\:border-purple-100:focus {
    border-color: #faf5ff;
  }

  .xl\:focus\:border-purple-200:focus {
    border-color: #e9d8fd;
  }

  .xl\:focus\:border-purple-300:focus {
    border-color: #d6bcfa;
  }

  .xl\:focus\:border-purple-400:focus {
    border-color: #b794f4;
  }

  .xl\:focus\:border-purple-500:focus {
    border-color: #9f7aea;
  }

  .xl\:focus\:border-purple-600:focus {
    border-color: #805ad5;
  }

  .xl\:focus\:border-purple-700:focus {
    border-color: #6b46c1;
  }

  .xl\:focus\:border-purple-800:focus {
    border-color: #553c9a;
  }

  .xl\:focus\:border-purple-900:focus {
    border-color: #44337a;
  }

  .xl\:focus\:border-pink-100:focus {
    border-color: #fff5f7;
  }

  .xl\:focus\:border-pink-200:focus {
    border-color: #fed7e2;
  }

  .xl\:focus\:border-pink-300:focus {
    border-color: #fbb6ce;
  }

  .xl\:focus\:border-pink-400:focus {
    border-color: #f687b3;
  }

  .xl\:focus\:border-pink-500:focus {
    border-color: #ed64a6;
  }

  .xl\:focus\:border-pink-600:focus {
    border-color: #d53f8c;
  }

  .xl\:focus\:border-pink-700:focus {
    border-color: #b83280;
  }

  .xl\:focus\:border-pink-800:focus {
    border-color: #97266d;
  }

  .xl\:focus\:border-pink-900:focus {
    border-color: #702459;
  }

  .xl\:rounded-none {
    border-radius: 0;
  }

  .xl\:rounded-sm {
    border-radius: 0.125rem;
  }

  .xl\:rounded {
    border-radius: 0.25rem;
  }

  .xl\:rounded-lg {
    border-radius: 0.5rem;
  }

  .xl\:rounded-full {
    border-radius: 9999px;
  }

  .xl\:rounded-t-none {
    border-top-left-radius: 0;
    border-top-right-radius: 0;
  }

  .xl\:rounded-r-none {
    border-top-right-radius: 0;
    border-bottom-right-radius: 0;
  }

  .xl\:rounded-b-none {
    border-bottom-right-radius: 0;
    border-bottom-left-radius: 0;
  }

  .xl\:rounded-l-none {
    border-top-left-radius: 0;
    border-bottom-left-radius: 0;
  }

  .xl\:rounded-t-sm {
    border-top-left-radius: 0.125rem;
    border-top-right-radius: 0.125rem;
  }

  .xl\:rounded-r-sm {
    border-top-right-radius: 0.125rem;
    border-bottom-right-radius: 0.125rem;
  }

  .xl\:rounded-b-sm {
    border-bottom-right-radius: 0.125rem;
    border-bottom-left-radius: 0.125rem;
  }

  .xl\:rounded-l-sm {
    border-top-left-radius: 0.125rem;
    border-bottom-left-radius: 0.125rem;
  }

  .xl\:rounded-t {
    border-top-left-radius: 0.25rem;
    border-top-right-radius: 0.25rem;
  }

  .xl\:rounded-r {
    border-top-right-radius: 0.25rem;
    border-bottom-right-radius: 0.25rem;
  }

  .xl\:rounded-b {
    border-bottom-right-radius: 0.25rem;
    border-bottom-left-radius: 0.25rem;
  }

  .xl\:rounded-l {
    border-top-left-radius: 0.25rem;
    border-bottom-left-radius: 0.25rem;
  }

  .xl\:rounded-t-lg {
    border-top-left-radius: 0.5rem;
    border-top-right-radius: 0.5rem;
  }

  .xl\:rounded-r-lg {
    border-top-right-radius: 0.5rem;
    border-bottom-right-radius: 0.5rem;
  }

  .xl\:rounded-b-lg {
    border-bottom-right-radius: 0.5rem;
    border-bottom-left-radius: 0.5rem;
  }

  .xl\:rounded-l-lg {
    border-top-left-radius: 0.5rem;
    border-bottom-left-radius: 0.5rem;
  }

  .xl\:rounded-t-full {
    border-top-left-radius: 9999px;
    border-top-right-radius: 9999px;
  }

  .xl\:rounded-r-full {
    border-top-right-radius: 9999px;
    border-bottom-right-radius: 9999px;
  }

  .xl\:rounded-b-full {
    border-bottom-right-radius: 9999px;
    border-bottom-left-radius: 9999px;
  }

  .xl\:rounded-l-full {
    border-top-left-radius: 9999px;
    border-bottom-left-radius: 9999px;
  }

  .xl\:rounded-tl-none {
    border-top-left-radius: 0;
  }

  .xl\:rounded-tr-none {
    border-top-right-radius: 0;
  }

  .xl\:rounded-br-none {
    border-bottom-right-radius: 0;
  }

  .xl\:rounded-bl-none {
    border-bottom-left-radius: 0;
  }

  .xl\:rounded-tl-sm {
    border-top-left-radius: 0.125rem;
  }

  .xl\:rounded-tr-sm {
    border-top-right-radius: 0.125rem;
  }

  .xl\:rounded-br-sm {
    border-bottom-right-radius: 0.125rem;
  }

  .xl\:rounded-bl-sm {
    border-bottom-left-radius: 0.125rem;
  }

  .xl\:rounded-tl {
    border-top-left-radius: 0.25rem;
  }

  .xl\:rounded-tr {
    border-top-right-radius: 0.25rem;
  }

  .xl\:rounded-br {
    border-bottom-right-radius: 0.25rem;
  }

  .xl\:rounded-bl {
    border-bottom-left-radius: 0.25rem;
  }

  .xl\:rounded-tl-lg {
    border-top-left-radius: 0.5rem;
  }

  .xl\:rounded-tr-lg {
    border-top-right-radius: 0.5rem;
  }

  .xl\:rounded-br-lg {
    border-bottom-right-radius: 0.5rem;
  }

  .xl\:rounded-bl-lg {
    border-bottom-left-radius: 0.5rem;
  }

  .xl\:rounded-tl-full {
    border-top-left-radius: 9999px;
  }

  .xl\:rounded-tr-full {
    border-top-right-radius: 9999px;
  }

  .xl\:rounded-br-full {
    border-bottom-right-radius: 9999px;
  }

  .xl\:rounded-bl-full {
    border-bottom-left-radius: 9999px;
  }

  .xl\:border-solid {
    border-style: solid;
  }

  .xl\:border-dashed {
    border-style: dashed;
  }

  .xl\:border-dotted {
    border-style: dotted;
  }

  .xl\:border-double {
    border-style: double;
  }

  .xl\:border-none {
    border-style: none;
  }

  .xl\:border-0 {
    border-width: 0;
  }

  .xl\:border-2 {
    border-width: 2px;
  }

  .xl\:border-4 {
    border-width: 4px;
  }

  .xl\:border-8 {
    border-width: 8px;
  }

  .xl\:border {
    border-width: 1px;
  }

  .xl\:border-t-0 {
    border-top-width: 0;
  }

  .xl\:border-r-0 {
    border-right-width: 0;
  }

  .xl\:border-b-0 {
    border-bottom-width: 0;
  }

  .xl\:border-l-0 {
    border-left-width: 0;
  }

  .xl\:border-t-2 {
    border-top-width: 2px;
  }

  .xl\:border-r-2 {
    border-right-width: 2px;
  }

  .xl\:border-b-2 {
    border-bottom-width: 2px;
  }

  .xl\:border-l-2 {
    border-left-width: 2px;
  }

  .xl\:border-t-4 {
    border-top-width: 4px;
  }

  .xl\:border-r-4 {
    border-right-width: 4px;
  }

  .xl\:border-b-4 {
    border-bottom-width: 4px;
  }

  .xl\:border-l-4 {
    border-left-width: 4px;
  }

  .xl\:border-t-8 {
    border-top-width: 8px;
  }

  .xl\:border-r-8 {
    border-right-width: 8px;
  }

  .xl\:border-b-8 {
    border-bottom-width: 8px;
  }

  .xl\:border-l-8 {
    border-left-width: 8px;
  }

  .xl\:border-t {
    border-top-width: 1px;
  }

  .xl\:border-r {
    border-right-width: 1px;
  }

  .xl\:border-b {
    border-bottom-width: 1px;
  }

  .xl\:border-l {
    border-left-width: 1px;
  }

  .xl\:cursor-auto {
    cursor: auto;
  }

  .xl\:cursor-default {
    cursor: default;
  }

  .xl\:cursor-pointer {
    cursor: pointer;
  }

  .xl\:cursor-wait {
    cursor: wait;
  }

  .xl\:cursor-text {
    cursor: text;
  }

  .xl\:cursor-move {
    cursor: move;
  }

  .xl\:cursor-not-allowed {
    cursor: not-allowed;
  }

  .xl\:block {
    display: block;
  }

  .xl\:inline-block {
    display: inline-block;
  }

  .xl\:inline {
    display: inline;
  }

  .xl\:flex {
    display: -webkit-box;
    display: flex;
  }

  .xl\:inline-flex {
    display: -webkit-inline-box;
    display: inline-flex;
  }

  .xl\:table {
    display: table;
  }

  .xl\:table-row {
    display: table-row;
  }

  .xl\:table-cell {
    display: table-cell;
  }

  .xl\:hidden {
    display: none;
  }

  .xl\:flex-row {
    -webkit-box-orient: horizontal;
    -webkit-box-direction: normal;
            flex-direction: row;
  }

  .xl\:flex-row-reverse {
    -webkit-box-orient: horizontal;
    -webkit-box-direction: reverse;
            flex-direction: row-reverse;
  }

  .xl\:flex-col {
    -webkit-box-orient: vertical;
    -webkit-box-direction: normal;
            flex-direction: column;
  }

  .xl\:flex-col-reverse {
    -webkit-box-orient: vertical;
    -webkit-box-direction: reverse;
            flex-direction: column-reverse;
  }

  .xl\:flex-wrap {
    flex-wrap: wrap;
  }

  .xl\:flex-wrap-reverse {
    flex-wrap: wrap-reverse;
  }

  .xl\:flex-no-wrap {
    flex-wrap: nowrap;
  }

  .xl\:items-start {
    -webkit-box-align: start;
            align-items: flex-start;
  }

  .xl\:items-end {
    -webkit-box-align: end;
            align-items: flex-end;
  }

  .xl\:items-center {
    -webkit-box-align: center;
            align-items: center;
  }

  .xl\:items-baseline {
    -webkit-box-align: baseline;
            align-items: baseline;
  }

  .xl\:items-stretch {
    -webkit-box-align: stretch;
            align-items: stretch;
  }

  .xl\:self-auto {
    align-self: auto;
  }

  .xl\:self-start {
    align-self: flex-start;
  }

  .xl\:self-end {
    align-self: flex-end;
  }

  .xl\:self-center {
    align-self: center;
  }

  .xl\:self-stretch {
    align-self: stretch;
  }

  .xl\:justify-start {
    -webkit-box-pack: start;
            justify-content: flex-start;
  }

  .xl\:justify-end {
    -webkit-box-pack: end;
            justify-content: flex-end;
  }

  .xl\:justify-center {
    -webkit-box-pack: center;
            justify-content: center;
  }

  .xl\:justify-between {
    -webkit-box-pack: justify;
            justify-content: space-between;
  }

  .xl\:justify-around {
    justify-content: space-around;
  }

  .xl\:content-center {
    align-content: center;
  }

  .xl\:content-start {
    align-content: flex-start;
  }

  .xl\:content-end {
    align-content: flex-end;
  }

  .xl\:content-between {
    align-content: space-between;
  }

  .xl\:content-around {
    align-content: space-around;
  }

  .xl\:flex-1 {
    -webkit-box-flex: 1;
            flex: 1 1 0%;
  }

  .xl\:flex-auto {
    -webkit-box-flex: 1;
            flex: 1 1 auto;
  }

  .xl\:flex-initial {
    -webkit-box-flex: 0;
            flex: 0 1 auto;
  }

  .xl\:flex-none {
    -webkit-box-flex: 0;
            flex: none;
  }

  .xl\:flex-grow-0 {
    -webkit-box-flex: 0;
            flex-grow: 0;
  }

  .xl\:flex-grow {
    -webkit-box-flex: 1;
            flex-grow: 1;
  }

  .xl\:flex-shrink-0 {
    flex-shrink: 0;
  }

  .xl\:flex-shrink {
    flex-shrink: 1;
  }

  .xl\:order-1 {
    -webkit-box-ordinal-group: 2;
            order: 1;
  }

  .xl\:order-2 {
    -webkit-box-ordinal-group: 3;
            order: 2;
  }

  .xl\:order-3 {
    -webkit-box-ordinal-group: 4;
            order: 3;
  }

  .xl\:order-4 {
    -webkit-box-ordinal-group: 5;
            order: 4;
  }

  .xl\:order-5 {
    -webkit-box-ordinal-group: 6;
            order: 5;
  }

  .xl\:order-6 {
    -webkit-box-ordinal-group: 7;
            order: 6;
  }

  .xl\:order-7 {
    -webkit-box-ordinal-group: 8;
            order: 7;
  }

  .xl\:order-8 {
    -webkit-box-ordinal-group: 9;
            order: 8;
  }

  .xl\:order-9 {
    -webkit-box-ordinal-group: 10;
            order: 9;
  }

  .xl\:order-10 {
    -webkit-box-ordinal-group: 11;
            order: 10;
  }

  .xl\:order-11 {
    -webkit-box-ordinal-group: 12;
            order: 11;
  }

  .xl\:order-12 {
    -webkit-box-ordinal-group: 13;
            order: 12;
  }

  .xl\:order-first {
    -webkit-box-ordinal-group: -9998;
            order: -9999;
  }

  .xl\:order-last {
    -webkit-box-ordinal-group: 10000;
            order: 9999;
  }

  .xl\:order-none {
    -webkit-box-ordinal-group: 1;
            order: 0;
  }

  .xl\:float-right {
    float: right;
  }

  .xl\:float-left {
    float: left;
  }

  .xl\:float-none {
    float: none;
  }

  .xl\:clearfix:after {
    content: "";
    display: table;
    clear: both;
  }

  .xl\:font-sans {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji";
  }

  .xl\:font-serif {
    font-family: Georgia, Cambria, "Times New Roman", Times, serif;
  }

  .xl\:font-mono {
    font-family: Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  }

  .xl\:font-hairline {
    font-weight: 100;
  }

  .xl\:font-thin {
    font-weight: 200;
  }

  .xl\:font-light {
    font-weight: 300;
  }

  .xl\:font-normal {
    font-weight: 400;
  }

  .xl\:font-medium {
    font-weight: 500;
  }

  .xl\:font-semibold {
    font-weight: 600;
  }

  .xl\:font-bold {
    font-weight: 700;
  }

  .xl\:font-extrabold {
    font-weight: 800;
  }

  .xl\:font-black {
    font-weight: 900;
  }

  .xl\:hover\:font-hairline:hover {
    font-weight: 100;
  }

  .xl\:hover\:font-thin:hover {
    font-weight: 200;
  }

  .xl\:hover\:font-light:hover {
    font-weight: 300;
  }

  .xl\:hover\:font-normal:hover {
    font-weight: 400;
  }

  .xl\:hover\:font-medium:hover {
    font-weight: 500;
  }

  .xl\:hover\:font-semibold:hover {
    font-weight: 600;
  }

  .xl\:hover\:font-bold:hover {
    font-weight: 700;
  }

  .xl\:hover\:font-extrabold:hover {
    font-weight: 800;
  }

  .xl\:hover\:font-black:hover {
    font-weight: 900;
  }

  .xl\:focus\:font-hairline:focus {
    font-weight: 100;
  }

  .xl\:focus\:font-thin:focus {
    font-weight: 200;
  }

  .xl\:focus\:font-light:focus {
    font-weight: 300;
  }

  .xl\:focus\:font-normal:focus {
    font-weight: 400;
  }

  .xl\:focus\:font-medium:focus {
    font-weight: 500;
  }

  .xl\:focus\:font-semibold:focus {
    font-weight: 600;
  }

  .xl\:focus\:font-bold:focus {
    font-weight: 700;
  }

  .xl\:focus\:font-extrabold:focus {
    font-weight: 800;
  }

  .xl\:focus\:font-black:focus {
    font-weight: 900;
  }

  .xl\:h-0 {
    height: 0;
  }

  .xl\:h-1 {
    height: 0.25rem;
  }

  .xl\:h-2 {
    height: 0.5rem;
  }

  .xl\:h-3 {
    height: 0.75rem;
  }

  .xl\:h-4 {
    height: 1rem;
  }

  .xl\:h-5 {
    height: 1.25rem;
  }

  .xl\:h-6 {
    height: 1.5rem;
  }

  .xl\:h-8 {
    height: 2rem;
  }

  .xl\:h-10 {
    height: 2.5rem;
  }

  .xl\:h-12 {
    height: 3rem;
  }

  .xl\:h-16 {
    height: 4rem;
  }

  .xl\:h-20 {
    height: 5rem;
  }

  .xl\:h-24 {
    height: 6rem;
  }

  .xl\:h-32 {
    height: 8rem;
  }

  .xl\:h-40 {
    height: 10rem;
  }

  .xl\:h-48 {
    height: 12rem;
  }

  .xl\:h-56 {
    height: 14rem;
  }

  .xl\:h-64 {
    height: 16rem;
  }

  .xl\:h-auto {
    height: auto;
  }

  .xl\:h-px {
    height: 1px;
  }

  .xl\:h-full {
    height: 100%;
  }

  .xl\:h-screen {
    height: 100vh;
  }

  .xl\:leading-none {
    line-height: 1;
  }

  .xl\:leading-tight {
    line-height: 1.25;
  }

  .xl\:leading-snug {
    line-height: 1.375;
  }

  .xl\:leading-normal {
    line-height: 1.5;
  }

  .xl\:leading-relaxed {
    line-height: 1.625;
  }

  .xl\:leading-loose {
    line-height: 2;
  }

  .xl\:list-inside {
    list-style-position: inside;
  }

  .xl\:list-outside {
    list-style-position: outside;
  }

  .xl\:list-none {
    list-style-type: none;
  }

  .xl\:list-disc {
    list-style-type: disc;
  }

  .xl\:list-decimal {
    list-style-type: decimal;
  }

  .xl\:m-0 {
    margin: 0;
  }

  .xl\:m-1 {
    margin: 0.25rem;
  }

  .xl\:m-2 {
    margin: 0.5rem;
  }

  .xl\:m-3 {
    margin: 0.75rem;
  }

  .xl\:m-4 {
    margin: 1rem;
  }

  .xl\:m-5 {
    margin: 1.25rem;
  }

  .xl\:m-6 {
    margin: 1.5rem;
  }

  .xl\:m-8 {
    margin: 2rem;
  }

  .xl\:m-10 {
    margin: 2.5rem;
  }

  .xl\:m-12 {
    margin: 3rem;
  }

  .xl\:m-16 {
    margin: 4rem;
  }

  .xl\:m-20 {
    margin: 5rem;
  }

  .xl\:m-24 {
    margin: 6rem;
  }

  .xl\:m-32 {
    margin: 8rem;
  }

  .xl\:m-40 {
    margin: 10rem;
  }

  .xl\:m-48 {
    margin: 12rem;
  }

  .xl\:m-56 {
    margin: 14rem;
  }

  .xl\:m-64 {
    margin: 16rem;
  }

  .xl\:m-auto {
    margin: auto;
  }

  .xl\:m-px {
    margin: 1px;
  }

  .xl\:-m-1 {
    margin: -0.25rem;
  }

  .xl\:-m-2 {
    margin: -0.5rem;
  }

  .xl\:-m-3 {
    margin: -0.75rem;
  }

  .xl\:-m-4 {
    margin: -1rem;
  }

  .xl\:-m-5 {
    margin: -1.25rem;
  }

  .xl\:-m-6 {
    margin: -1.5rem;
  }

  .xl\:-m-8 {
    margin: -2rem;
  }

  .xl\:-m-10 {
    margin: -2.5rem;
  }

  .xl\:-m-12 {
    margin: -3rem;
  }

  .xl\:-m-16 {
    margin: -4rem;
  }

  .xl\:-m-20 {
    margin: -5rem;
  }

  .xl\:-m-24 {
    margin: -6rem;
  }

  .xl\:-m-32 {
    margin: -8rem;
  }

  .xl\:-m-40 {
    margin: -10rem;
  }

  .xl\:-m-48 {
    margin: -12rem;
  }

  .xl\:-m-56 {
    margin: -14rem;
  }

  .xl\:-m-64 {
    margin: -16rem;
  }

  .xl\:-m-px {
    margin: -1px;
  }

  .xl\:my-0 {
    margin-top: 0;
    margin-bottom: 0;
  }

  .xl\:mx-0 {
    margin-left: 0;
    margin-right: 0;
  }

  .xl\:my-1 {
    margin-top: 0.25rem;
    margin-bottom: 0.25rem;
  }

  .xl\:mx-1 {
    margin-left: 0.25rem;
    margin-right: 0.25rem;
  }

  .xl\:my-2 {
    margin-top: 0.5rem;
    margin-bottom: 0.5rem;
  }

  .xl\:mx-2 {
    margin-left: 0.5rem;
    margin-right: 0.5rem;
  }

  .xl\:my-3 {
    margin-top: 0.75rem;
    margin-bottom: 0.75rem;
  }

  .xl\:mx-3 {
    margin-left: 0.75rem;
    margin-right: 0.75rem;
  }

  .xl\:my-4 {
    margin-top: 1rem;
    margin-bottom: 1rem;
  }

  .xl\:mx-4 {
    margin-left: 1rem;
    margin-right: 1rem;
  }

  .xl\:my-5 {
    margin-top: 1.25rem;
    margin-bottom: 1.25rem;
  }

  .xl\:mx-5 {
    margin-left: 1.25rem;
    margin-right: 1.25rem;
  }

  .xl\:my-6 {
    margin-top: 1.5rem;
    margin-bottom: 1.5rem;
  }

  .xl\:mx-6 {
    margin-left: 1.5rem;
    margin-right: 1.5rem;
  }

  .xl\:my-8 {
    margin-top: 2rem;
    margin-bottom: 2rem;
  }

  .xl\:mx-8 {
    margin-left: 2rem;
    margin-right: 2rem;
  }

  .xl\:my-10 {
    margin-top: 2.5rem;
    margin-bottom: 2.5rem;
  }

  .xl\:mx-10 {
    margin-left: 2.5rem;
    margin-right: 2.5rem;
  }

  .xl\:my-12 {
    margin-top: 3rem;
    margin-bottom: 3rem;
  }

  .xl\:mx-12 {
    margin-left: 3rem;
    margin-right: 3rem;
  }

  .xl\:my-16 {
    margin-top: 4rem;
    margin-bottom: 4rem;
  }

  .xl\:mx-16 {
    margin-left: 4rem;
    margin-right: 4rem;
  }

  .xl\:my-20 {
    margin-top: 5rem;
    margin-bottom: 5rem;
  }

  .xl\:mx-20 {
    margin-left: 5rem;
    margin-right: 5rem;
  }

  .xl\:my-24 {
    margin-top: 6rem;
    margin-bottom: 6rem;
  }

  .xl\:mx-24 {
    margin-left: 6rem;
    margin-right: 6rem;
  }

  .xl\:my-32 {
    margin-top: 8rem;
    margin-bottom: 8rem;
  }

  .xl\:mx-32 {
    margin-left: 8rem;
    margin-right: 8rem;
  }

  .xl\:my-40 {
    margin-top: 10rem;
    margin-bottom: 10rem;
  }

  .xl\:mx-40 {
    margin-left: 10rem;
    margin-right: 10rem;
  }

  .xl\:my-48 {
    margin-top: 12rem;
    margin-bottom: 12rem;
  }

  .xl\:mx-48 {
    margin-left: 12rem;
    margin-right: 12rem;
  }

  .xl\:my-56 {
    margin-top: 14rem;
    margin-bottom: 14rem;
  }

  .xl\:mx-56 {
    margin-left: 14rem;
    margin-right: 14rem;
  }

  .xl\:my-64 {
    margin-top: 16rem;
    margin-bottom: 16rem;
  }

  .xl\:mx-64 {
    margin-left: 16rem;
    margin-right: 16rem;
  }

  .xl\:my-auto {
    margin-top: auto;
    margin-bottom: auto;
  }

  .xl\:mx-auto {
    margin-left: auto;
    margin-right: auto;
  }

  .xl\:my-px {
    margin-top: 1px;
    margin-bottom: 1px;
  }

  .xl\:mx-px {
    margin-left: 1px;
    margin-right: 1px;
  }

  .xl\:-my-1 {
    margin-top: -0.25rem;
    margin-bottom: -0.25rem;
  }

  .xl\:-mx-1 {
    margin-left: -0.25rem;
    margin-right: -0.25rem;
  }

  .xl\:-my-2 {
    margin-top: -0.5rem;
    margin-bottom: -0.5rem;
  }

  .xl\:-mx-2 {
    margin-left: -0.5rem;
    margin-right: -0.5rem;
  }

  .xl\:-my-3 {
    margin-top: -0.75rem;
    margin-bottom: -0.75rem;
  }

  .xl\:-mx-3 {
    margin-left: -0.75rem;
    margin-right: -0.75rem;
  }

  .xl\:-my-4 {
    margin-top: -1rem;
    margin-bottom: -1rem;
  }

  .xl\:-mx-4 {
    margin-left: -1rem;
    margin-right: -1rem;
  }

  .xl\:-my-5 {
    margin-top: -1.25rem;
    margin-bottom: -1.25rem;
  }

  .xl\:-mx-5 {
    margin-left: -1.25rem;
    margin-right: -1.25rem;
  }

  .xl\:-my-6 {
    margin-top: -1.5rem;
    margin-bottom: -1.5rem;
  }

  .xl\:-mx-6 {
    margin-left: -1.5rem;
    margin-right: -1.5rem;
  }

  .xl\:-my-8 {
    margin-top: -2rem;
    margin-bottom: -2rem;
  }

  .xl\:-mx-8 {
    margin-left: -2rem;
    margin-right: -2rem;
  }

  .xl\:-my-10 {
    margin-top: -2.5rem;
    margin-bottom: -2.5rem;
  }

  .xl\:-mx-10 {
    margin-left: -2.5rem;
    margin-right: -2.5rem;
  }

  .xl\:-my-12 {
    margin-top: -3rem;
    margin-bottom: -3rem;
  }

  .xl\:-mx-12 {
    margin-left: -3rem;
    margin-right: -3rem;
  }

  .xl\:-my-16 {
    margin-top: -4rem;
    margin-bottom: -4rem;
  }

  .xl\:-mx-16 {
    margin-left: -4rem;
    margin-right: -4rem;
  }

  .xl\:-my-20 {
    margin-top: -5rem;
    margin-bottom: -5rem;
  }

  .xl\:-mx-20 {
    margin-left: -5rem;
    margin-right: -5rem;
  }

  .xl\:-my-24 {
    margin-top: -6rem;
    margin-bottom: -6rem;
  }

  .xl\:-mx-24 {
    margin-left: -6rem;
    margin-right: -6rem;
  }

  .xl\:-my-32 {
    margin-top: -8rem;
    margin-bottom: -8rem;
  }

  .xl\:-mx-32 {
    margin-left: -8rem;
    margin-right: -8rem;
  }

  .xl\:-my-40 {
    margin-top: -10rem;
    margin-bottom: -10rem;
  }

  .xl\:-mx-40 {
    margin-left: -10rem;
    margin-right: -10rem;
  }

  .xl\:-my-48 {
    margin-top: -12rem;
    margin-bottom: -12rem;
  }

  .xl\:-mx-48 {
    margin-left: -12rem;
    margin-right: -12rem;
  }

  .xl\:-my-56 {
    margin-top: -14rem;
    margin-bottom: -14rem;
  }

  .xl\:-mx-56 {
    margin-left: -14rem;
    margin-right: -14rem;
  }

  .xl\:-my-64 {
    margin-top: -16rem;
    margin-bottom: -16rem;
  }

  .xl\:-mx-64 {
    margin-left: -16rem;
    margin-right: -16rem;
  }

  .xl\:-my-px {
    margin-top: -1px;
    margin-bottom: -1px;
  }

  .xl\:-mx-px {
    margin-left: -1px;
    margin-right: -1px;
  }

  .xl\:mt-0 {
    margin-top: 0;
  }

  .xl\:mr-0 {
    margin-right: 0;
  }

  .xl\:mb-0 {
    margin-bottom: 0;
  }

  .xl\:ml-0 {
    margin-left: 0;
  }

  .xl\:mt-1 {
    margin-top: 0.25rem;
  }

  .xl\:mr-1 {
    margin-right: 0.25rem;
  }

  .xl\:mb-1 {
    margin-bottom: 0.25rem;
  }

  .xl\:ml-1 {
    margin-left: 0.25rem;
  }

  .xl\:mt-2 {
    margin-top: 0.5rem;
  }

  .xl\:mr-2 {
    margin-right: 0.5rem;
  }

  .xl\:mb-2 {
    margin-bottom: 0.5rem;
  }

  .xl\:ml-2 {
    margin-left: 0.5rem;
  }

  .xl\:mt-3 {
    margin-top: 0.75rem;
  }

  .xl\:mr-3 {
    margin-right: 0.75rem;
  }

  .xl\:mb-3 {
    margin-bottom: 0.75rem;
  }

  .xl\:ml-3 {
    margin-left: 0.75rem;
  }

  .xl\:mt-4 {
    margin-top: 1rem;
  }

  .xl\:mr-4 {
    margin-right: 1rem;
  }

  .xl\:mb-4 {
    margin-bottom: 1rem;
  }

  .xl\:ml-4 {
    margin-left: 1rem;
  }

  .xl\:mt-5 {
    margin-top: 1.25rem;
  }

  .xl\:mr-5 {
    margin-right: 1.25rem;
  }

  .xl\:mb-5 {
    margin-bottom: 1.25rem;
  }

  .xl\:ml-5 {
    margin-left: 1.25rem;
  }

  .xl\:mt-6 {
    margin-top: 1.5rem;
  }

  .xl\:mr-6 {
    margin-right: 1.5rem;
  }

  .xl\:mb-6 {
    margin-bottom: 1.5rem;
  }

  .xl\:ml-6 {
    margin-left: 1.5rem;
  }

  .xl\:mt-8 {
    margin-top: 2rem;
  }

  .xl\:mr-8 {
    margin-right: 2rem;
  }

  .xl\:mb-8 {
    margin-bottom: 2rem;
  }

  .xl\:ml-8 {
    margin-left: 2rem;
  }

  .xl\:mt-10 {
    margin-top: 2.5rem;
  }

  .xl\:mr-10 {
    margin-right: 2.5rem;
  }

  .xl\:mb-10 {
    margin-bottom: 2.5rem;
  }

  .xl\:ml-10 {
    margin-left: 2.5rem;
  }

  .xl\:mt-12 {
    margin-top: 3rem;
  }

  .xl\:mr-12 {
    margin-right: 3rem;
  }

  .xl\:mb-12 {
    margin-bottom: 3rem;
  }

  .xl\:ml-12 {
    margin-left: 3rem;
  }

  .xl\:mt-16 {
    margin-top: 4rem;
  }

  .xl\:mr-16 {
    margin-right: 4rem;
  }

  .xl\:mb-16 {
    margin-bottom: 4rem;
  }

  .xl\:ml-16 {
    margin-left: 4rem;
  }

  .xl\:mt-20 {
    margin-top: 5rem;
  }

  .xl\:mr-20 {
    margin-right: 5rem;
  }

  .xl\:mb-20 {
    margin-bottom: 5rem;
  }

  .xl\:ml-20 {
    margin-left: 5rem;
  }

  .xl\:mt-24 {
    margin-top: 6rem;
  }

  .xl\:mr-24 {
    margin-right: 6rem;
  }

  .xl\:mb-24 {
    margin-bottom: 6rem;
  }

  .xl\:ml-24 {
    margin-left: 6rem;
  }

  .xl\:mt-32 {
    margin-top: 8rem;
  }

  .xl\:mr-32 {
    margin-right: 8rem;
  }

  .xl\:mb-32 {
    margin-bottom: 8rem;
  }

  .xl\:ml-32 {
    margin-left: 8rem;
  }

  .xl\:mt-40 {
    margin-top: 10rem;
  }

  .xl\:mr-40 {
    margin-right: 10rem;
  }

  .xl\:mb-40 {
    margin-bottom: 10rem;
  }

  .xl\:ml-40 {
    margin-left: 10rem;
  }

  .xl\:mt-48 {
    margin-top: 12rem;
  }

  .xl\:mr-48 {
    margin-right: 12rem;
  }

  .xl\:mb-48 {
    margin-bottom: 12rem;
  }

  .xl\:ml-48 {
    margin-left: 12rem;
  }

  .xl\:mt-56 {
    margin-top: 14rem;
  }

  .xl\:mr-56 {
    margin-right: 14rem;
  }

  .xl\:mb-56 {
    margin-bottom: 14rem;
  }

  .xl\:ml-56 {
    margin-left: 14rem;
  }

  .xl\:mt-64 {
    margin-top: 16rem;
  }

  .xl\:mr-64 {
    margin-right: 16rem;
  }

  .xl\:mb-64 {
    margin-bottom: 16rem;
  }

  .xl\:ml-64 {
    margin-left: 16rem;
  }

  .xl\:mt-auto {
    margin-top: auto;
  }

  .xl\:mr-auto {
    margin-right: auto;
  }

  .xl\:mb-auto {
    margin-bottom: auto;
  }

  .xl\:ml-auto {
    margin-left: auto;
  }

  .xl\:mt-px {
    margin-top: 1px;
  }

  .xl\:mr-px {
    margin-right: 1px;
  }

  .xl\:mb-px {
    margin-bottom: 1px;
  }

  .xl\:ml-px {
    margin-left: 1px;
  }

  .xl\:-mt-1 {
    margin-top: -0.25rem;
  }

  .xl\:-mr-1 {
    margin-right: -0.25rem;
  }

  .xl\:-mb-1 {
    margin-bottom: -0.25rem;
  }

  .xl\:-ml-1 {
    margin-left: -0.25rem;
  }

  .xl\:-mt-2 {
    margin-top: -0.5rem;
  }

  .xl\:-mr-2 {
    margin-right: -0.5rem;
  }

  .xl\:-mb-2 {
    margin-bottom: -0.5rem;
  }

  .xl\:-ml-2 {
    margin-left: -0.5rem;
  }

  .xl\:-mt-3 {
    margin-top: -0.75rem;
  }

  .xl\:-mr-3 {
    margin-right: -0.75rem;
  }

  .xl\:-mb-3 {
    margin-bottom: -0.75rem;
  }

  .xl\:-ml-3 {
    margin-left: -0.75rem;
  }

  .xl\:-mt-4 {
    margin-top: -1rem;
  }

  .xl\:-mr-4 {
    margin-right: -1rem;
  }

  .xl\:-mb-4 {
    margin-bottom: -1rem;
  }

  .xl\:-ml-4 {
    margin-left: -1rem;
  }

  .xl\:-mt-5 {
    margin-top: -1.25rem;
  }

  .xl\:-mr-5 {
    margin-right: -1.25rem;
  }

  .xl\:-mb-5 {
    margin-bottom: -1.25rem;
  }

  .xl\:-ml-5 {
    margin-left: -1.25rem;
  }

  .xl\:-mt-6 {
    margin-top: -1.5rem;
  }

  .xl\:-mr-6 {
    margin-right: -1.5rem;
  }

  .xl\:-mb-6 {
    margin-bottom: -1.5rem;
  }

  .xl\:-ml-6 {
    margin-left: -1.5rem;
  }

  .xl\:-mt-8 {
    margin-top: -2rem;
  }

  .xl\:-mr-8 {
    margin-right: -2rem;
  }

  .xl\:-mb-8 {
    margin-bottom: -2rem;
  }

  .xl\:-ml-8 {
    margin-left: -2rem;
  }

  .xl\:-mt-10 {
    margin-top: -2.5rem;
  }

  .xl\:-mr-10 {
    margin-right: -2.5rem;
  }

  .xl\:-mb-10 {
    margin-bottom: -2.5rem;
  }

  .xl\:-ml-10 {
    margin-left: -2.5rem;
  }

  .xl\:-mt-12 {
    margin-top: -3rem;
  }

  .xl\:-mr-12 {
    margin-right: -3rem;
  }

  .xl\:-mb-12 {
    margin-bottom: -3rem;
  }

  .xl\:-ml-12 {
    margin-left: -3rem;
  }

  .xl\:-mt-16 {
    margin-top: -4rem;
  }

  .xl\:-mr-16 {
    margin-right: -4rem;
  }

  .xl\:-mb-16 {
    margin-bottom: -4rem;
  }

  .xl\:-ml-16 {
    margin-left: -4rem;
  }

  .xl\:-mt-20 {
    margin-top: -5rem;
  }

  .xl\:-mr-20 {
    margin-right: -5rem;
  }

  .xl\:-mb-20 {
    margin-bottom: -5rem;
  }

  .xl\:-ml-20 {
    margin-left: -5rem;
  }

  .xl\:-mt-24 {
    margin-top: -6rem;
  }

  .xl\:-mr-24 {
    margin-right: -6rem;
  }

  .xl\:-mb-24 {
    margin-bottom: -6rem;
  }

  .xl\:-ml-24 {
    margin-left: -6rem;
  }

  .xl\:-mt-32 {
    margin-top: -8rem;
  }

  .xl\:-mr-32 {
    margin-right: -8rem;
  }

  .xl\:-mb-32 {
    margin-bottom: -8rem;
  }

  .xl\:-ml-32 {
    margin-left: -8rem;
  }

  .xl\:-mt-40 {
    margin-top: -10rem;
  }

  .xl\:-mr-40 {
    margin-right: -10rem;
  }

  .xl\:-mb-40 {
    margin-bottom: -10rem;
  }

  .xl\:-ml-40 {
    margin-left: -10rem;
  }

  .xl\:-mt-48 {
    margin-top: -12rem;
  }

  .xl\:-mr-48 {
    margin-right: -12rem;
  }

  .xl\:-mb-48 {
    margin-bottom: -12rem;
  }

  .xl\:-ml-48 {
    margin-left: -12rem;
  }

  .xl\:-mt-56 {
    margin-top: -14rem;
  }

  .xl\:-mr-56 {
    margin-right: -14rem;
  }

  .xl\:-mb-56 {
    margin-bottom: -14rem;
  }

  .xl\:-ml-56 {
    margin-left: -14rem;
  }

  .xl\:-mt-64 {
    margin-top: -16rem;
  }

  .xl\:-mr-64 {
    margin-right: -16rem;
  }

  .xl\:-mb-64 {
    margin-bottom: -16rem;
  }

  .xl\:-ml-64 {
    margin-left: -16rem;
  }

  .xl\:-mt-px {
    margin-top: -1px;
  }

  .xl\:-mr-px {
    margin-right: -1px;
  }

  .xl\:-mb-px {
    margin-bottom: -1px;
  }

  .xl\:-ml-px {
    margin-left: -1px;
  }

  .xl\:max-h-full {
    max-height: 100%;
  }

  .xl\:max-h-screen {
    max-height: 100vh;
  }

  .xl\:max-w-xs {
    max-width: 20rem;
  }

  .xl\:max-w-sm {
    max-width: 24rem;
  }

  .xl\:max-w-md {
    max-width: 28rem;
  }

  .xl\:max-w-lg {
    max-width: 32rem;
  }

  .xl\:max-w-xl {
    max-width: 36rem;
  }

  .xl\:max-w-2xl {
    max-width: 42rem;
  }

  .xl\:max-w-3xl {
    max-width: 48rem;
  }

  .xl\:max-w-4xl {
    max-width: 56rem;
  }

  .xl\:max-w-5xl {
    max-width: 64rem;
  }

  .xl\:max-w-6xl {
    max-width: 72rem;
  }

  .xl\:max-w-full {
    max-width: 100%;
  }

  .xl\:min-h-0 {
    min-height: 0;
  }

  .xl\:min-h-full {
    min-height: 100%;
  }

  .xl\:min-h-screen {
    min-height: 100vh;
  }

  .xl\:min-w-0 {
    min-width: 0;
  }

  .xl\:min-w-full {
    min-width: 100%;
  }

  .xl\:object-contain {
    -o-object-fit: contain;
       object-fit: contain;
  }

  .xl\:object-cover {
    -o-object-fit: cover;
       object-fit: cover;
  }

  .xl\:object-fill {
    -o-object-fit: fill;
       object-fit: fill;
  }

  .xl\:object-none {
    -o-object-fit: none;
       object-fit: none;
  }

  .xl\:object-scale-down {
    -o-object-fit: scale-down;
       object-fit: scale-down;
  }

  .xl\:object-bottom {
    -o-object-position: bottom;
       object-position: bottom;
  }

  .xl\:object-center {
    -o-object-position: center;
       object-position: center;
  }

  .xl\:object-left {
    -o-object-position: left;
       object-position: left;
  }

  .xl\:object-left-bottom {
    -o-object-position: left bottom;
       object-position: left bottom;
  }

  .xl\:object-left-top {
    -o-object-position: left top;
       object-position: left top;
  }

  .xl\:object-right {
    -o-object-position: right;
       object-position: right;
  }

  .xl\:object-right-bottom {
    -o-object-position: right bottom;
       object-position: right bottom;
  }

  .xl\:object-right-top {
    -o-object-position: right top;
       object-position: right top;
  }

  .xl\:object-top {
    -o-object-position: top;
       object-position: top;
  }

  .xl\:opacity-0 {
    opacity: 0;
  }

  .xl\:opacity-25 {
    opacity: 0.25;
  }

  .xl\:opacity-50 {
    opacity: 0.5;
  }

  .xl\:opacity-75 {
    opacity: 0.75;
  }

  .xl\:opacity-100 {
    opacity: 1;
  }

  .xl\:hover\:opacity-0:hover {
    opacity: 0;
  }

  .xl\:hover\:opacity-25:hover {
    opacity: 0.25;
  }

  .xl\:hover\:opacity-50:hover {
    opacity: 0.5;
  }

  .xl\:hover\:opacity-75:hover {
    opacity: 0.75;
  }

  .xl\:hover\:opacity-100:hover {
    opacity: 1;
  }

  .xl\:focus\:opacity-0:focus {
    opacity: 0;
  }

  .xl\:focus\:opacity-25:focus {
    opacity: 0.25;
  }

  .xl\:focus\:opacity-50:focus {
    opacity: 0.5;
  }

  .xl\:focus\:opacity-75:focus {
    opacity: 0.75;
  }

  .xl\:focus\:opacity-100:focus {
    opacity: 1;
  }

  .xl\:outline-none {
    outline: 0;
  }

  .xl\:focus\:outline-none:focus {
    outline: 0;
  }

  .xl\:overflow-auto {
    overflow: auto;
  }

  .xl\:overflow-hidden {
    overflow: hidden;
  }

  .xl\:overflow-visible {
    overflow: visible;
  }

  .xl\:overflow-scroll {
    overflow: scroll;
  }

  .xl\:overflow-x-auto {
    overflow-x: auto;
  }

  .xl\:overflow-y-auto {
    overflow-y: auto;
  }

  .xl\:overflow-x-hidden {
    overflow-x: hidden;
  }

  .xl\:overflow-y-hidden {
    overflow-y: hidden;
  }

  .xl\:overflow-x-visible {
    overflow-x: visible;
  }

  .xl\:overflow-y-visible {
    overflow-y: visible;
  }

  .xl\:overflow-x-scroll {
    overflow-x: scroll;
  }

  .xl\:overflow-y-scroll {
    overflow-y: scroll;
  }

  .xl\:scrolling-touch {
    -webkit-overflow-scrolling: touch;
  }

  .xl\:scrolling-auto {
    -webkit-overflow-scrolling: auto;
  }

  .xl\:p-0 {
    padding: 0;
  }

  .xl\:p-1 {
    padding: 0.25rem;
  }

  .xl\:p-2 {
    padding: 0.5rem;
  }

  .xl\:p-3 {
    padding: 0.75rem;
  }

  .xl\:p-4 {
    padding: 1rem;
  }

  .xl\:p-5 {
    padding: 1.25rem;
  }

  .xl\:p-6 {
    padding: 1.5rem;
  }

  .xl\:p-8 {
    padding: 2rem;
  }

  .xl\:p-10 {
    padding: 2.5rem;
  }

  .xl\:p-12 {
    padding: 3rem;
  }

  .xl\:p-16 {
    padding: 4rem;
  }

  .xl\:p-20 {
    padding: 5rem;
  }

  .xl\:p-24 {
    padding: 6rem;
  }

  .xl\:p-32 {
    padding: 8rem;
  }

  .xl\:p-40 {
    padding: 10rem;
  }

  .xl\:p-48 {
    padding: 12rem;
  }

  .xl\:p-56 {
    padding: 14rem;
  }

  .xl\:p-64 {
    padding: 16rem;
  }

  .xl\:p-px {
    padding: 1px;
  }

  .xl\:py-0 {
    padding-top: 0;
    padding-bottom: 0;
  }

  .xl\:px-0 {
    padding-left: 0;
    padding-right: 0;
  }

  .xl\:py-1 {
    padding-top: 0.25rem;
    padding-bottom: 0.25rem;
  }

  .xl\:px-1 {
    padding-left: 0.25rem;
    padding-right: 0.25rem;
  }

  .xl\:py-2 {
    padding-top: 0.5rem;
    padding-bottom: 0.5rem;
  }

  .xl\:px-2 {
    padding-left: 0.5rem;
    padding-right: 0.5rem;
  }

  .xl\:py-3 {
    padding-top: 0.75rem;
    padding-bottom: 0.75rem;
  }

  .xl\:px-3 {
    padding-left: 0.75rem;
    padding-right: 0.75rem;
  }

  .xl\:py-4 {
    padding-top: 1rem;
    padding-bottom: 1rem;
  }

  .xl\:px-4 {
    padding-left: 1rem;
    padding-right: 1rem;
  }

  .xl\:py-5 {
    padding-top: 1.25rem;
    padding-bottom: 1.25rem;
  }

  .xl\:px-5 {
    padding-left: 1.25rem;
    padding-right: 1.25rem;
  }

  .xl\:py-6 {
    padding-top: 1.5rem;
    padding-bottom: 1.5rem;
  }

  .xl\:px-6 {
    padding-left: 1.5rem;
    padding-right: 1.5rem;
  }

  .xl\:py-8 {
    padding-top: 2rem;
    padding-bottom: 2rem;
  }

  .xl\:px-8 {
    padding-left: 2rem;
    padding-right: 2rem;
  }

  .xl\:py-10 {
    padding-top: 2.5rem;
    padding-bottom: 2.5rem;
  }

  .xl\:px-10 {
    padding-left: 2.5rem;
    padding-right: 2.5rem;
  }

  .xl\:py-12 {
    padding-top: 3rem;
    padding-bottom: 3rem;
  }

  .xl\:px-12 {
    padding-left: 3rem;
    padding-right: 3rem;
  }

  .xl\:py-16 {
    padding-top: 4rem;
    padding-bottom: 4rem;
  }

  .xl\:px-16 {
    padding-left: 4rem;
    padding-right: 4rem;
  }

  .xl\:py-20 {
    padding-top: 5rem;
    padding-bottom: 5rem;
  }

  .xl\:px-20 {
    padding-left: 5rem;
    padding-right: 5rem;
  }

  .xl\:py-24 {
    padding-top: 6rem;
    padding-bottom: 6rem;
  }

  .xl\:px-24 {
    padding-left: 6rem;
    padding-right: 6rem;
  }

  .xl\:py-32 {
    padding-top: 8rem;
    padding-bottom: 8rem;
  }

  .xl\:px-32 {
    padding-left: 8rem;
    padding-right: 8rem;
  }

  .xl\:py-40 {
    padding-top: 10rem;
    padding-bottom: 10rem;
  }

  .xl\:px-40 {
    padding-left: 10rem;
    padding-right: 10rem;
  }

  .xl\:py-48 {
    padding-top: 12rem;
    padding-bottom: 12rem;
  }

  .xl\:px-48 {
    padding-left: 12rem;
    padding-right: 12rem;
  }

  .xl\:py-56 {
    padding-top: 14rem;
    padding-bottom: 14rem;
  }

  .xl\:px-56 {
    padding-left: 14rem;
    padding-right: 14rem;
  }

  .xl\:py-64 {
    padding-top: 16rem;
    padding-bottom: 16rem;
  }

  .xl\:px-64 {
    padding-left: 16rem;
    padding-right: 16rem;
  }

  .xl\:py-px {
    padding-top: 1px;
    padding-bottom: 1px;
  }

  .xl\:px-px {
    padding-left: 1px;
    padding-right: 1px;
  }

  .xl\:pt-0 {
    padding-top: 0;
  }

  .xl\:pr-0 {
    padding-right: 0;
  }

  .xl\:pb-0 {
    padding-bottom: 0;
  }

  .xl\:pl-0 {
    padding-left: 0;
  }

  .xl\:pt-1 {
    padding-top: 0.25rem;
  }

  .xl\:pr-1 {
    padding-right: 0.25rem;
  }

  .xl\:pb-1 {
    padding-bottom: 0.25rem;
  }

  .xl\:pl-1 {
    padding-left: 0.25rem;
  }

  .xl\:pt-2 {
    padding-top: 0.5rem;
  }

  .xl\:pr-2 {
    padding-right: 0.5rem;
  }

  .xl\:pb-2 {
    padding-bottom: 0.5rem;
  }

  .xl\:pl-2 {
    padding-left: 0.5rem;
  }

  .xl\:pt-3 {
    padding-top: 0.75rem;
  }

  .xl\:pr-3 {
    padding-right: 0.75rem;
  }

  .xl\:pb-3 {
    padding-bottom: 0.75rem;
  }

  .xl\:pl-3 {
    padding-left: 0.75rem;
  }

  .xl\:pt-4 {
    padding-top: 1rem;
  }

  .xl\:pr-4 {
    padding-right: 1rem;
  }

  .xl\:pb-4 {
    padding-bottom: 1rem;
  }

  .xl\:pl-4 {
    padding-left: 1rem;
  }

  .xl\:pt-5 {
    padding-top: 1.25rem;
  }

  .xl\:pr-5 {
    padding-right: 1.25rem;
  }

  .xl\:pb-5 {
    padding-bottom: 1.25rem;
  }

  .xl\:pl-5 {
    padding-left: 1.25rem;
  }

  .xl\:pt-6 {
    padding-top: 1.5rem;
  }

  .xl\:pr-6 {
    padding-right: 1.5rem;
  }

  .xl\:pb-6 {
    padding-bottom: 1.5rem;
  }

  .xl\:pl-6 {
    padding-left: 1.5rem;
  }

  .xl\:pt-8 {
    padding-top: 2rem;
  }

  .xl\:pr-8 {
    padding-right: 2rem;
  }

  .xl\:pb-8 {
    padding-bottom: 2rem;
  }

  .xl\:pl-8 {
    padding-left: 2rem;
  }

  .xl\:pt-10 {
    padding-top: 2.5rem;
  }

  .xl\:pr-10 {
    padding-right: 2.5rem;
  }

  .xl\:pb-10 {
    padding-bottom: 2.5rem;
  }

  .xl\:pl-10 {
    padding-left: 2.5rem;
  }

  .xl\:pt-12 {
    padding-top: 3rem;
  }

  .xl\:pr-12 {
    padding-right: 3rem;
  }

  .xl\:pb-12 {
    padding-bottom: 3rem;
  }

  .xl\:pl-12 {
    padding-left: 3rem;
  }

  .xl\:pt-16 {
    padding-top: 4rem;
  }

  .xl\:pr-16 {
    padding-right: 4rem;
  }

  .xl\:pb-16 {
    padding-bottom: 4rem;
  }

  .xl\:pl-16 {
    padding-left: 4rem;
  }

  .xl\:pt-20 {
    padding-top: 5rem;
  }

  .xl\:pr-20 {
    padding-right: 5rem;
  }

  .xl\:pb-20 {
    padding-bottom: 5rem;
  }

  .xl\:pl-20 {
    padding-left: 5rem;
  }

  .xl\:pt-24 {
    padding-top: 6rem;
  }

  .xl\:pr-24 {
    padding-right: 6rem;
  }

  .xl\:pb-24 {
    padding-bottom: 6rem;
  }

  .xl\:pl-24 {
    padding-left: 6rem;
  }

  .xl\:pt-32 {
    padding-top: 8rem;
  }

  .xl\:pr-32 {
    padding-right: 8rem;
  }

  .xl\:pb-32 {
    padding-bottom: 8rem;
  }

  .xl\:pl-32 {
    padding-left: 8rem;
  }

  .xl\:pt-40 {
    padding-top: 10rem;
  }

  .xl\:pr-40 {
    padding-right: 10rem;
  }

  .xl\:pb-40 {
    padding-bottom: 10rem;
  }

  .xl\:pl-40 {
    padding-left: 10rem;
  }

  .xl\:pt-48 {
    padding-top: 12rem;
  }

  .xl\:pr-48 {
    padding-right: 12rem;
  }

  .xl\:pb-48 {
    padding-bottom: 12rem;
  }

  .xl\:pl-48 {
    padding-left: 12rem;
  }

  .xl\:pt-56 {
    padding-top: 14rem;
  }

  .xl\:pr-56 {
    padding-right: 14rem;
  }

  .xl\:pb-56 {
    padding-bottom: 14rem;
  }

  .xl\:pl-56 {
    padding-left: 14rem;
  }

  .xl\:pt-64 {
    padding-top: 16rem;
  }

  .xl\:pr-64 {
    padding-right: 16rem;
  }

  .xl\:pb-64 {
    padding-bottom: 16rem;
  }

  .xl\:pl-64 {
    padding-left: 16rem;
  }

  .xl\:pt-px {
    padding-top: 1px;
  }

  .xl\:pr-px {
    padding-right: 1px;
  }

  .xl\:pb-px {
    padding-bottom: 1px;
  }

  .xl\:pl-px {
    padding-left: 1px;
  }

  .xl\:placeholder-transparent::-webkit-input-placeholder {
    color: transparent;
  }

  .xl\:placeholder-transparent::-moz-placeholder {
    color: transparent;
  }

  .xl\:placeholder-transparent:-ms-input-placeholder {
    color: transparent;
  }

  .xl\:placeholder-transparent::-ms-input-placeholder {
    color: transparent;
  }

  .xl\:placeholder-transparent::placeholder {
    color: transparent;
  }

  .xl\:placeholder-black::-webkit-input-placeholder {
    color: #000;
  }

  .xl\:placeholder-black::-moz-placeholder {
    color: #000;
  }

  .xl\:placeholder-black:-ms-input-placeholder {
    color: #000;
  }

  .xl\:placeholder-black::-ms-input-placeholder {
    color: #000;
  }

  .xl\:placeholder-black::placeholder {
    color: #000;
  }

  .xl\:placeholder-white::-webkit-input-placeholder {
    color: #fff;
  }

  .xl\:placeholder-white::-moz-placeholder {
    color: #fff;
  }

  .xl\:placeholder-white:-ms-input-placeholder {
    color: #fff;
  }

  .xl\:placeholder-white::-ms-input-placeholder {
    color: #fff;
  }

  .xl\:placeholder-white::placeholder {
    color: #fff;
  }

  .xl\:placeholder-gray-100::-webkit-input-placeholder {
    color: #f7fafc;
  }

  .xl\:placeholder-gray-100::-moz-placeholder {
    color: #f7fafc;
  }

  .xl\:placeholder-gray-100:-ms-input-placeholder {
    color: #f7fafc;
  }

  .xl\:placeholder-gray-100::-ms-input-placeholder {
    color: #f7fafc;
  }

  .xl\:placeholder-gray-100::placeholder {
    color: #f7fafc;
  }

  .xl\:placeholder-gray-200::-webkit-input-placeholder {
    color: #edf2f7;
  }

  .xl\:placeholder-gray-200::-moz-placeholder {
    color: #edf2f7;
  }

  .xl\:placeholder-gray-200:-ms-input-placeholder {
    color: #edf2f7;
  }

  .xl\:placeholder-gray-200::-ms-input-placeholder {
    color: #edf2f7;
  }

  .xl\:placeholder-gray-200::placeholder {
    color: #edf2f7;
  }

  .xl\:placeholder-gray-300::-webkit-input-placeholder {
    color: #e2e8f0;
  }

  .xl\:placeholder-gray-300::-moz-placeholder {
    color: #e2e8f0;
  }

  .xl\:placeholder-gray-300:-ms-input-placeholder {
    color: #e2e8f0;
  }

  .xl\:placeholder-gray-300::-ms-input-placeholder {
    color: #e2e8f0;
  }

  .xl\:placeholder-gray-300::placeholder {
    color: #e2e8f0;
  }

  .xl\:placeholder-gray-400::-webkit-input-placeholder {
    color: #cbd5e0;
  }

  .xl\:placeholder-gray-400::-moz-placeholder {
    color: #cbd5e0;
  }

  .xl\:placeholder-gray-400:-ms-input-placeholder {
    color: #cbd5e0;
  }

  .xl\:placeholder-gray-400::-ms-input-placeholder {
    color: #cbd5e0;
  }

  .xl\:placeholder-gray-400::placeholder {
    color: #cbd5e0;
  }

  .xl\:placeholder-gray-500::-webkit-input-placeholder {
    color: #a0aec0;
  }

  .xl\:placeholder-gray-500::-moz-placeholder {
    color: #a0aec0;
  }

  .xl\:placeholder-gray-500:-ms-input-placeholder {
    color: #a0aec0;
  }

  .xl\:placeholder-gray-500::-ms-input-placeholder {
    color: #a0aec0;
  }

  .xl\:placeholder-gray-500::placeholder {
    color: #a0aec0;
  }

  .xl\:placeholder-gray-600::-webkit-input-placeholder {
    color: #718096;
  }

  .xl\:placeholder-gray-600::-moz-placeholder {
    color: #718096;
  }

  .xl\:placeholder-gray-600:-ms-input-placeholder {
    color: #718096;
  }

  .xl\:placeholder-gray-600::-ms-input-placeholder {
    color: #718096;
  }

  .xl\:placeholder-gray-600::placeholder {
    color: #718096;
  }

  .xl\:placeholder-gray-700::-webkit-input-placeholder {
    color: #4a5568;
  }

  .xl\:placeholder-gray-700::-moz-placeholder {
    color: #4a5568;
  }

  .xl\:placeholder-gray-700:-ms-input-placeholder {
    color: #4a5568;
  }

  .xl\:placeholder-gray-700::-ms-input-placeholder {
    color: #4a5568;
  }

  .xl\:placeholder-gray-700::placeholder {
    color: #4a5568;
  }

  .xl\:placeholder-gray-800::-webkit-input-placeholder {
    color: #2d3748;
  }

  .xl\:placeholder-gray-800::-moz-placeholder {
    color: #2d3748;
  }

  .xl\:placeholder-gray-800:-ms-input-placeholder {
    color: #2d3748;
  }

  .xl\:placeholder-gray-800::-ms-input-placeholder {
    color: #2d3748;
  }

  .xl\:placeholder-gray-800::placeholder {
    color: #2d3748;
  }

  .xl\:placeholder-gray-900::-webkit-input-placeholder {
    color: #1a202c;
  }

  .xl\:placeholder-gray-900::-moz-placeholder {
    color: #1a202c;
  }

  .xl\:placeholder-gray-900:-ms-input-placeholder {
    color: #1a202c;
  }

  .xl\:placeholder-gray-900::-ms-input-placeholder {
    color: #1a202c;
  }

  .xl\:placeholder-gray-900::placeholder {
    color: #1a202c;
  }

  .xl\:placeholder-red-100::-webkit-input-placeholder {
    color: #fff5f5;
  }

  .xl\:placeholder-red-100::-moz-placeholder {
    color: #fff5f5;
  }

  .xl\:placeholder-red-100:-ms-input-placeholder {
    color: #fff5f5;
  }

  .xl\:placeholder-red-100::-ms-input-placeholder {
    color: #fff5f5;
  }

  .xl\:placeholder-red-100::placeholder {
    color: #fff5f5;
  }

  .xl\:placeholder-red-200::-webkit-input-placeholder {
    color: #fed7d7;
  }

  .xl\:placeholder-red-200::-moz-placeholder {
    color: #fed7d7;
  }

  .xl\:placeholder-red-200:-ms-input-placeholder {
    color: #fed7d7;
  }

  .xl\:placeholder-red-200::-ms-input-placeholder {
    color: #fed7d7;
  }

  .xl\:placeholder-red-200::placeholder {
    color: #fed7d7;
  }

  .xl\:placeholder-red-300::-webkit-input-placeholder {
    color: #feb2b2;
  }

  .xl\:placeholder-red-300::-moz-placeholder {
    color: #feb2b2;
  }

  .xl\:placeholder-red-300:-ms-input-placeholder {
    color: #feb2b2;
  }

  .xl\:placeholder-red-300::-ms-input-placeholder {
    color: #feb2b2;
  }

  .xl\:placeholder-red-300::placeholder {
    color: #feb2b2;
  }

  .xl\:placeholder-red-400::-webkit-input-placeholder {
    color: #fc8181;
  }

  .xl\:placeholder-red-400::-moz-placeholder {
    color: #fc8181;
  }

  .xl\:placeholder-red-400:-ms-input-placeholder {
    color: #fc8181;
  }

  .xl\:placeholder-red-400::-ms-input-placeholder {
    color: #fc8181;
  }

  .xl\:placeholder-red-400::placeholder {
    color: #fc8181;
  }

  .xl\:placeholder-red-500::-webkit-input-placeholder {
    color: #f56565;
  }

  .xl\:placeholder-red-500::-moz-placeholder {
    color: #f56565;
  }

  .xl\:placeholder-red-500:-ms-input-placeholder {
    color: #f56565;
  }

  .xl\:placeholder-red-500::-ms-input-placeholder {
    color: #f56565;
  }

  .xl\:placeholder-red-500::placeholder {
    color: #f56565;
  }

  .xl\:placeholder-red-600::-webkit-input-placeholder {
    color: #e53e3e;
  }

  .xl\:placeholder-red-600::-moz-placeholder {
    color: #e53e3e;
  }

  .xl\:placeholder-red-600:-ms-input-placeholder {
    color: #e53e3e;
  }

  .xl\:placeholder-red-600::-ms-input-placeholder {
    color: #e53e3e;
  }

  .xl\:placeholder-red-600::placeholder {
    color: #e53e3e;
  }

  .xl\:placeholder-red-700::-webkit-input-placeholder {
    color: #c53030;
  }

  .xl\:placeholder-red-700::-moz-placeholder {
    color: #c53030;
  }

  .xl\:placeholder-red-700:-ms-input-placeholder {
    color: #c53030;
  }

  .xl\:placeholder-red-700::-ms-input-placeholder {
    color: #c53030;
  }

  .xl\:placeholder-red-700::placeholder {
    color: #c53030;
  }

  .xl\:placeholder-red-800::-webkit-input-placeholder {
    color: #9b2c2c;
  }

  .xl\:placeholder-red-800::-moz-placeholder {
    color: #9b2c2c;
  }

  .xl\:placeholder-red-800:-ms-input-placeholder {
    color: #9b2c2c;
  }

  .xl\:placeholder-red-800::-ms-input-placeholder {
    color: #9b2c2c;
  }

  .xl\:placeholder-red-800::placeholder {
    color: #9b2c2c;
  }

  .xl\:placeholder-red-900::-webkit-input-placeholder {
    color: #742a2a;
  }

  .xl\:placeholder-red-900::-moz-placeholder {
    color: #742a2a;
  }

  .xl\:placeholder-red-900:-ms-input-placeholder {
    color: #742a2a;
  }

  .xl\:placeholder-red-900::-ms-input-placeholder {
    color: #742a2a;
  }

  .xl\:placeholder-red-900::placeholder {
    color: #742a2a;
  }

  .xl\:placeholder-orange-100::-webkit-input-placeholder {
    color: #fffaf0;
  }

  .xl\:placeholder-orange-100::-moz-placeholder {
    color: #fffaf0;
  }

  .xl\:placeholder-orange-100:-ms-input-placeholder {
    color: #fffaf0;
  }

  .xl\:placeholder-orange-100::-ms-input-placeholder {
    color: #fffaf0;
  }

  .xl\:placeholder-orange-100::placeholder {
    color: #fffaf0;
  }

  .xl\:placeholder-orange-200::-webkit-input-placeholder {
    color: #feebc8;
  }

  .xl\:placeholder-orange-200::-moz-placeholder {
    color: #feebc8;
  }

  .xl\:placeholder-orange-200:-ms-input-placeholder {
    color: #feebc8;
  }

  .xl\:placeholder-orange-200::-ms-input-placeholder {
    color: #feebc8;
  }

  .xl\:placeholder-orange-200::placeholder {
    color: #feebc8;
  }

  .xl\:placeholder-orange-300::-webkit-input-placeholder {
    color: #fbd38d;
  }

  .xl\:placeholder-orange-300::-moz-placeholder {
    color: #fbd38d;
  }

  .xl\:placeholder-orange-300:-ms-input-placeholder {
    color: #fbd38d;
  }

  .xl\:placeholder-orange-300::-ms-input-placeholder {
    color: #fbd38d;
  }

  .xl\:placeholder-orange-300::placeholder {
    color: #fbd38d;
  }

  .xl\:placeholder-orange-400::-webkit-input-placeholder {
    color: #f6ad55;
  }

  .xl\:placeholder-orange-400::-moz-placeholder {
    color: #f6ad55;
  }

  .xl\:placeholder-orange-400:-ms-input-placeholder {
    color: #f6ad55;
  }

  .xl\:placeholder-orange-400::-ms-input-placeholder {
    color: #f6ad55;
  }

  .xl\:placeholder-orange-400::placeholder {
    color: #f6ad55;
  }

  .xl\:placeholder-orange-500::-webkit-input-placeholder {
    color: #ed8936;
  }

  .xl\:placeholder-orange-500::-moz-placeholder {
    color: #ed8936;
  }

  .xl\:placeholder-orange-500:-ms-input-placeholder {
    color: #ed8936;
  }

  .xl\:placeholder-orange-500::-ms-input-placeholder {
    color: #ed8936;
  }

  .xl\:placeholder-orange-500::placeholder {
    color: #ed8936;
  }

  .xl\:placeholder-orange-600::-webkit-input-placeholder {
    color: #dd6b20;
  }

  .xl\:placeholder-orange-600::-moz-placeholder {
    color: #dd6b20;
  }

  .xl\:placeholder-orange-600:-ms-input-placeholder {
    color: #dd6b20;
  }

  .xl\:placeholder-orange-600::-ms-input-placeholder {
    color: #dd6b20;
  }

  .xl\:placeholder-orange-600::placeholder {
    color: #dd6b20;
  }

  .xl\:placeholder-orange-700::-webkit-input-placeholder {
    color: #c05621;
  }

  .xl\:placeholder-orange-700::-moz-placeholder {
    color: #c05621;
  }

  .xl\:placeholder-orange-700:-ms-input-placeholder {
    color: #c05621;
  }

  .xl\:placeholder-orange-700::-ms-input-placeholder {
    color: #c05621;
  }

  .xl\:placeholder-orange-700::placeholder {
    color: #c05621;
  }

  .xl\:placeholder-orange-800::-webkit-input-placeholder {
    color: #9c4221;
  }

  .xl\:placeholder-orange-800::-moz-placeholder {
    color: #9c4221;
  }

  .xl\:placeholder-orange-800:-ms-input-placeholder {
    color: #9c4221;
  }

  .xl\:placeholder-orange-800::-ms-input-placeholder {
    color: #9c4221;
  }

  .xl\:placeholder-orange-800::placeholder {
    color: #9c4221;
  }

  .xl\:placeholder-orange-900::-webkit-input-placeholder {
    color: #7b341e;
  }

  .xl\:placeholder-orange-900::-moz-placeholder {
    color: #7b341e;
  }

  .xl\:placeholder-orange-900:-ms-input-placeholder {
    color: #7b341e;
  }

  .xl\:placeholder-orange-900::-ms-input-placeholder {
    color: #7b341e;
  }

  .xl\:placeholder-orange-900::placeholder {
    color: #7b341e;
  }

  .xl\:placeholder-yellow-100::-webkit-input-placeholder {
    color: #fffff0;
  }

  .xl\:placeholder-yellow-100::-moz-placeholder {
    color: #fffff0;
  }

  .xl\:placeholder-yellow-100:-ms-input-placeholder {
    color: #fffff0;
  }

  .xl\:placeholder-yellow-100::-ms-input-placeholder {
    color: #fffff0;
  }

  .xl\:placeholder-yellow-100::placeholder {
    color: #fffff0;
  }

  .xl\:placeholder-yellow-200::-webkit-input-placeholder {
    color: #fefcbf;
  }

  .xl\:placeholder-yellow-200::-moz-placeholder {
    color: #fefcbf;
  }

  .xl\:placeholder-yellow-200:-ms-input-placeholder {
    color: #fefcbf;
  }

  .xl\:placeholder-yellow-200::-ms-input-placeholder {
    color: #fefcbf;
  }

  .xl\:placeholder-yellow-200::placeholder {
    color: #fefcbf;
  }

  .xl\:placeholder-yellow-300::-webkit-input-placeholder {
    color: #faf089;
  }

  .xl\:placeholder-yellow-300::-moz-placeholder {
    color: #faf089;
  }

  .xl\:placeholder-yellow-300:-ms-input-placeholder {
    color: #faf089;
  }

  .xl\:placeholder-yellow-300::-ms-input-placeholder {
    color: #faf089;
  }

  .xl\:placeholder-yellow-300::placeholder {
    color: #faf089;
  }

  .xl\:placeholder-yellow-400::-webkit-input-placeholder {
    color: #f6e05e;
  }

  .xl\:placeholder-yellow-400::-moz-placeholder {
    color: #f6e05e;
  }

  .xl\:placeholder-yellow-400:-ms-input-placeholder {
    color: #f6e05e;
  }

  .xl\:placeholder-yellow-400::-ms-input-placeholder {
    color: #f6e05e;
  }

  .xl\:placeholder-yellow-400::placeholder {
    color: #f6e05e;
  }

  .xl\:placeholder-yellow-500::-webkit-input-placeholder {
    color: #ecc94b;
  }

  .xl\:placeholder-yellow-500::-moz-placeholder {
    color: #ecc94b;
  }

  .xl\:placeholder-yellow-500:-ms-input-placeholder {
    color: #ecc94b;
  }

  .xl\:placeholder-yellow-500::-ms-input-placeholder {
    color: #ecc94b;
  }

  .xl\:placeholder-yellow-500::placeholder {
    color: #ecc94b;
  }

  .xl\:placeholder-yellow-600::-webkit-input-placeholder {
    color: #d69e2e;
  }

  .xl\:placeholder-yellow-600::-moz-placeholder {
    color: #d69e2e;
  }

  .xl\:placeholder-yellow-600:-ms-input-placeholder {
    color: #d69e2e;
  }

  .xl\:placeholder-yellow-600::-ms-input-placeholder {
    color: #d69e2e;
  }

  .xl\:placeholder-yellow-600::placeholder {
    color: #d69e2e;
  }

  .xl\:placeholder-yellow-700::-webkit-input-placeholder {
    color: #b7791f;
  }

  .xl\:placeholder-yellow-700::-moz-placeholder {
    color: #b7791f;
  }

  .xl\:placeholder-yellow-700:-ms-input-placeholder {
    color: #b7791f;
  }

  .xl\:placeholder-yellow-700::-ms-input-placeholder {
    color: #b7791f;
  }

  .xl\:placeholder-yellow-700::placeholder {
    color: #b7791f;
  }

  .xl\:placeholder-yellow-800::-webkit-input-placeholder {
    color: #975a16;
  }

  .xl\:placeholder-yellow-800::-moz-placeholder {
    color: #975a16;
  }

  .xl\:placeholder-yellow-800:-ms-input-placeholder {
    color: #975a16;
  }

  .xl\:placeholder-yellow-800::-ms-input-placeholder {
    color: #975a16;
  }

  .xl\:placeholder-yellow-800::placeholder {
    color: #975a16;
  }

  .xl\:placeholder-yellow-900::-webkit-input-placeholder {
    color: #744210;
  }

  .xl\:placeholder-yellow-900::-moz-placeholder {
    color: #744210;
  }

  .xl\:placeholder-yellow-900:-ms-input-placeholder {
    color: #744210;
  }

  .xl\:placeholder-yellow-900::-ms-input-placeholder {
    color: #744210;
  }

  .xl\:placeholder-yellow-900::placeholder {
    color: #744210;
  }

  .xl\:placeholder-green-100::-webkit-input-placeholder {
    color: #f0fff4;
  }

  .xl\:placeholder-green-100::-moz-placeholder {
    color: #f0fff4;
  }

  .xl\:placeholder-green-100:-ms-input-placeholder {
    color: #f0fff4;
  }

  .xl\:placeholder-green-100::-ms-input-placeholder {
    color: #f0fff4;
  }

  .xl\:placeholder-green-100::placeholder {
    color: #f0fff4;
  }

  .xl\:placeholder-green-200::-webkit-input-placeholder {
    color: #c6f6d5;
  }

  .xl\:placeholder-green-200::-moz-placeholder {
    color: #c6f6d5;
  }

  .xl\:placeholder-green-200:-ms-input-placeholder {
    color: #c6f6d5;
  }

  .xl\:placeholder-green-200::-ms-input-placeholder {
    color: #c6f6d5;
  }

  .xl\:placeholder-green-200::placeholder {
    color: #c6f6d5;
  }

  .xl\:placeholder-green-300::-webkit-input-placeholder {
    color: #9ae6b4;
  }

  .xl\:placeholder-green-300::-moz-placeholder {
    color: #9ae6b4;
  }

  .xl\:placeholder-green-300:-ms-input-placeholder {
    color: #9ae6b4;
  }

  .xl\:placeholder-green-300::-ms-input-placeholder {
    color: #9ae6b4;
  }

  .xl\:placeholder-green-300::placeholder {
    color: #9ae6b4;
  }

  .xl\:placeholder-green-400::-webkit-input-placeholder {
    color: #68d391;
  }

  .xl\:placeholder-green-400::-moz-placeholder {
    color: #68d391;
  }

  .xl\:placeholder-green-400:-ms-input-placeholder {
    color: #68d391;
  }

  .xl\:placeholder-green-400::-ms-input-placeholder {
    color: #68d391;
  }

  .xl\:placeholder-green-400::placeholder {
    color: #68d391;
  }

  .xl\:placeholder-green-500::-webkit-input-placeholder {
    color: #48bb78;
  }

  .xl\:placeholder-green-500::-moz-placeholder {
    color: #48bb78;
  }

  .xl\:placeholder-green-500:-ms-input-placeholder {
    color: #48bb78;
  }

  .xl\:placeholder-green-500::-ms-input-placeholder {
    color: #48bb78;
  }

  .xl\:placeholder-green-500::placeholder {
    color: #48bb78;
  }

  .xl\:placeholder-green-600::-webkit-input-placeholder {
    color: #38a169;
  }

  .xl\:placeholder-green-600::-moz-placeholder {
    color: #38a169;
  }

  .xl\:placeholder-green-600:-ms-input-placeholder {
    color: #38a169;
  }

  .xl\:placeholder-green-600::-ms-input-placeholder {
    color: #38a169;
  }

  .xl\:placeholder-green-600::placeholder {
    color: #38a169;
  }

  .xl\:placeholder-green-700::-webkit-input-placeholder {
    color: #2f855a;
  }

  .xl\:placeholder-green-700::-moz-placeholder {
    color: #2f855a;
  }

  .xl\:placeholder-green-700:-ms-input-placeholder {
    color: #2f855a;
  }

  .xl\:placeholder-green-700::-ms-input-placeholder {
    color: #2f855a;
  }

  .xl\:placeholder-green-700::placeholder {
    color: #2f855a;
  }

  .xl\:placeholder-green-800::-webkit-input-placeholder {
    color: #276749;
  }

  .xl\:placeholder-green-800::-moz-placeholder {
    color: #276749;
  }

  .xl\:placeholder-green-800:-ms-input-placeholder {
    color: #276749;
  }

  .xl\:placeholder-green-800::-ms-input-placeholder {
    color: #276749;
  }

  .xl\:placeholder-green-800::placeholder {
    color: #276749;
  }

  .xl\:placeholder-green-900::-webkit-input-placeholder {
    color: #22543d;
  }

  .xl\:placeholder-green-900::-moz-placeholder {
    color: #22543d;
  }

  .xl\:placeholder-green-900:-ms-input-placeholder {
    color: #22543d;
  }

  .xl\:placeholder-green-900::-ms-input-placeholder {
    color: #22543d;
  }

  .xl\:placeholder-green-900::placeholder {
    color: #22543d;
  }

  .xl\:placeholder-teal-100::-webkit-input-placeholder {
    color: #e6fffa;
  }

  .xl\:placeholder-teal-100::-moz-placeholder {
    color: #e6fffa;
  }

  .xl\:placeholder-teal-100:-ms-input-placeholder {
    color: #e6fffa;
  }

  .xl\:placeholder-teal-100::-ms-input-placeholder {
    color: #e6fffa;
  }

  .xl\:placeholder-teal-100::placeholder {
    color: #e6fffa;
  }

  .xl\:placeholder-teal-200::-webkit-input-placeholder {
    color: #b2f5ea;
  }

  .xl\:placeholder-teal-200::-moz-placeholder {
    color: #b2f5ea;
  }

  .xl\:placeholder-teal-200:-ms-input-placeholder {
    color: #b2f5ea;
  }

  .xl\:placeholder-teal-200::-ms-input-placeholder {
    color: #b2f5ea;
  }

  .xl\:placeholder-teal-200::placeholder {
    color: #b2f5ea;
  }

  .xl\:placeholder-teal-300::-webkit-input-placeholder {
    color: #81e6d9;
  }

  .xl\:placeholder-teal-300::-moz-placeholder {
    color: #81e6d9;
  }

  .xl\:placeholder-teal-300:-ms-input-placeholder {
    color: #81e6d9;
  }

  .xl\:placeholder-teal-300::-ms-input-placeholder {
    color: #81e6d9;
  }

  .xl\:placeholder-teal-300::placeholder {
    color: #81e6d9;
  }

  .xl\:placeholder-teal-400::-webkit-input-placeholder {
    color: #4fd1c5;
  }

  .xl\:placeholder-teal-400::-moz-placeholder {
    color: #4fd1c5;
  }

  .xl\:placeholder-teal-400:-ms-input-placeholder {
    color: #4fd1c5;
  }

  .xl\:placeholder-teal-400::-ms-input-placeholder {
    color: #4fd1c5;
  }

  .xl\:placeholder-teal-400::placeholder {
    color: #4fd1c5;
  }

  .xl\:placeholder-teal-500::-webkit-input-placeholder {
    color: #38b2ac;
  }

  .xl\:placeholder-teal-500::-moz-placeholder {
    color: #38b2ac;
  }

  .xl\:placeholder-teal-500:-ms-input-placeholder {
    color: #38b2ac;
  }

  .xl\:placeholder-teal-500::-ms-input-placeholder {
    color: #38b2ac;
  }

  .xl\:placeholder-teal-500::placeholder {
    color: #38b2ac;
  }

  .xl\:placeholder-teal-600::-webkit-input-placeholder {
    color: #319795;
  }

  .xl\:placeholder-teal-600::-moz-placeholder {
    color: #319795;
  }

  .xl\:placeholder-teal-600:-ms-input-placeholder {
    color: #319795;
  }

  .xl\:placeholder-teal-600::-ms-input-placeholder {
    color: #319795;
  }

  .xl\:placeholder-teal-600::placeholder {
    color: #319795;
  }

  .xl\:placeholder-teal-700::-webkit-input-placeholder {
    color: #2c7a7b;
  }

  .xl\:placeholder-teal-700::-moz-placeholder {
    color: #2c7a7b;
  }

  .xl\:placeholder-teal-700:-ms-input-placeholder {
    color: #2c7a7b;
  }

  .xl\:placeholder-teal-700::-ms-input-placeholder {
    color: #2c7a7b;
  }

  .xl\:placeholder-teal-700::placeholder {
    color: #2c7a7b;
  }

  .xl\:placeholder-teal-800::-webkit-input-placeholder {
    color: #285e61;
  }

  .xl\:placeholder-teal-800::-moz-placeholder {
    color: #285e61;
  }

  .xl\:placeholder-teal-800:-ms-input-placeholder {
    color: #285e61;
  }

  .xl\:placeholder-teal-800::-ms-input-placeholder {
    color: #285e61;
  }

  .xl\:placeholder-teal-800::placeholder {
    color: #285e61;
  }

  .xl\:placeholder-teal-900::-webkit-input-placeholder {
    color: #234e52;
  }

  .xl\:placeholder-teal-900::-moz-placeholder {
    color: #234e52;
  }

  .xl\:placeholder-teal-900:-ms-input-placeholder {
    color: #234e52;
  }

  .xl\:placeholder-teal-900::-ms-input-placeholder {
    color: #234e52;
  }

  .xl\:placeholder-teal-900::placeholder {
    color: #234e52;
  }

  .xl\:placeholder-blue-100::-webkit-input-placeholder {
    color: #ebf8ff;
  }

  .xl\:placeholder-blue-100::-moz-placeholder {
    color: #ebf8ff;
  }

  .xl\:placeholder-blue-100:-ms-input-placeholder {
    color: #ebf8ff;
  }

  .xl\:placeholder-blue-100::-ms-input-placeholder {
    color: #ebf8ff;
  }

  .xl\:placeholder-blue-100::placeholder {
    color: #ebf8ff;
  }

  .xl\:placeholder-blue-200::-webkit-input-placeholder {
    color: #bee3f8;
  }

  .xl\:placeholder-blue-200::-moz-placeholder {
    color: #bee3f8;
  }

  .xl\:placeholder-blue-200:-ms-input-placeholder {
    color: #bee3f8;
  }

  .xl\:placeholder-blue-200::-ms-input-placeholder {
    color: #bee3f8;
  }

  .xl\:placeholder-blue-200::placeholder {
    color: #bee3f8;
  }

  .xl\:placeholder-blue-300::-webkit-input-placeholder {
    color: #90cdf4;
  }

  .xl\:placeholder-blue-300::-moz-placeholder {
    color: #90cdf4;
  }

  .xl\:placeholder-blue-300:-ms-input-placeholder {
    color: #90cdf4;
  }

  .xl\:placeholder-blue-300::-ms-input-placeholder {
    color: #90cdf4;
  }

  .xl\:placeholder-blue-300::placeholder {
    color: #90cdf4;
  }

  .xl\:placeholder-blue-400::-webkit-input-placeholder {
    color: #63b3ed;
  }

  .xl\:placeholder-blue-400::-moz-placeholder {
    color: #63b3ed;
  }

  .xl\:placeholder-blue-400:-ms-input-placeholder {
    color: #63b3ed;
  }

  .xl\:placeholder-blue-400::-ms-input-placeholder {
    color: #63b3ed;
  }

  .xl\:placeholder-blue-400::placeholder {
    color: #63b3ed;
  }

  .xl\:placeholder-blue-500::-webkit-input-placeholder {
    color: #4299e1;
  }

  .xl\:placeholder-blue-500::-moz-placeholder {
    color: #4299e1;
  }

  .xl\:placeholder-blue-500:-ms-input-placeholder {
    color: #4299e1;
  }

  .xl\:placeholder-blue-500::-ms-input-placeholder {
    color: #4299e1;
  }

  .xl\:placeholder-blue-500::placeholder {
    color: #4299e1;
  }

  .xl\:placeholder-blue-600::-webkit-input-placeholder {
    color: #3182ce;
  }

  .xl\:placeholder-blue-600::-moz-placeholder {
    color: #3182ce;
  }

  .xl\:placeholder-blue-600:-ms-input-placeholder {
    color: #3182ce;
  }

  .xl\:placeholder-blue-600::-ms-input-placeholder {
    color: #3182ce;
  }

  .xl\:placeholder-blue-600::placeholder {
    color: #3182ce;
  }

  .xl\:placeholder-blue-700::-webkit-input-placeholder {
    color: #2b6cb0;
  }

  .xl\:placeholder-blue-700::-moz-placeholder {
    color: #2b6cb0;
  }

  .xl\:placeholder-blue-700:-ms-input-placeholder {
    color: #2b6cb0;
  }

  .xl\:placeholder-blue-700::-ms-input-placeholder {
    color: #2b6cb0;
  }

  .xl\:placeholder-blue-700::placeholder {
    color: #2b6cb0;
  }

  .xl\:placeholder-blue-800::-webkit-input-placeholder {
    color: #2c5282;
  }

  .xl\:placeholder-blue-800::-moz-placeholder {
    color: #2c5282;
  }

  .xl\:placeholder-blue-800:-ms-input-placeholder {
    color: #2c5282;
  }

  .xl\:placeholder-blue-800::-ms-input-placeholder {
    color: #2c5282;
  }

  .xl\:placeholder-blue-800::placeholder {
    color: #2c5282;
  }

  .xl\:placeholder-blue-900::-webkit-input-placeholder {
    color: #2a4365;
  }

  .xl\:placeholder-blue-900::-moz-placeholder {
    color: #2a4365;
  }

  .xl\:placeholder-blue-900:-ms-input-placeholder {
    color: #2a4365;
  }

  .xl\:placeholder-blue-900::-ms-input-placeholder {
    color: #2a4365;
  }

  .xl\:placeholder-blue-900::placeholder {
    color: #2a4365;
  }

  .xl\:placeholder-indigo-100::-webkit-input-placeholder {
    color: #ebf4ff;
  }

  .xl\:placeholder-indigo-100::-moz-placeholder {
    color: #ebf4ff;
  }

  .xl\:placeholder-indigo-100:-ms-input-placeholder {
    color: #ebf4ff;
  }

  .xl\:placeholder-indigo-100::-ms-input-placeholder {
    color: #ebf4ff;
  }

  .xl\:placeholder-indigo-100::placeholder {
    color: #ebf4ff;
  }

  .xl\:placeholder-indigo-200::-webkit-input-placeholder {
    color: #c3dafe;
  }

  .xl\:placeholder-indigo-200::-moz-placeholder {
    color: #c3dafe;
  }

  .xl\:placeholder-indigo-200:-ms-input-placeholder {
    color: #c3dafe;
  }

  .xl\:placeholder-indigo-200::-ms-input-placeholder {
    color: #c3dafe;
  }

  .xl\:placeholder-indigo-200::placeholder {
    color: #c3dafe;
  }

  .xl\:placeholder-indigo-300::-webkit-input-placeholder {
    color: #a3bffa;
  }

  .xl\:placeholder-indigo-300::-moz-placeholder {
    color: #a3bffa;
  }

  .xl\:placeholder-indigo-300:-ms-input-placeholder {
    color: #a3bffa;
  }

  .xl\:placeholder-indigo-300::-ms-input-placeholder {
    color: #a3bffa;
  }

  .xl\:placeholder-indigo-300::placeholder {
    color: #a3bffa;
  }

  .xl\:placeholder-indigo-400::-webkit-input-placeholder {
    color: #7f9cf5;
  }

  .xl\:placeholder-indigo-400::-moz-placeholder {
    color: #7f9cf5;
  }

  .xl\:placeholder-indigo-400:-ms-input-placeholder {
    color: #7f9cf5;
  }

  .xl\:placeholder-indigo-400::-ms-input-placeholder {
    color: #7f9cf5;
  }

  .xl\:placeholder-indigo-400::placeholder {
    color: #7f9cf5;
  }

  .xl\:placeholder-indigo-500::-webkit-input-placeholder {
    color: #667eea;
  }

  .xl\:placeholder-indigo-500::-moz-placeholder {
    color: #667eea;
  }

  .xl\:placeholder-indigo-500:-ms-input-placeholder {
    color: #667eea;
  }

  .xl\:placeholder-indigo-500::-ms-input-placeholder {
    color: #667eea;
  }

  .xl\:placeholder-indigo-500::placeholder {
    color: #667eea;
  }

  .xl\:placeholder-indigo-600::-webkit-input-placeholder {
    color: #5a67d8;
  }

  .xl\:placeholder-indigo-600::-moz-placeholder {
    color: #5a67d8;
  }

  .xl\:placeholder-indigo-600:-ms-input-placeholder {
    color: #5a67d8;
  }

  .xl\:placeholder-indigo-600::-ms-input-placeholder {
    color: #5a67d8;
  }

  .xl\:placeholder-indigo-600::placeholder {
    color: #5a67d8;
  }

  .xl\:placeholder-indigo-700::-webkit-input-placeholder {
    color: #4c51bf;
  }

  .xl\:placeholder-indigo-700::-moz-placeholder {
    color: #4c51bf;
  }

  .xl\:placeholder-indigo-700:-ms-input-placeholder {
    color: #4c51bf;
  }

  .xl\:placeholder-indigo-700::-ms-input-placeholder {
    color: #4c51bf;
  }

  .xl\:placeholder-indigo-700::placeholder {
    color: #4c51bf;
  }

  .xl\:placeholder-indigo-800::-webkit-input-placeholder {
    color: #434190;
  }

  .xl\:placeholder-indigo-800::-moz-placeholder {
    color: #434190;
  }

  .xl\:placeholder-indigo-800:-ms-input-placeholder {
    color: #434190;
  }

  .xl\:placeholder-indigo-800::-ms-input-placeholder {
    color: #434190;
  }

  .xl\:placeholder-indigo-800::placeholder {
    color: #434190;
  }

  .xl\:placeholder-indigo-900::-webkit-input-placeholder {
    color: #3c366b;
  }

  .xl\:placeholder-indigo-900::-moz-placeholder {
    color: #3c366b;
  }

  .xl\:placeholder-indigo-900:-ms-input-placeholder {
    color: #3c366b;
  }

  .xl\:placeholder-indigo-900::-ms-input-placeholder {
    color: #3c366b;
  }

  .xl\:placeholder-indigo-900::placeholder {
    color: #3c366b;
  }

  .xl\:placeholder-purple-100::-webkit-input-placeholder {
    color: #faf5ff;
  }

  .xl\:placeholder-purple-100::-moz-placeholder {
    color: #faf5ff;
  }

  .xl\:placeholder-purple-100:-ms-input-placeholder {
    color: #faf5ff;
  }

  .xl\:placeholder-purple-100::-ms-input-placeholder {
    color: #faf5ff;
  }

  .xl\:placeholder-purple-100::placeholder {
    color: #faf5ff;
  }

  .xl\:placeholder-purple-200::-webkit-input-placeholder {
    color: #e9d8fd;
  }

  .xl\:placeholder-purple-200::-moz-placeholder {
    color: #e9d8fd;
  }

  .xl\:placeholder-purple-200:-ms-input-placeholder {
    color: #e9d8fd;
  }

  .xl\:placeholder-purple-200::-ms-input-placeholder {
    color: #e9d8fd;
  }

  .xl\:placeholder-purple-200::placeholder {
    color: #e9d8fd;
  }

  .xl\:placeholder-purple-300::-webkit-input-placeholder {
    color: #d6bcfa;
  }

  .xl\:placeholder-purple-300::-moz-placeholder {
    color: #d6bcfa;
  }

  .xl\:placeholder-purple-300:-ms-input-placeholder {
    color: #d6bcfa;
  }

  .xl\:placeholder-purple-300::-ms-input-placeholder {
    color: #d6bcfa;
  }

  .xl\:placeholder-purple-300::placeholder {
    color: #d6bcfa;
  }

  .xl\:placeholder-purple-400::-webkit-input-placeholder {
    color: #b794f4;
  }

  .xl\:placeholder-purple-400::-moz-placeholder {
    color: #b794f4;
  }

  .xl\:placeholder-purple-400:-ms-input-placeholder {
    color: #b794f4;
  }

  .xl\:placeholder-purple-400::-ms-input-placeholder {
    color: #b794f4;
  }

  .xl\:placeholder-purple-400::placeholder {
    color: #b794f4;
  }

  .xl\:placeholder-purple-500::-webkit-input-placeholder {
    color: #9f7aea;
  }

  .xl\:placeholder-purple-500::-moz-placeholder {
    color: #9f7aea;
  }

  .xl\:placeholder-purple-500:-ms-input-placeholder {
    color: #9f7aea;
  }

  .xl\:placeholder-purple-500::-ms-input-placeholder {
    color: #9f7aea;
  }

  .xl\:placeholder-purple-500::placeholder {
    color: #9f7aea;
  }

  .xl\:placeholder-purple-600::-webkit-input-placeholder {
    color: #805ad5;
  }

  .xl\:placeholder-purple-600::-moz-placeholder {
    color: #805ad5;
  }

  .xl\:placeholder-purple-600:-ms-input-placeholder {
    color: #805ad5;
  }

  .xl\:placeholder-purple-600::-ms-input-placeholder {
    color: #805ad5;
  }

  .xl\:placeholder-purple-600::placeholder {
    color: #805ad5;
  }

  .xl\:placeholder-purple-700::-webkit-input-placeholder {
    color: #6b46c1;
  }

  .xl\:placeholder-purple-700::-moz-placeholder {
    color: #6b46c1;
  }

  .xl\:placeholder-purple-700:-ms-input-placeholder {
    color: #6b46c1;
  }

  .xl\:placeholder-purple-700::-ms-input-placeholder {
    color: #6b46c1;
  }

  .xl\:placeholder-purple-700::placeholder {
    color: #6b46c1;
  }

  .xl\:placeholder-purple-800::-webkit-input-placeholder {
    color: #553c9a;
  }

  .xl\:placeholder-purple-800::-moz-placeholder {
    color: #553c9a;
  }

  .xl\:placeholder-purple-800:-ms-input-placeholder {
    color: #553c9a;
  }

  .xl\:placeholder-purple-800::-ms-input-placeholder {
    color: #553c9a;
  }

  .xl\:placeholder-purple-800::placeholder {
    color: #553c9a;
  }

  .xl\:placeholder-purple-900::-webkit-input-placeholder {
    color: #44337a;
  }

  .xl\:placeholder-purple-900::-moz-placeholder {
    color: #44337a;
  }

  .xl\:placeholder-purple-900:-ms-input-placeholder {
    color: #44337a;
  }

  .xl\:placeholder-purple-900::-ms-input-placeholder {
    color: #44337a;
  }

  .xl\:placeholder-purple-900::placeholder {
    color: #44337a;
  }

  .xl\:placeholder-pink-100::-webkit-input-placeholder {
    color: #fff5f7;
  }

  .xl\:placeholder-pink-100::-moz-placeholder {
    color: #fff5f7;
  }

  .xl\:placeholder-pink-100:-ms-input-placeholder {
    color: #fff5f7;
  }

  .xl\:placeholder-pink-100::-ms-input-placeholder {
    color: #fff5f7;
  }

  .xl\:placeholder-pink-100::placeholder {
    color: #fff5f7;
  }

  .xl\:placeholder-pink-200::-webkit-input-placeholder {
    color: #fed7e2;
  }

  .xl\:placeholder-pink-200::-moz-placeholder {
    color: #fed7e2;
  }

  .xl\:placeholder-pink-200:-ms-input-placeholder {
    color: #fed7e2;
  }

  .xl\:placeholder-pink-200::-ms-input-placeholder {
    color: #fed7e2;
  }

  .xl\:placeholder-pink-200::placeholder {
    color: #fed7e2;
  }

  .xl\:placeholder-pink-300::-webkit-input-placeholder {
    color: #fbb6ce;
  }

  .xl\:placeholder-pink-300::-moz-placeholder {
    color: #fbb6ce;
  }

  .xl\:placeholder-pink-300:-ms-input-placeholder {
    color: #fbb6ce;
  }

  .xl\:placeholder-pink-300::-ms-input-placeholder {
    color: #fbb6ce;
  }

  .xl\:placeholder-pink-300::placeholder {
    color: #fbb6ce;
  }

  .xl\:placeholder-pink-400::-webkit-input-placeholder {
    color: #f687b3;
  }

  .xl\:placeholder-pink-400::-moz-placeholder {
    color: #f687b3;
  }

  .xl\:placeholder-pink-400:-ms-input-placeholder {
    color: #f687b3;
  }

  .xl\:placeholder-pink-400::-ms-input-placeholder {
    color: #f687b3;
  }

  .xl\:placeholder-pink-400::placeholder {
    color: #f687b3;
  }

  .xl\:placeholder-pink-500::-webkit-input-placeholder {
    color: #ed64a6;
  }

  .xl\:placeholder-pink-500::-moz-placeholder {
    color: #ed64a6;
  }

  .xl\:placeholder-pink-500:-ms-input-placeholder {
    color: #ed64a6;
  }

  .xl\:placeholder-pink-500::-ms-input-placeholder {
    color: #ed64a6;
  }

  .xl\:placeholder-pink-500::placeholder {
    color: #ed64a6;
  }

  .xl\:placeholder-pink-600::-webkit-input-placeholder {
    color: #d53f8c;
  }

  .xl\:placeholder-pink-600::-moz-placeholder {
    color: #d53f8c;
  }

  .xl\:placeholder-pink-600:-ms-input-placeholder {
    color: #d53f8c;
  }

  .xl\:placeholder-pink-600::-ms-input-placeholder {
    color: #d53f8c;
  }

  .xl\:placeholder-pink-600::placeholder {
    color: #d53f8c;
  }

  .xl\:placeholder-pink-700::-webkit-input-placeholder {
    color: #b83280;
  }

  .xl\:placeholder-pink-700::-moz-placeholder {
    color: #b83280;
  }

  .xl\:placeholder-pink-700:-ms-input-placeholder {
    color: #b83280;
  }

  .xl\:placeholder-pink-700::-ms-input-placeholder {
    color: #b83280;
  }

  .xl\:placeholder-pink-700::placeholder {
    color: #b83280;
  }

  .xl\:placeholder-pink-800::-webkit-input-placeholder {
    color: #97266d;
  }

  .xl\:placeholder-pink-800::-moz-placeholder {
    color: #97266d;
  }

  .xl\:placeholder-pink-800:-ms-input-placeholder {
    color: #97266d;
  }

  .xl\:placeholder-pink-800::-ms-input-placeholder {
    color: #97266d;
  }

  .xl\:placeholder-pink-800::placeholder {
    color: #97266d;
  }

  .xl\:placeholder-pink-900::-webkit-input-placeholder {
    color: #702459;
  }

  .xl\:placeholder-pink-900::-moz-placeholder {
    color: #702459;
  }

  .xl\:placeholder-pink-900:-ms-input-placeholder {
    color: #702459;
  }

  .xl\:placeholder-pink-900::-ms-input-placeholder {
    color: #702459;
  }

  .xl\:placeholder-pink-900::placeholder {
    color: #702459;
  }

  .xl\:focus\:placeholder-transparent:focus::-webkit-input-placeholder {
    color: transparent;
  }

  .xl\:focus\:placeholder-transparent:focus::-moz-placeholder {
    color: transparent;
  }

  .xl\:focus\:placeholder-transparent:focus:-ms-input-placeholder {
    color: transparent;
  }

  .xl\:focus\:placeholder-transparent:focus::-ms-input-placeholder {
    color: transparent;
  }

  .xl\:focus\:placeholder-transparent:focus::placeholder {
    color: transparent;
  }

  .xl\:focus\:placeholder-black:focus::-webkit-input-placeholder {
    color: #000;
  }

  .xl\:focus\:placeholder-black:focus::-moz-placeholder {
    color: #000;
  }

  .xl\:focus\:placeholder-black:focus:-ms-input-placeholder {
    color: #000;
  }

  .xl\:focus\:placeholder-black:focus::-ms-input-placeholder {
    color: #000;
  }

  .xl\:focus\:placeholder-black:focus::placeholder {
    color: #000;
  }

  .xl\:focus\:placeholder-white:focus::-webkit-input-placeholder {
    color: #fff;
  }

  .xl\:focus\:placeholder-white:focus::-moz-placeholder {
    color: #fff;
  }

  .xl\:focus\:placeholder-white:focus:-ms-input-placeholder {
    color: #fff;
  }

  .xl\:focus\:placeholder-white:focus::-ms-input-placeholder {
    color: #fff;
  }

  .xl\:focus\:placeholder-white:focus::placeholder {
    color: #fff;
  }

  .xl\:focus\:placeholder-gray-100:focus::-webkit-input-placeholder {
    color: #f7fafc;
  }

  .xl\:focus\:placeholder-gray-100:focus::-moz-placeholder {
    color: #f7fafc;
  }

  .xl\:focus\:placeholder-gray-100:focus:-ms-input-placeholder {
    color: #f7fafc;
  }

  .xl\:focus\:placeholder-gray-100:focus::-ms-input-placeholder {
    color: #f7fafc;
  }

  .xl\:focus\:placeholder-gray-100:focus::placeholder {
    color: #f7fafc;
  }

  .xl\:focus\:placeholder-gray-200:focus::-webkit-input-placeholder {
    color: #edf2f7;
  }

  .xl\:focus\:placeholder-gray-200:focus::-moz-placeholder {
    color: #edf2f7;
  }

  .xl\:focus\:placeholder-gray-200:focus:-ms-input-placeholder {
    color: #edf2f7;
  }

  .xl\:focus\:placeholder-gray-200:focus::-ms-input-placeholder {
    color: #edf2f7;
  }

  .xl\:focus\:placeholder-gray-200:focus::placeholder {
    color: #edf2f7;
  }

  .xl\:focus\:placeholder-gray-300:focus::-webkit-input-placeholder {
    color: #e2e8f0;
  }

  .xl\:focus\:placeholder-gray-300:focus::-moz-placeholder {
    color: #e2e8f0;
  }

  .xl\:focus\:placeholder-gray-300:focus:-ms-input-placeholder {
    color: #e2e8f0;
  }

  .xl\:focus\:placeholder-gray-300:focus::-ms-input-placeholder {
    color: #e2e8f0;
  }

  .xl\:focus\:placeholder-gray-300:focus::placeholder {
    color: #e2e8f0;
  }

  .xl\:focus\:placeholder-gray-400:focus::-webkit-input-placeholder {
    color: #cbd5e0;
  }

  .xl\:focus\:placeholder-gray-400:focus::-moz-placeholder {
    color: #cbd5e0;
  }

  .xl\:focus\:placeholder-gray-400:focus:-ms-input-placeholder {
    color: #cbd5e0;
  }

  .xl\:focus\:placeholder-gray-400:focus::-ms-input-placeholder {
    color: #cbd5e0;
  }

  .xl\:focus\:placeholder-gray-400:focus::placeholder {
    color: #cbd5e0;
  }

  .xl\:focus\:placeholder-gray-500:focus::-webkit-input-placeholder {
    color: #a0aec0;
  }

  .xl\:focus\:placeholder-gray-500:focus::-moz-placeholder {
    color: #a0aec0;
  }

  .xl\:focus\:placeholder-gray-500:focus:-ms-input-placeholder {
    color: #a0aec0;
  }

  .xl\:focus\:placeholder-gray-500:focus::-ms-input-placeholder {
    color: #a0aec0;
  }

  .xl\:focus\:placeholder-gray-500:focus::placeholder {
    color: #a0aec0;
  }

  .xl\:focus\:placeholder-gray-600:focus::-webkit-input-placeholder {
    color: #718096;
  }

  .xl\:focus\:placeholder-gray-600:focus::-moz-placeholder {
    color: #718096;
  }

  .xl\:focus\:placeholder-gray-600:focus:-ms-input-placeholder {
    color: #718096;
  }

  .xl\:focus\:placeholder-gray-600:focus::-ms-input-placeholder {
    color: #718096;
  }

  .xl\:focus\:placeholder-gray-600:focus::placeholder {
    color: #718096;
  }

  .xl\:focus\:placeholder-gray-700:focus::-webkit-input-placeholder {
    color: #4a5568;
  }

  .xl\:focus\:placeholder-gray-700:focus::-moz-placeholder {
    color: #4a5568;
  }

  .xl\:focus\:placeholder-gray-700:focus:-ms-input-placeholder {
    color: #4a5568;
  }

  .xl\:focus\:placeholder-gray-700:focus::-ms-input-placeholder {
    color: #4a5568;
  }

  .xl\:focus\:placeholder-gray-700:focus::placeholder {
    color: #4a5568;
  }

  .xl\:focus\:placeholder-gray-800:focus::-webkit-input-placeholder {
    color: #2d3748;
  }

  .xl\:focus\:placeholder-gray-800:focus::-moz-placeholder {
    color: #2d3748;
  }

  .xl\:focus\:placeholder-gray-800:focus:-ms-input-placeholder {
    color: #2d3748;
  }

  .xl\:focus\:placeholder-gray-800:focus::-ms-input-placeholder {
    color: #2d3748;
  }

  .xl\:focus\:placeholder-gray-800:focus::placeholder {
    color: #2d3748;
  }

  .xl\:focus\:placeholder-gray-900:focus::-webkit-input-placeholder {
    color: #1a202c;
  }

  .xl\:focus\:placeholder-gray-900:focus::-moz-placeholder {
    color: #1a202c;
  }

  .xl\:focus\:placeholder-gray-900:focus:-ms-input-placeholder {
    color: #1a202c;
  }

  .xl\:focus\:placeholder-gray-900:focus::-ms-input-placeholder {
    color: #1a202c;
  }

  .xl\:focus\:placeholder-gray-900:focus::placeholder {
    color: #1a202c;
  }

  .xl\:focus\:placeholder-red-100:focus::-webkit-input-placeholder {
    color: #fff5f5;
  }

  .xl\:focus\:placeholder-red-100:focus::-moz-placeholder {
    color: #fff5f5;
  }

  .xl\:focus\:placeholder-red-100:focus:-ms-input-placeholder {
    color: #fff5f5;
  }

  .xl\:focus\:placeholder-red-100:focus::-ms-input-placeholder {
    color: #fff5f5;
  }

  .xl\:focus\:placeholder-red-100:focus::placeholder {
    color: #fff5f5;
  }

  .xl\:focus\:placeholder-red-200:focus::-webkit-input-placeholder {
    color: #fed7d7;
  }

  .xl\:focus\:placeholder-red-200:focus::-moz-placeholder {
    color: #fed7d7;
  }

  .xl\:focus\:placeholder-red-200:focus:-ms-input-placeholder {
    color: #fed7d7;
  }

  .xl\:focus\:placeholder-red-200:focus::-ms-input-placeholder {
    color: #fed7d7;
  }

  .xl\:focus\:placeholder-red-200:focus::placeholder {
    color: #fed7d7;
  }

  .xl\:focus\:placeholder-red-300:focus::-webkit-input-placeholder {
    color: #feb2b2;
  }

  .xl\:focus\:placeholder-red-300:focus::-moz-placeholder {
    color: #feb2b2;
  }

  .xl\:focus\:placeholder-red-300:focus:-ms-input-placeholder {
    color: #feb2b2;
  }

  .xl\:focus\:placeholder-red-300:focus::-ms-input-placeholder {
    color: #feb2b2;
  }

  .xl\:focus\:placeholder-red-300:focus::placeholder {
    color: #feb2b2;
  }

  .xl\:focus\:placeholder-red-400:focus::-webkit-input-placeholder {
    color: #fc8181;
  }

  .xl\:focus\:placeholder-red-400:focus::-moz-placeholder {
    color: #fc8181;
  }

  .xl\:focus\:placeholder-red-400:focus:-ms-input-placeholder {
    color: #fc8181;
  }

  .xl\:focus\:placeholder-red-400:focus::-ms-input-placeholder {
    color: #fc8181;
  }

  .xl\:focus\:placeholder-red-400:focus::placeholder {
    color: #fc8181;
  }

  .xl\:focus\:placeholder-red-500:focus::-webkit-input-placeholder {
    color: #f56565;
  }

  .xl\:focus\:placeholder-red-500:focus::-moz-placeholder {
    color: #f56565;
  }

  .xl\:focus\:placeholder-red-500:focus:-ms-input-placeholder {
    color: #f56565;
  }

  .xl\:focus\:placeholder-red-500:focus::-ms-input-placeholder {
    color: #f56565;
  }

  .xl\:focus\:placeholder-red-500:focus::placeholder {
    color: #f56565;
  }

  .xl\:focus\:placeholder-red-600:focus::-webkit-input-placeholder {
    color: #e53e3e;
  }

  .xl\:focus\:placeholder-red-600:focus::-moz-placeholder {
    color: #e53e3e;
  }

  .xl\:focus\:placeholder-red-600:focus:-ms-input-placeholder {
    color: #e53e3e;
  }

  .xl\:focus\:placeholder-red-600:focus::-ms-input-placeholder {
    color: #e53e3e;
  }

  .xl\:focus\:placeholder-red-600:focus::placeholder {
    color: #e53e3e;
  }

  .xl\:focus\:placeholder-red-700:focus::-webkit-input-placeholder {
    color: #c53030;
  }

  .xl\:focus\:placeholder-red-700:focus::-moz-placeholder {
    color: #c53030;
  }

  .xl\:focus\:placeholder-red-700:focus:-ms-input-placeholder {
    color: #c53030;
  }

  .xl\:focus\:placeholder-red-700:focus::-ms-input-placeholder {
    color: #c53030;
  }

  .xl\:focus\:placeholder-red-700:focus::placeholder {
    color: #c53030;
  }

  .xl\:focus\:placeholder-red-800:focus::-webkit-input-placeholder {
    color: #9b2c2c;
  }

  .xl\:focus\:placeholder-red-800:focus::-moz-placeholder {
    color: #9b2c2c;
  }

  .xl\:focus\:placeholder-red-800:focus:-ms-input-placeholder {
    color: #9b2c2c;
  }

  .xl\:focus\:placeholder-red-800:focus::-ms-input-placeholder {
    color: #9b2c2c;
  }

  .xl\:focus\:placeholder-red-800:focus::placeholder {
    color: #9b2c2c;
  }

  .xl\:focus\:placeholder-red-900:focus::-webkit-input-placeholder {
    color: #742a2a;
  }

  .xl\:focus\:placeholder-red-900:focus::-moz-placeholder {
    color: #742a2a;
  }

  .xl\:focus\:placeholder-red-900:focus:-ms-input-placeholder {
    color: #742a2a;
  }

  .xl\:focus\:placeholder-red-900:focus::-ms-input-placeholder {
    color: #742a2a;
  }

  .xl\:focus\:placeholder-red-900:focus::placeholder {
    color: #742a2a;
  }

  .xl\:focus\:placeholder-orange-100:focus::-webkit-input-placeholder {
    color: #fffaf0;
  }

  .xl\:focus\:placeholder-orange-100:focus::-moz-placeholder {
    color: #fffaf0;
  }

  .xl\:focus\:placeholder-orange-100:focus:-ms-input-placeholder {
    color: #fffaf0;
  }

  .xl\:focus\:placeholder-orange-100:focus::-ms-input-placeholder {
    color: #fffaf0;
  }

  .xl\:focus\:placeholder-orange-100:focus::placeholder {
    color: #fffaf0;
  }

  .xl\:focus\:placeholder-orange-200:focus::-webkit-input-placeholder {
    color: #feebc8;
  }

  .xl\:focus\:placeholder-orange-200:focus::-moz-placeholder {
    color: #feebc8;
  }

  .xl\:focus\:placeholder-orange-200:focus:-ms-input-placeholder {
    color: #feebc8;
  }

  .xl\:focus\:placeholder-orange-200:focus::-ms-input-placeholder {
    color: #feebc8;
  }

  .xl\:focus\:placeholder-orange-200:focus::placeholder {
    color: #feebc8;
  }

  .xl\:focus\:placeholder-orange-300:focus::-webkit-input-placeholder {
    color: #fbd38d;
  }

  .xl\:focus\:placeholder-orange-300:focus::-moz-placeholder {
    color: #fbd38d;
  }

  .xl\:focus\:placeholder-orange-300:focus:-ms-input-placeholder {
    color: #fbd38d;
  }

  .xl\:focus\:placeholder-orange-300:focus::-ms-input-placeholder {
    color: #fbd38d;
  }

  .xl\:focus\:placeholder-orange-300:focus::placeholder {
    color: #fbd38d;
  }

  .xl\:focus\:placeholder-orange-400:focus::-webkit-input-placeholder {
    color: #f6ad55;
  }

  .xl\:focus\:placeholder-orange-400:focus::-moz-placeholder {
    color: #f6ad55;
  }

  .xl\:focus\:placeholder-orange-400:focus:-ms-input-placeholder {
    color: #f6ad55;
  }

  .xl\:focus\:placeholder-orange-400:focus::-ms-input-placeholder {
    color: #f6ad55;
  }

  .xl\:focus\:placeholder-orange-400:focus::placeholder {
    color: #f6ad55;
  }

  .xl\:focus\:placeholder-orange-500:focus::-webkit-input-placeholder {
    color: #ed8936;
  }

  .xl\:focus\:placeholder-orange-500:focus::-moz-placeholder {
    color: #ed8936;
  }

  .xl\:focus\:placeholder-orange-500:focus:-ms-input-placeholder {
    color: #ed8936;
  }

  .xl\:focus\:placeholder-orange-500:focus::-ms-input-placeholder {
    color: #ed8936;
  }

  .xl\:focus\:placeholder-orange-500:focus::placeholder {
    color: #ed8936;
  }

  .xl\:focus\:placeholder-orange-600:focus::-webkit-input-placeholder {
    color: #dd6b20;
  }

  .xl\:focus\:placeholder-orange-600:focus::-moz-placeholder {
    color: #dd6b20;
  }

  .xl\:focus\:placeholder-orange-600:focus:-ms-input-placeholder {
    color: #dd6b20;
  }

  .xl\:focus\:placeholder-orange-600:focus::-ms-input-placeholder {
    color: #dd6b20;
  }

  .xl\:focus\:placeholder-orange-600:focus::placeholder {
    color: #dd6b20;
  }

  .xl\:focus\:placeholder-orange-700:focus::-webkit-input-placeholder {
    color: #c05621;
  }

  .xl\:focus\:placeholder-orange-700:focus::-moz-placeholder {
    color: #c05621;
  }

  .xl\:focus\:placeholder-orange-700:focus:-ms-input-placeholder {
    color: #c05621;
  }

  .xl\:focus\:placeholder-orange-700:focus::-ms-input-placeholder {
    color: #c05621;
  }

  .xl\:focus\:placeholder-orange-700:focus::placeholder {
    color: #c05621;
  }

  .xl\:focus\:placeholder-orange-800:focus::-webkit-input-placeholder {
    color: #9c4221;
  }

  .xl\:focus\:placeholder-orange-800:focus::-moz-placeholder {
    color: #9c4221;
  }

  .xl\:focus\:placeholder-orange-800:focus:-ms-input-placeholder {
    color: #9c4221;
  }

  .xl\:focus\:placeholder-orange-800:focus::-ms-input-placeholder {
    color: #9c4221;
  }

  .xl\:focus\:placeholder-orange-800:focus::placeholder {
    color: #9c4221;
  }

  .xl\:focus\:placeholder-orange-900:focus::-webkit-input-placeholder {
    color: #7b341e;
  }

  .xl\:focus\:placeholder-orange-900:focus::-moz-placeholder {
    color: #7b341e;
  }

  .xl\:focus\:placeholder-orange-900:focus:-ms-input-placeholder {
    color: #7b341e;
  }

  .xl\:focus\:placeholder-orange-900:focus::-ms-input-placeholder {
    color: #7b341e;
  }

  .xl\:focus\:placeholder-orange-900:focus::placeholder {
    color: #7b341e;
  }

  .xl\:focus\:placeholder-yellow-100:focus::-webkit-input-placeholder {
    color: #fffff0;
  }

  .xl\:focus\:placeholder-yellow-100:focus::-moz-placeholder {
    color: #fffff0;
  }

  .xl\:focus\:placeholder-yellow-100:focus:-ms-input-placeholder {
    color: #fffff0;
  }

  .xl\:focus\:placeholder-yellow-100:focus::-ms-input-placeholder {
    color: #fffff0;
  }

  .xl\:focus\:placeholder-yellow-100:focus::placeholder {
    color: #fffff0;
  }

  .xl\:focus\:placeholder-yellow-200:focus::-webkit-input-placeholder {
    color: #fefcbf;
  }

  .xl\:focus\:placeholder-yellow-200:focus::-moz-placeholder {
    color: #fefcbf;
  }

  .xl\:focus\:placeholder-yellow-200:focus:-ms-input-placeholder {
    color: #fefcbf;
  }

  .xl\:focus\:placeholder-yellow-200:focus::-ms-input-placeholder {
    color: #fefcbf;
  }

  .xl\:focus\:placeholder-yellow-200:focus::placeholder {
    color: #fefcbf;
  }

  .xl\:focus\:placeholder-yellow-300:focus::-webkit-input-placeholder {
    color: #faf089;
  }

  .xl\:focus\:placeholder-yellow-300:focus::-moz-placeholder {
    color: #faf089;
  }

  .xl\:focus\:placeholder-yellow-300:focus:-ms-input-placeholder {
    color: #faf089;
  }

  .xl\:focus\:placeholder-yellow-300:focus::-ms-input-placeholder {
    color: #faf089;
  }

  .xl\:focus\:placeholder-yellow-300:focus::placeholder {
    color: #faf089;
  }

  .xl\:focus\:placeholder-yellow-400:focus::-webkit-input-placeholder {
    color: #f6e05e;
  }

  .xl\:focus\:placeholder-yellow-400:focus::-moz-placeholder {
    color: #f6e05e;
  }

  .xl\:focus\:placeholder-yellow-400:focus:-ms-input-placeholder {
    color: #f6e05e;
  }

  .xl\:focus\:placeholder-yellow-400:focus::-ms-input-placeholder {
    color: #f6e05e;
  }

  .xl\:focus\:placeholder-yellow-400:focus::placeholder {
    color: #f6e05e;
  }

  .xl\:focus\:placeholder-yellow-500:focus::-webkit-input-placeholder {
    color: #ecc94b;
  }

  .xl\:focus\:placeholder-yellow-500:focus::-moz-placeholder {
    color: #ecc94b;
  }

  .xl\:focus\:placeholder-yellow-500:focus:-ms-input-placeholder {
    color: #ecc94b;
  }

  .xl\:focus\:placeholder-yellow-500:focus::-ms-input-placeholder {
    color: #ecc94b;
  }

  .xl\:focus\:placeholder-yellow-500:focus::placeholder {
    color: #ecc94b;
  }

  .xl\:focus\:placeholder-yellow-600:focus::-webkit-input-placeholder {
    color: #d69e2e;
  }

  .xl\:focus\:placeholder-yellow-600:focus::-moz-placeholder {
    color: #d69e2e;
  }

  .xl\:focus\:placeholder-yellow-600:focus:-ms-input-placeholder {
    color: #d69e2e;
  }

  .xl\:focus\:placeholder-yellow-600:focus::-ms-input-placeholder {
    color: #d69e2e;
  }

  .xl\:focus\:placeholder-yellow-600:focus::placeholder {
    color: #d69e2e;
  }

  .xl\:focus\:placeholder-yellow-700:focus::-webkit-input-placeholder {
    color: #b7791f;
  }

  .xl\:focus\:placeholder-yellow-700:focus::-moz-placeholder {
    color: #b7791f;
  }

  .xl\:focus\:placeholder-yellow-700:focus:-ms-input-placeholder {
    color: #b7791f;
  }

  .xl\:focus\:placeholder-yellow-700:focus::-ms-input-placeholder {
    color: #b7791f;
  }

  .xl\:focus\:placeholder-yellow-700:focus::placeholder {
    color: #b7791f;
  }

  .xl\:focus\:placeholder-yellow-800:focus::-webkit-input-placeholder {
    color: #975a16;
  }

  .xl\:focus\:placeholder-yellow-800:focus::-moz-placeholder {
    color: #975a16;
  }

  .xl\:focus\:placeholder-yellow-800:focus:-ms-input-placeholder {
    color: #975a16;
  }

  .xl\:focus\:placeholder-yellow-800:focus::-ms-input-placeholder {
    color: #975a16;
  }

  .xl\:focus\:placeholder-yellow-800:focus::placeholder {
    color: #975a16;
  }

  .xl\:focus\:placeholder-yellow-900:focus::-webkit-input-placeholder {
    color: #744210;
  }

  .xl\:focus\:placeholder-yellow-900:focus::-moz-placeholder {
    color: #744210;
  }

  .xl\:focus\:placeholder-yellow-900:focus:-ms-input-placeholder {
    color: #744210;
  }

  .xl\:focus\:placeholder-yellow-900:focus::-ms-input-placeholder {
    color: #744210;
  }

  .xl\:focus\:placeholder-yellow-900:focus::placeholder {
    color: #744210;
  }

  .xl\:focus\:placeholder-green-100:focus::-webkit-input-placeholder {
    color: #f0fff4;
  }

  .xl\:focus\:placeholder-green-100:focus::-moz-placeholder {
    color: #f0fff4;
  }

  .xl\:focus\:placeholder-green-100:focus:-ms-input-placeholder {
    color: #f0fff4;
  }

  .xl\:focus\:placeholder-green-100:focus::-ms-input-placeholder {
    color: #f0fff4;
  }

  .xl\:focus\:placeholder-green-100:focus::placeholder {
    color: #f0fff4;
  }

  .xl\:focus\:placeholder-green-200:focus::-webkit-input-placeholder {
    color: #c6f6d5;
  }

  .xl\:focus\:placeholder-green-200:focus::-moz-placeholder {
    color: #c6f6d5;
  }

  .xl\:focus\:placeholder-green-200:focus:-ms-input-placeholder {
    color: #c6f6d5;
  }

  .xl\:focus\:placeholder-green-200:focus::-ms-input-placeholder {
    color: #c6f6d5;
  }

  .xl\:focus\:placeholder-green-200:focus::placeholder {
    color: #c6f6d5;
  }

  .xl\:focus\:placeholder-green-300:focus::-webkit-input-placeholder {
    color: #9ae6b4;
  }

  .xl\:focus\:placeholder-green-300:focus::-moz-placeholder {
    color: #9ae6b4;
  }

  .xl\:focus\:placeholder-green-300:focus:-ms-input-placeholder {
    color: #9ae6b4;
  }

  .xl\:focus\:placeholder-green-300:focus::-ms-input-placeholder {
    color: #9ae6b4;
  }

  .xl\:focus\:placeholder-green-300:focus::placeholder {
    color: #9ae6b4;
  }

  .xl\:focus\:placeholder-green-400:focus::-webkit-input-placeholder {
    color: #68d391;
  }

  .xl\:focus\:placeholder-green-400:focus::-moz-placeholder {
    color: #68d391;
  }

  .xl\:focus\:placeholder-green-400:focus:-ms-input-placeholder {
    color: #68d391;
  }

  .xl\:focus\:placeholder-green-400:focus::-ms-input-placeholder {
    color: #68d391;
  }

  .xl\:focus\:placeholder-green-400:focus::placeholder {
    color: #68d391;
  }

  .xl\:focus\:placeholder-green-500:focus::-webkit-input-placeholder {
    color: #48bb78;
  }

  .xl\:focus\:placeholder-green-500:focus::-moz-placeholder {
    color: #48bb78;
  }

  .xl\:focus\:placeholder-green-500:focus:-ms-input-placeholder {
    color: #48bb78;
  }

  .xl\:focus\:placeholder-green-500:focus::-ms-input-placeholder {
    color: #48bb78;
  }

  .xl\:focus\:placeholder-green-500:focus::placeholder {
    color: #48bb78;
  }

  .xl\:focus\:placeholder-green-600:focus::-webkit-input-placeholder {
    color: #38a169;
  }

  .xl\:focus\:placeholder-green-600:focus::-moz-placeholder {
    color: #38a169;
  }

  .xl\:focus\:placeholder-green-600:focus:-ms-input-placeholder {
    color: #38a169;
  }

  .xl\:focus\:placeholder-green-600:focus::-ms-input-placeholder {
    color: #38a169;
  }

  .xl\:focus\:placeholder-green-600:focus::placeholder {
    color: #38a169;
  }

  .xl\:focus\:placeholder-green-700:focus::-webkit-input-placeholder {
    color: #2f855a;
  }

  .xl\:focus\:placeholder-green-700:focus::-moz-placeholder {
    color: #2f855a;
  }

  .xl\:focus\:placeholder-green-700:focus:-ms-input-placeholder {
    color: #2f855a;
  }

  .xl\:focus\:placeholder-green-700:focus::-ms-input-placeholder {
    color: #2f855a;
  }

  .xl\:focus\:placeholder-green-700:focus::placeholder {
    color: #2f855a;
  }

  .xl\:focus\:placeholder-green-800:focus::-webkit-input-placeholder {
    color: #276749;
  }

  .xl\:focus\:placeholder-green-800:focus::-moz-placeholder {
    color: #276749;
  }

  .xl\:focus\:placeholder-green-800:focus:-ms-input-placeholder {
    color: #276749;
  }

  .xl\:focus\:placeholder-green-800:focus::-ms-input-placeholder {
    color: #276749;
  }

  .xl\:focus\:placeholder-green-800:focus::placeholder {
    color: #276749;
  }

  .xl\:focus\:placeholder-green-900:focus::-webkit-input-placeholder {
    color: #22543d;
  }

  .xl\:focus\:placeholder-green-900:focus::-moz-placeholder {
    color: #22543d;
  }

  .xl\:focus\:placeholder-green-900:focus:-ms-input-placeholder {
    color: #22543d;
  }

  .xl\:focus\:placeholder-green-900:focus::-ms-input-placeholder {
    color: #22543d;
  }

  .xl\:focus\:placeholder-green-900:focus::placeholder {
    color: #22543d;
  }

  .xl\:focus\:placeholder-teal-100:focus::-webkit-input-placeholder {
    color: #e6fffa;
  }

  .xl\:focus\:placeholder-teal-100:focus::-moz-placeholder {
    color: #e6fffa;
  }

  .xl\:focus\:placeholder-teal-100:focus:-ms-input-placeholder {
    color: #e6fffa;
  }

  .xl\:focus\:placeholder-teal-100:focus::-ms-input-placeholder {
    color: #e6fffa;
  }

  .xl\:focus\:placeholder-teal-100:focus::placeholder {
    color: #e6fffa;
  }

  .xl\:focus\:placeholder-teal-200:focus::-webkit-input-placeholder {
    color: #b2f5ea;
  }

  .xl\:focus\:placeholder-teal-200:focus::-moz-placeholder {
    color: #b2f5ea;
  }

  .xl\:focus\:placeholder-teal-200:focus:-ms-input-placeholder {
    color: #b2f5ea;
  }

  .xl\:focus\:placeholder-teal-200:focus::-ms-input-placeholder {
    color: #b2f5ea;
  }

  .xl\:focus\:placeholder-teal-200:focus::placeholder {
    color: #b2f5ea;
  }

  .xl\:focus\:placeholder-teal-300:focus::-webkit-input-placeholder {
    color: #81e6d9;
  }

  .xl\:focus\:placeholder-teal-300:focus::-moz-placeholder {
    color: #81e6d9;
  }

  .xl\:focus\:placeholder-teal-300:focus:-ms-input-placeholder {
    color: #81e6d9;
  }

  .xl\:focus\:placeholder-teal-300:focus::-ms-input-placeholder {
    color: #81e6d9;
  }

  .xl\:focus\:placeholder-teal-300:focus::placeholder {
    color: #81e6d9;
  }

  .xl\:focus\:placeholder-teal-400:focus::-webkit-input-placeholder {
    color: #4fd1c5;
  }

  .xl\:focus\:placeholder-teal-400:focus::-moz-placeholder {
    color: #4fd1c5;
  }

  .xl\:focus\:placeholder-teal-400:focus:-ms-input-placeholder {
    color: #4fd1c5;
  }

  .xl\:focus\:placeholder-teal-400:focus::-ms-input-placeholder {
    color: #4fd1c5;
  }

  .xl\:focus\:placeholder-teal-400:focus::placeholder {
    color: #4fd1c5;
  }

  .xl\:focus\:placeholder-teal-500:focus::-webkit-input-placeholder {
    color: #38b2ac;
  }

  .xl\:focus\:placeholder-teal-500:focus::-moz-placeholder {
    color: #38b2ac;
  }

  .xl\:focus\:placeholder-teal-500:focus:-ms-input-placeholder {
    color: #38b2ac;
  }

  .xl\:focus\:placeholder-teal-500:focus::-ms-input-placeholder {
    color: #38b2ac;
  }

  .xl\:focus\:placeholder-teal-500:focus::placeholder {
    color: #38b2ac;
  }

  .xl\:focus\:placeholder-teal-600:focus::-webkit-input-placeholder {
    color: #319795;
  }

  .xl\:focus\:placeholder-teal-600:focus::-moz-placeholder {
    color: #319795;
  }

  .xl\:focus\:placeholder-teal-600:focus:-ms-input-placeholder {
    color: #319795;
  }

  .xl\:focus\:placeholder-teal-600:focus::-ms-input-placeholder {
    color: #319795;
  }

  .xl\:focus\:placeholder-teal-600:focus::placeholder {
    color: #319795;
  }

  .xl\:focus\:placeholder-teal-700:focus::-webkit-input-placeholder {
    color: #2c7a7b;
  }

  .xl\:focus\:placeholder-teal-700:focus::-moz-placeholder {
    color: #2c7a7b;
  }

  .xl\:focus\:placeholder-teal-700:focus:-ms-input-placeholder {
    color: #2c7a7b;
  }

  .xl\:focus\:placeholder-teal-700:focus::-ms-input-placeholder {
    color: #2c7a7b;
  }

  .xl\:focus\:placeholder-teal-700:focus::placeholder {
    color: #2c7a7b;
  }

  .xl\:focus\:placeholder-teal-800:focus::-webkit-input-placeholder {
    color: #285e61;
  }

  .xl\:focus\:placeholder-teal-800:focus::-moz-placeholder {
    color: #285e61;
  }

  .xl\:focus\:placeholder-teal-800:focus:-ms-input-placeholder {
    color: #285e61;
  }

  .xl\:focus\:placeholder-teal-800:focus::-ms-input-placeholder {
    color: #285e61;
  }

  .xl\:focus\:placeholder-teal-800:focus::placeholder {
    color: #285e61;
  }

  .xl\:focus\:placeholder-teal-900:focus::-webkit-input-placeholder {
    color: #234e52;
  }

  .xl\:focus\:placeholder-teal-900:focus::-moz-placeholder {
    color: #234e52;
  }

  .xl\:focus\:placeholder-teal-900:focus:-ms-input-placeholder {
    color: #234e52;
  }

  .xl\:focus\:placeholder-teal-900:focus::-ms-input-placeholder {
    color: #234e52;
  }

  .xl\:focus\:placeholder-teal-900:focus::placeholder {
    color: #234e52;
  }

  .xl\:focus\:placeholder-blue-100:focus::-webkit-input-placeholder {
    color: #ebf8ff;
  }

  .xl\:focus\:placeholder-blue-100:focus::-moz-placeholder {
    color: #ebf8ff;
  }

  .xl\:focus\:placeholder-blue-100:focus:-ms-input-placeholder {
    color: #ebf8ff;
  }

  .xl\:focus\:placeholder-blue-100:focus::-ms-input-placeholder {
    color: #ebf8ff;
  }

  .xl\:focus\:placeholder-blue-100:focus::placeholder {
    color: #ebf8ff;
  }

  .xl\:focus\:placeholder-blue-200:focus::-webkit-input-placeholder {
    color: #bee3f8;
  }

  .xl\:focus\:placeholder-blue-200:focus::-moz-placeholder {
    color: #bee3f8;
  }

  .xl\:focus\:placeholder-blue-200:focus:-ms-input-placeholder {
    color: #bee3f8;
  }

  .xl\:focus\:placeholder-blue-200:focus::-ms-input-placeholder {
    color: #bee3f8;
  }

  .xl\:focus\:placeholder-blue-200:focus::placeholder {
    color: #bee3f8;
  }

  .xl\:focus\:placeholder-blue-300:focus::-webkit-input-placeholder {
    color: #90cdf4;
  }

  .xl\:focus\:placeholder-blue-300:focus::-moz-placeholder {
    color: #90cdf4;
  }

  .xl\:focus\:placeholder-blue-300:focus:-ms-input-placeholder {
    color: #90cdf4;
  }

  .xl\:focus\:placeholder-blue-300:focus::-ms-input-placeholder {
    color: #90cdf4;
  }

  .xl\:focus\:placeholder-blue-300:focus::placeholder {
    color: #90cdf4;
  }

  .xl\:focus\:placeholder-blue-400:focus::-webkit-input-placeholder {
    color: #63b3ed;
  }

  .xl\:focus\:placeholder-blue-400:focus::-moz-placeholder {
    color: #63b3ed;
  }

  .xl\:focus\:placeholder-blue-400:focus:-ms-input-placeholder {
    color: #63b3ed;
  }

  .xl\:focus\:placeholder-blue-400:focus::-ms-input-placeholder {
    color: #63b3ed;
  }

  .xl\:focus\:placeholder-blue-400:focus::placeholder {
    color: #63b3ed;
  }

  .xl\:focus\:placeholder-blue-500:focus::-webkit-input-placeholder {
    color: #4299e1;
  }

  .xl\:focus\:placeholder-blue-500:focus::-moz-placeholder {
    color: #4299e1;
  }

  .xl\:focus\:placeholder-blue-500:focus:-ms-input-placeholder {
    color: #4299e1;
  }

  .xl\:focus\:placeholder-blue-500:focus::-ms-input-placeholder {
    color: #4299e1;
  }

  .xl\:focus\:placeholder-blue-500:focus::placeholder {
    color: #4299e1;
  }

  .xl\:focus\:placeholder-blue-600:focus::-webkit-input-placeholder {
    color: #3182ce;
  }

  .xl\:focus\:placeholder-blue-600:focus::-moz-placeholder {
    color: #3182ce;
  }

  .xl\:focus\:placeholder-blue-600:focus:-ms-input-placeholder {
    color: #3182ce;
  }

  .xl\:focus\:placeholder-blue-600:focus::-ms-input-placeholder {
    color: #3182ce;
  }

  .xl\:focus\:placeholder-blue-600:focus::placeholder {
    color: #3182ce;
  }

  .xl\:focus\:placeholder-blue-700:focus::-webkit-input-placeholder {
    color: #2b6cb0;
  }

  .xl\:focus\:placeholder-blue-700:focus::-moz-placeholder {
    color: #2b6cb0;
  }

  .xl\:focus\:placeholder-blue-700:focus:-ms-input-placeholder {
    color: #2b6cb0;
  }

  .xl\:focus\:placeholder-blue-700:focus::-ms-input-placeholder {
    color: #2b6cb0;
  }

  .xl\:focus\:placeholder-blue-700:focus::placeholder {
    color: #2b6cb0;
  }

  .xl\:focus\:placeholder-blue-800:focus::-webkit-input-placeholder {
    color: #2c5282;
  }

  .xl\:focus\:placeholder-blue-800:focus::-moz-placeholder {
    color: #2c5282;
  }

  .xl\:focus\:placeholder-blue-800:focus:-ms-input-placeholder {
    color: #2c5282;
  }

  .xl\:focus\:placeholder-blue-800:focus::-ms-input-placeholder {
    color: #2c5282;
  }

  .xl\:focus\:placeholder-blue-800:focus::placeholder {
    color: #2c5282;
  }

  .xl\:focus\:placeholder-blue-900:focus::-webkit-input-placeholder {
    color: #2a4365;
  }

  .xl\:focus\:placeholder-blue-900:focus::-moz-placeholder {
    color: #2a4365;
  }

  .xl\:focus\:placeholder-blue-900:focus:-ms-input-placeholder {
    color: #2a4365;
  }

  .xl\:focus\:placeholder-blue-900:focus::-ms-input-placeholder {
    color: #2a4365;
  }

  .xl\:focus\:placeholder-blue-900:focus::placeholder {
    color: #2a4365;
  }

  .xl\:focus\:placeholder-indigo-100:focus::-webkit-input-placeholder {
    color: #ebf4ff;
  }

  .xl\:focus\:placeholder-indigo-100:focus::-moz-placeholder {
    color: #ebf4ff;
  }

  .xl\:focus\:placeholder-indigo-100:focus:-ms-input-placeholder {
    color: #ebf4ff;
  }

  .xl\:focus\:placeholder-indigo-100:focus::-ms-input-placeholder {
    color: #ebf4ff;
  }

  .xl\:focus\:placeholder-indigo-100:focus::placeholder {
    color: #ebf4ff;
  }

  .xl\:focus\:placeholder-indigo-200:focus::-webkit-input-placeholder {
    color: #c3dafe;
  }

  .xl\:focus\:placeholder-indigo-200:focus::-moz-placeholder {
    color: #c3dafe;
  }

  .xl\:focus\:placeholder-indigo-200:focus:-ms-input-placeholder {
    color: #c3dafe;
  }

  .xl\:focus\:placeholder-indigo-200:focus::-ms-input-placeholder {
    color: #c3dafe;
  }

  .xl\:focus\:placeholder-indigo-200:focus::placeholder {
    color: #c3dafe;
  }

  .xl\:focus\:placeholder-indigo-300:focus::-webkit-input-placeholder {
    color: #a3bffa;
  }

  .xl\:focus\:placeholder-indigo-300:focus::-moz-placeholder {
    color: #a3bffa;
  }

  .xl\:focus\:placeholder-indigo-300:focus:-ms-input-placeholder {
    color: #a3bffa;
  }

  .xl\:focus\:placeholder-indigo-300:focus::-ms-input-placeholder {
    color: #a3bffa;
  }

  .xl\:focus\:placeholder-indigo-300:focus::placeholder {
    color: #a3bffa;
  }

  .xl\:focus\:placeholder-indigo-400:focus::-webkit-input-placeholder {
    color: #7f9cf5;
  }

  .xl\:focus\:placeholder-indigo-400:focus::-moz-placeholder {
    color: #7f9cf5;
  }

  .xl\:focus\:placeholder-indigo-400:focus:-ms-input-placeholder {
    color: #7f9cf5;
  }

  .xl\:focus\:placeholder-indigo-400:focus::-ms-input-placeholder {
    color: #7f9cf5;
  }

  .xl\:focus\:placeholder-indigo-400:focus::placeholder {
    color: #7f9cf5;
  }

  .xl\:focus\:placeholder-indigo-500:focus::-webkit-input-placeholder {
    color: #667eea;
  }

  .xl\:focus\:placeholder-indigo-500:focus::-moz-placeholder {
    color: #667eea;
  }

  .xl\:focus\:placeholder-indigo-500:focus:-ms-input-placeholder {
    color: #667eea;
  }

  .xl\:focus\:placeholder-indigo-500:focus::-ms-input-placeholder {
    color: #667eea;
  }

  .xl\:focus\:placeholder-indigo-500:focus::placeholder {
    color: #667eea;
  }

  .xl\:focus\:placeholder-indigo-600:focus::-webkit-input-placeholder {
    color: #5a67d8;
  }

  .xl\:focus\:placeholder-indigo-600:focus::-moz-placeholder {
    color: #5a67d8;
  }

  .xl\:focus\:placeholder-indigo-600:focus:-ms-input-placeholder {
    color: #5a67d8;
  }

  .xl\:focus\:placeholder-indigo-600:focus::-ms-input-placeholder {
    color: #5a67d8;
  }

  .xl\:focus\:placeholder-indigo-600:focus::placeholder {
    color: #5a67d8;
  }

  .xl\:focus\:placeholder-indigo-700:focus::-webkit-input-placeholder {
    color: #4c51bf;
  }

  .xl\:focus\:placeholder-indigo-700:focus::-moz-placeholder {
    color: #4c51bf;
  }

  .xl\:focus\:placeholder-indigo-700:focus:-ms-input-placeholder {
    color: #4c51bf;
  }

  .xl\:focus\:placeholder-indigo-700:focus::-ms-input-placeholder {
    color: #4c51bf;
  }

  .xl\:focus\:placeholder-indigo-700:focus::placeholder {
    color: #4c51bf;
  }

  .xl\:focus\:placeholder-indigo-800:focus::-webkit-input-placeholder {
    color: #434190;
  }

  .xl\:focus\:placeholder-indigo-800:focus::-moz-placeholder {
    color: #434190;
  }

  .xl\:focus\:placeholder-indigo-800:focus:-ms-input-placeholder {
    color: #434190;
  }

  .xl\:focus\:placeholder-indigo-800:focus::-ms-input-placeholder {
    color: #434190;
  }

  .xl\:focus\:placeholder-indigo-800:focus::placeholder {
    color: #434190;
  }

  .xl\:focus\:placeholder-indigo-900:focus::-webkit-input-placeholder {
    color: #3c366b;
  }

  .xl\:focus\:placeholder-indigo-900:focus::-moz-placeholder {
    color: #3c366b;
  }

  .xl\:focus\:placeholder-indigo-900:focus:-ms-input-placeholder {
    color: #3c366b;
  }

  .xl\:focus\:placeholder-indigo-900:focus::-ms-input-placeholder {
    color: #3c366b;
  }

  .xl\:focus\:placeholder-indigo-900:focus::placeholder {
    color: #3c366b;
  }

  .xl\:focus\:placeholder-purple-100:focus::-webkit-input-placeholder {
    color: #faf5ff;
  }

  .xl\:focus\:placeholder-purple-100:focus::-moz-placeholder {
    color: #faf5ff;
  }

  .xl\:focus\:placeholder-purple-100:focus:-ms-input-placeholder {
    color: #faf5ff;
  }

  .xl\:focus\:placeholder-purple-100:focus::-ms-input-placeholder {
    color: #faf5ff;
  }

  .xl\:focus\:placeholder-purple-100:focus::placeholder {
    color: #faf5ff;
  }

  .xl\:focus\:placeholder-purple-200:focus::-webkit-input-placeholder {
    color: #e9d8fd;
  }

  .xl\:focus\:placeholder-purple-200:focus::-moz-placeholder {
    color: #e9d8fd;
  }

  .xl\:focus\:placeholder-purple-200:focus:-ms-input-placeholder {
    color: #e9d8fd;
  }

  .xl\:focus\:placeholder-purple-200:focus::-ms-input-placeholder {
    color: #e9d8fd;
  }

  .xl\:focus\:placeholder-purple-200:focus::placeholder {
    color: #e9d8fd;
  }

  .xl\:focus\:placeholder-purple-300:focus::-webkit-input-placeholder {
    color: #d6bcfa;
  }

  .xl\:focus\:placeholder-purple-300:focus::-moz-placeholder {
    color: #d6bcfa;
  }

  .xl\:focus\:placeholder-purple-300:focus:-ms-input-placeholder {
    color: #d6bcfa;
  }

  .xl\:focus\:placeholder-purple-300:focus::-ms-input-placeholder {
    color: #d6bcfa;
  }

  .xl\:focus\:placeholder-purple-300:focus::placeholder {
    color: #d6bcfa;
  }

  .xl\:focus\:placeholder-purple-400:focus::-webkit-input-placeholder {
    color: #b794f4;
  }

  .xl\:focus\:placeholder-purple-400:focus::-moz-placeholder {
    color: #b794f4;
  }

  .xl\:focus\:placeholder-purple-400:focus:-ms-input-placeholder {
    color: #b794f4;
  }

  .xl\:focus\:placeholder-purple-400:focus::-ms-input-placeholder {
    color: #b794f4;
  }

  .xl\:focus\:placeholder-purple-400:focus::placeholder {
    color: #b794f4;
  }

  .xl\:focus\:placeholder-purple-500:focus::-webkit-input-placeholder {
    color: #9f7aea;
  }

  .xl\:focus\:placeholder-purple-500:focus::-moz-placeholder {
    color: #9f7aea;
  }

  .xl\:focus\:placeholder-purple-500:focus:-ms-input-placeholder {
    color: #9f7aea;
  }

  .xl\:focus\:placeholder-purple-500:focus::-ms-input-placeholder {
    color: #9f7aea;
  }

  .xl\:focus\:placeholder-purple-500:focus::placeholder {
    color: #9f7aea;
  }

  .xl\:focus\:placeholder-purple-600:focus::-webkit-input-placeholder {
    color: #805ad5;
  }

  .xl\:focus\:placeholder-purple-600:focus::-moz-placeholder {
    color: #805ad5;
  }

  .xl\:focus\:placeholder-purple-600:focus:-ms-input-placeholder {
    color: #805ad5;
  }

  .xl\:focus\:placeholder-purple-600:focus::-ms-input-placeholder {
    color: #805ad5;
  }

  .xl\:focus\:placeholder-purple-600:focus::placeholder {
    color: #805ad5;
  }

  .xl\:focus\:placeholder-purple-700:focus::-webkit-input-placeholder {
    color: #6b46c1;
  }

  .xl\:focus\:placeholder-purple-700:focus::-moz-placeholder {
    color: #6b46c1;
  }

  .xl\:focus\:placeholder-purple-700:focus:-ms-input-placeholder {
    color: #6b46c1;
  }

  .xl\:focus\:placeholder-purple-700:focus::-ms-input-placeholder {
    color: #6b46c1;
  }

  .xl\:focus\:placeholder-purple-700:focus::placeholder {
    color: #6b46c1;
  }

  .xl\:focus\:placeholder-purple-800:focus::-webkit-input-placeholder {
    color: #553c9a;
  }

  .xl\:focus\:placeholder-purple-800:focus::-moz-placeholder {
    color: #553c9a;
  }

  .xl\:focus\:placeholder-purple-800:focus:-ms-input-placeholder {
    color: #553c9a;
  }

  .xl\:focus\:placeholder-purple-800:focus::-ms-input-placeholder {
    color: #553c9a;
  }

  .xl\:focus\:placeholder-purple-800:focus::placeholder {
    color: #553c9a;
  }

  .xl\:focus\:placeholder-purple-900:focus::-webkit-input-placeholder {
    color: #44337a;
  }

  .xl\:focus\:placeholder-purple-900:focus::-moz-placeholder {
    color: #44337a;
  }

  .xl\:focus\:placeholder-purple-900:focus:-ms-input-placeholder {
    color: #44337a;
  }

  .xl\:focus\:placeholder-purple-900:focus::-ms-input-placeholder {
    color: #44337a;
  }

  .xl\:focus\:placeholder-purple-900:focus::placeholder {
    color: #44337a;
  }

  .xl\:focus\:placeholder-pink-100:focus::-webkit-input-placeholder {
    color: #fff5f7;
  }

  .xl\:focus\:placeholder-pink-100:focus::-moz-placeholder {
    color: #fff5f7;
  }

  .xl\:focus\:placeholder-pink-100:focus:-ms-input-placeholder {
    color: #fff5f7;
  }

  .xl\:focus\:placeholder-pink-100:focus::-ms-input-placeholder {
    color: #fff5f7;
  }

  .xl\:focus\:placeholder-pink-100:focus::placeholder {
    color: #fff5f7;
  }

  .xl\:focus\:placeholder-pink-200:focus::-webkit-input-placeholder {
    color: #fed7e2;
  }

  .xl\:focus\:placeholder-pink-200:focus::-moz-placeholder {
    color: #fed7e2;
  }

  .xl\:focus\:placeholder-pink-200:focus:-ms-input-placeholder {
    color: #fed7e2;
  }

  .xl\:focus\:placeholder-pink-200:focus::-ms-input-placeholder {
    color: #fed7e2;
  }

  .xl\:focus\:placeholder-pink-200:focus::placeholder {
    color: #fed7e2;
  }

  .xl\:focus\:placeholder-pink-300:focus::-webkit-input-placeholder {
    color: #fbb6ce;
  }

  .xl\:focus\:placeholder-pink-300:focus::-moz-placeholder {
    color: #fbb6ce;
  }

  .xl\:focus\:placeholder-pink-300:focus:-ms-input-placeholder {
    color: #fbb6ce;
  }

  .xl\:focus\:placeholder-pink-300:focus::-ms-input-placeholder {
    color: #fbb6ce;
  }

  .xl\:focus\:placeholder-pink-300:focus::placeholder {
    color: #fbb6ce;
  }

  .xl\:focus\:placeholder-pink-400:focus::-webkit-input-placeholder {
    color: #f687b3;
  }

  .xl\:focus\:placeholder-pink-400:focus::-moz-placeholder {
    color: #f687b3;
  }

  .xl\:focus\:placeholder-pink-400:focus:-ms-input-placeholder {
    color: #f687b3;
  }

  .xl\:focus\:placeholder-pink-400:focus::-ms-input-placeholder {
    color: #f687b3;
  }

  .xl\:focus\:placeholder-pink-400:focus::placeholder {
    color: #f687b3;
  }

  .xl\:focus\:placeholder-pink-500:focus::-webkit-input-placeholder {
    color: #ed64a6;
  }

  .xl\:focus\:placeholder-pink-500:focus::-moz-placeholder {
    color: #ed64a6;
  }

  .xl\:focus\:placeholder-pink-500:focus:-ms-input-placeholder {
    color: #ed64a6;
  }

  .xl\:focus\:placeholder-pink-500:focus::-ms-input-placeholder {
    color: #ed64a6;
  }

  .xl\:focus\:placeholder-pink-500:focus::placeholder {
    color: #ed64a6;
  }

  .xl\:focus\:placeholder-pink-600:focus::-webkit-input-placeholder {
    color: #d53f8c;
  }

  .xl\:focus\:placeholder-pink-600:focus::-moz-placeholder {
    color: #d53f8c;
  }

  .xl\:focus\:placeholder-pink-600:focus:-ms-input-placeholder {
    color: #d53f8c;
  }

  .xl\:focus\:placeholder-pink-600:focus::-ms-input-placeholder {
    color: #d53f8c;
  }

  .xl\:focus\:placeholder-pink-600:focus::placeholder {
    color: #d53f8c;
  }

  .xl\:focus\:placeholder-pink-700:focus::-webkit-input-placeholder {
    color: #b83280;
  }

  .xl\:focus\:placeholder-pink-700:focus::-moz-placeholder {
    color: #b83280;
  }

  .xl\:focus\:placeholder-pink-700:focus:-ms-input-placeholder {
    color: #b83280;
  }

  .xl\:focus\:placeholder-pink-700:focus::-ms-input-placeholder {
    color: #b83280;
  }

  .xl\:focus\:placeholder-pink-700:focus::placeholder {
    color: #b83280;
  }

  .xl\:focus\:placeholder-pink-800:focus::-webkit-input-placeholder {
    color: #97266d;
  }

  .xl\:focus\:placeholder-pink-800:focus::-moz-placeholder {
    color: #97266d;
  }

  .xl\:focus\:placeholder-pink-800:focus:-ms-input-placeholder {
    color: #97266d;
  }

  .xl\:focus\:placeholder-pink-800:focus::-ms-input-placeholder {
    color: #97266d;
  }

  .xl\:focus\:placeholder-pink-800:focus::placeholder {
    color: #97266d;
  }

  .xl\:focus\:placeholder-pink-900:focus::-webkit-input-placeholder {
    color: #702459;
  }

  .xl\:focus\:placeholder-pink-900:focus::-moz-placeholder {
    color: #702459;
  }

  .xl\:focus\:placeholder-pink-900:focus:-ms-input-placeholder {
    color: #702459;
  }

  .xl\:focus\:placeholder-pink-900:focus::-ms-input-placeholder {
    color: #702459;
  }

  .xl\:focus\:placeholder-pink-900:focus::placeholder {
    color: #702459;
  }

  .xl\:pointer-events-none {
    pointer-events: none;
  }

  .xl\:pointer-events-auto {
    pointer-events: auto;
  }

  .xl\:static {
    position: static;
  }

  .xl\:fixed {
    position: fixed;
  }

  .xl\:absolute {
    position: absolute;
  }

  .xl\:relative {
    position: relative;
  }

  .xl\:sticky {
    position: -webkit-sticky;
    position: sticky;
  }

  .xl\:inset-0 {
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
  }

  .xl\:inset-auto {
    top: auto;
    right: auto;
    bottom: auto;
    left: auto;
  }

  .xl\:inset-y-0 {
    top: 0;
    bottom: 0;
  }

  .xl\:inset-x-0 {
    right: 0;
    left: 0;
  }

  .xl\:inset-y-auto {
    top: auto;
    bottom: auto;
  }

  .xl\:inset-x-auto {
    right: auto;
    left: auto;
  }

  .xl\:top-0 {
    top: 0;
  }

  .xl\:right-0 {
    right: 0;
  }

  .xl\:bottom-0 {
    bottom: 0;
  }

  .xl\:left-0 {
    left: 0;
  }

  .xl\:top-auto {
    top: auto;
  }

  .xl\:right-auto {
    right: auto;
  }

  .xl\:bottom-auto {
    bottom: auto;
  }

  .xl\:left-auto {
    left: auto;
  }

  .xl\:resize-none {
    resize: none;
  }

  .xl\:resize-y {
    resize: vertical;
  }

  .xl\:resize-x {
    resize: horizontal;
  }

  .xl\:resize {
    resize: both;
  }

  .xl\:shadow {
    box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06);
  }

  .xl\:shadow-md {
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
  }

  .xl\:shadow-lg {
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
  }

  .xl\:shadow-xl {
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
  }

  .xl\:shadow-2xl {
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
  }

  .xl\:shadow-inner {
    box-shadow: inset 0 2px 4px 0 rgba(0, 0, 0, 0.06);
  }

  .xl\:shadow-outline {
    box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.5);
  }

  .xl\:shadow-none {
    box-shadow: none;
  }

  .xl\:hover\:shadow:hover {
    box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06);
  }

  .xl\:hover\:shadow-md:hover {
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
  }

  .xl\:hover\:shadow-lg:hover {
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
  }

  .xl\:hover\:shadow-xl:hover {
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
  }

  .xl\:hover\:shadow-2xl:hover {
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
  }

  .xl\:hover\:shadow-inner:hover {
    box-shadow: inset 0 2px 4px 0 rgba(0, 0, 0, 0.06);
  }

  .xl\:hover\:shadow-outline:hover {
    box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.5);
  }

  .xl\:hover\:shadow-none:hover {
    box-shadow: none;
  }

  .xl\:focus\:shadow:focus {
    box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06);
  }

  .xl\:focus\:shadow-md:focus {
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
  }

  .xl\:focus\:shadow-lg:focus {
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
  }

  .xl\:focus\:shadow-xl:focus {
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
  }

  .xl\:focus\:shadow-2xl:focus {
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
  }

  .xl\:focus\:shadow-inner:focus {
    box-shadow: inset 0 2px 4px 0 rgba(0, 0, 0, 0.06);
  }

  .xl\:focus\:shadow-outline:focus {
    box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.5);
  }

  .xl\:focus\:shadow-none:focus {
    box-shadow: none;
  }

  .xl\:fill-current {
    fill: currentColor;
  }

  .xl\:stroke-current {
    stroke: currentColor;
  }

  .xl\:table-auto {
    table-layout: auto;
  }

  .xl\:table-fixed {
    table-layout: fixed;
  }

  .xl\:text-left {
    text-align: left;
  }

  .xl\:text-center {
    text-align: center;
  }

  .xl\:text-right {
    text-align: right;
  }

  .xl\:text-justify {
    text-align: justify;
  }

  .xl\:text-transparent {
    color: transparent;
  }

  .xl\:text-black {
    color: #000;
  }

  .xl\:text-white {
    color: #fff;
  }

  .xl\:text-gray-100 {
    color: #f7fafc;
  }

  .xl\:text-gray-200 {
    color: #edf2f7;
  }

  .xl\:text-gray-300 {
    color: #e2e8f0;
  }

  .xl\:text-gray-400 {
    color: #cbd5e0;
  }

  .xl\:text-gray-500 {
    color: #a0aec0;
  }

  .xl\:text-gray-600 {
    color: #718096;
  }

  .xl\:text-gray-700 {
    color: #4a5568;
  }

  .xl\:text-gray-800 {
    color: #2d3748;
  }

  .xl\:text-gray-900 {
    color: #1a202c;
  }

  .xl\:text-red-100 {
    color: #fff5f5;
  }

  .xl\:text-red-200 {
    color: #fed7d7;
  }

  .xl\:text-red-300 {
    color: #feb2b2;
  }

  .xl\:text-red-400 {
    color: #fc8181;
  }

  .xl\:text-red-500 {
    color: #f56565;
  }

  .xl\:text-red-600 {
    color: #e53e3e;
  }

  .xl\:text-red-700 {
    color: #c53030;
  }

  .xl\:text-red-800 {
    color: #9b2c2c;
  }

  .xl\:text-red-900 {
    color: #742a2a;
  }

  .xl\:text-orange-100 {
    color: #fffaf0;
  }

  .xl\:text-orange-200 {
    color: #feebc8;
  }

  .xl\:text-orange-300 {
    color: #fbd38d;
  }

  .xl\:text-orange-400 {
    color: #f6ad55;
  }

  .xl\:text-orange-500 {
    color: #ed8936;
  }

  .xl\:text-orange-600 {
    color: #dd6b20;
  }

  .xl\:text-orange-700 {
    color: #c05621;
  }

  .xl\:text-orange-800 {
    color: #9c4221;
  }

  .xl\:text-orange-900 {
    color: #7b341e;
  }

  .xl\:text-yellow-100 {
    color: #fffff0;
  }

  .xl\:text-yellow-200 {
    color: #fefcbf;
  }

  .xl\:text-yellow-300 {
    color: #faf089;
  }

  .xl\:text-yellow-400 {
    color: #f6e05e;
  }

  .xl\:text-yellow-500 {
    color: #ecc94b;
  }

  .xl\:text-yellow-600 {
    color: #d69e2e;
  }

  .xl\:text-yellow-700 {
    color: #b7791f;
  }

  .xl\:text-yellow-800 {
    color: #975a16;
  }

  .xl\:text-yellow-900 {
    color: #744210;
  }

  .xl\:text-green-100 {
    color: #f0fff4;
  }

  .xl\:text-green-200 {
    color: #c6f6d5;
  }

  .xl\:text-green-300 {
    color: #9ae6b4;
  }

  .xl\:text-green-400 {
    color: #68d391;
  }

  .xl\:text-green-500 {
    color: #48bb78;
  }

  .xl\:text-green-600 {
    color: #38a169;
  }

  .xl\:text-green-700 {
    color: #2f855a;
  }

  .xl\:text-green-800 {
    color: #276749;
  }

  .xl\:text-green-900 {
    color: #22543d;
  }

  .xl\:text-teal-100 {
    color: #e6fffa;
  }

  .xl\:text-teal-200 {
    color: #b2f5ea;
  }

  .xl\:text-teal-300 {
    color: #81e6d9;
  }

  .xl\:text-teal-400 {
    color: #4fd1c5;
  }

  .xl\:text-teal-500 {
    color: #38b2ac;
  }

  .xl\:text-teal-600 {
    color: #319795;
  }

  .xl\:text-teal-700 {
    color: #2c7a7b;
  }

  .xl\:text-teal-800 {
    color: #285e61;
  }

  .xl\:text-teal-900 {
    color: #234e52;
  }

  .xl\:text-blue-100 {
    color: #ebf8ff;
  }

  .xl\:text-blue-200 {
    color: #bee3f8;
  }

  .xl\:text-blue-300 {
    color: #90cdf4;
  }

  .xl\:text-blue-400 {
    color: #63b3ed;
  }

  .xl\:text-blue-500 {
    color: #4299e1;
  }

  .xl\:text-blue-600 {
    color: #3182ce;
  }

  .xl\:text-blue-700 {
    color: #2b6cb0;
  }

  .xl\:text-blue-800 {
    color: #2c5282;
  }

  .xl\:text-blue-900 {
    color: #2a4365;
  }

  .xl\:text-indigo-100 {
    color: #ebf4ff;
  }

  .xl\:text-indigo-200 {
    color: #c3dafe;
  }

  .xl\:text-indigo-300 {
    color: #a3bffa;
  }

  .xl\:text-indigo-400 {
    color: #7f9cf5;
  }

  .xl\:text-indigo-500 {
    color: #667eea;
  }

  .xl\:text-indigo-600 {
    color: #5a67d8;
  }

  .xl\:text-indigo-700 {
    color: #4c51bf;
  }

  .xl\:text-indigo-800 {
    color: #434190;
  }

  .xl\:text-indigo-900 {
    color: #3c366b;
  }

  .xl\:text-purple-100 {
    color: #faf5ff;
  }

  .xl\:text-purple-200 {
    color: #e9d8fd;
  }

  .xl\:text-purple-300 {
    color: #d6bcfa;
  }

  .xl\:text-purple-400 {
    color: #b794f4;
  }

  .xl\:text-purple-500 {
    color: #9f7aea;
  }

  .xl\:text-purple-600 {
    color: #805ad5;
  }

  .xl\:text-purple-700 {
    color: #6b46c1;
  }

  .xl\:text-purple-800 {
    color: #553c9a;
  }

  .xl\:text-purple-900 {
    color: #44337a;
  }

  .xl\:text-pink-100 {
    color: #fff5f7;
  }

  .xl\:text-pink-200 {
    color: #fed7e2;
  }

  .xl\:text-pink-300 {
    color: #fbb6ce;
  }

  .xl\:text-pink-400 {
    color: #f687b3;
  }

  .xl\:text-pink-500 {
    color: #ed64a6;
  }

  .xl\:text-pink-600 {
    color: #d53f8c;
  }

  .xl\:text-pink-700 {
    color: #b83280;
  }

  .xl\:text-pink-800 {
    color: #97266d;
  }

  .xl\:text-pink-900 {
    color: #702459;
  }

  .xl\:hover\:text-transparent:hover {
    color: transparent;
  }

  .xl\:hover\:text-black:hover {
    color: #000;
  }

  .xl\:hover\:text-white:hover {
    color: #fff;
  }

  .xl\:hover\:text-gray-100:hover {
    color: #f7fafc;
  }

  .xl\:hover\:text-gray-200:hover {
    color: #edf2f7;
  }

  .xl\:hover\:text-gray-300:hover {
    color: #e2e8f0;
  }

  .xl\:hover\:text-gray-400:hover {
    color: #cbd5e0;
  }

  .xl\:hover\:text-gray-500:hover {
    color: #a0aec0;
  }

  .xl\:hover\:text-gray-600:hover {
    color: #718096;
  }

  .xl\:hover\:text-gray-700:hover {
    color: #4a5568;
  }

  .xl\:hover\:text-gray-800:hover {
    color: #2d3748;
  }

  .xl\:hover\:text-gray-900:hover {
    color: #1a202c;
  }

  .xl\:hover\:text-red-100:hover {
    color: #fff5f5;
  }

  .xl\:hover\:text-red-200:hover {
    color: #fed7d7;
  }

  .xl\:hover\:text-red-300:hover {
    color: #feb2b2;
  }

  .xl\:hover\:text-red-400:hover {
    color: #fc8181;
  }

  .xl\:hover\:text-red-500:hover {
    color: #f56565;
  }

  .xl\:hover\:text-red-600:hover {
    color: #e53e3e;
  }

  .xl\:hover\:text-red-700:hover {
    color: #c53030;
  }

  .xl\:hover\:text-red-800:hover {
    color: #9b2c2c;
  }

  .xl\:hover\:text-red-900:hover {
    color: #742a2a;
  }

  .xl\:hover\:text-orange-100:hover {
    color: #fffaf0;
  }

  .xl\:hover\:text-orange-200:hover {
    color: #feebc8;
  }

  .xl\:hover\:text-orange-300:hover {
    color: #fbd38d;
  }

  .xl\:hover\:text-orange-400:hover {
    color: #f6ad55;
  }

  .xl\:hover\:text-orange-500:hover {
    color: #ed8936;
  }

  .xl\:hover\:text-orange-600:hover {
    color: #dd6b20;
  }

  .xl\:hover\:text-orange-700:hover {
    color: #c05621;
  }

  .xl\:hover\:text-orange-800:hover {
    color: #9c4221;
  }

  .xl\:hover\:text-orange-900:hover {
    color: #7b341e;
  }

  .xl\:hover\:text-yellow-100:hover {
    color: #fffff0;
  }

  .xl\:hover\:text-yellow-200:hover {
    color: #fefcbf;
  }

  .xl\:hover\:text-yellow-300:hover {
    color: #faf089;
  }

  .xl\:hover\:text-yellow-400:hover {
    color: #f6e05e;
  }

  .xl\:hover\:text-yellow-500:hover {
    color: #ecc94b;
  }

  .xl\:hover\:text-yellow-600:hover {
    color: #d69e2e;
  }

  .xl\:hover\:text-yellow-700:hover {
    color: #b7791f;
  }

  .xl\:hover\:text-yellow-800:hover {
    color: #975a16;
  }

  .xl\:hover\:text-yellow-900:hover {
    color: #744210;
  }

  .xl\:hover\:text-green-100:hover {
    color: #f0fff4;
  }

  .xl\:hover\:text-green-200:hover {
    color: #c6f6d5;
  }

  .xl\:hover\:text-green-300:hover {
    color: #9ae6b4;
  }

  .xl\:hover\:text-green-400:hover {
    color: #68d391;
  }

  .xl\:hover\:text-green-500:hover {
    color: #48bb78;
  }

  .xl\:hover\:text-green-600:hover {
    color: #38a169;
  }

  .xl\:hover\:text-green-700:hover {
    color: #2f855a;
  }

  .xl\:hover\:text-green-800:hover {
    color: #276749;
  }

  .xl\:hover\:text-green-900:hover {
    color: #22543d;
  }

  .xl\:hover\:text-teal-100:hover {
    color: #e6fffa;
  }

  .xl\:hover\:text-teal-200:hover {
    color: #b2f5ea;
  }

  .xl\:hover\:text-teal-300:hover {
    color: #81e6d9;
  }

  .xl\:hover\:text-teal-400:hover {
    color: #4fd1c5;
  }

  .xl\:hover\:text-teal-500:hover {
    color: #38b2ac;
  }

  .xl\:hover\:text-teal-600:hover {
    color: #319795;
  }

  .xl\:hover\:text-teal-700:hover {
    color: #2c7a7b;
  }

  .xl\:hover\:text-teal-800:hover {
    color: #285e61;
  }

  .xl\:hover\:text-teal-900:hover {
    color: #234e52;
  }

  .xl\:hover\:text-blue-100:hover {
    color: #ebf8ff;
  }

  .xl\:hover\:text-blue-200:hover {
    color: #bee3f8;
  }

  .xl\:hover\:text-blue-300:hover {
    color: #90cdf4;
  }

  .xl\:hover\:text-blue-400:hover {
    color: #63b3ed;
  }

  .xl\:hover\:text-blue-500:hover {
    color: #4299e1;
  }

  .xl\:hover\:text-blue-600:hover {
    color: #3182ce;
  }

  .xl\:hover\:text-blue-700:hover {
    color: #2b6cb0;
  }

  .xl\:hover\:text-blue-800:hover {
    color: #2c5282;
  }

  .xl\:hover\:text-blue-900:hover {
    color: #2a4365;
  }

  .xl\:hover\:text-indigo-100:hover {
    color: #ebf4ff;
  }

  .xl\:hover\:text-indigo-200:hover {
    color: #c3dafe;
  }

  .xl\:hover\:text-indigo-300:hover {
    color: #a3bffa;
  }

  .xl\:hover\:text-indigo-400:hover {
    color: #7f9cf5;
  }

  .xl\:hover\:text-indigo-500:hover {
    color: #667eea;
  }

  .xl\:hover\:text-indigo-600:hover {
    color: #5a67d8;
  }

  .xl\:hover\:text-indigo-700:hover {
    color: #4c51bf;
  }

  .xl\:hover\:text-indigo-800:hover {
    color: #434190;
  }

  .xl\:hover\:text-indigo-900:hover {
    color: #3c366b;
  }

  .xl\:hover\:text-purple-100:hover {
    color: #faf5ff;
  }

  .xl\:hover\:text-purple-200:hover {
    color: #e9d8fd;
  }

  .xl\:hover\:text-purple-300:hover {
    color: #d6bcfa;
  }

  .xl\:hover\:text-purple-400:hover {
    color: #b794f4;
  }

  .xl\:hover\:text-purple-500:hover {
    color: #9f7aea;
  }

  .xl\:hover\:text-purple-600:hover {
    color: #805ad5;
  }

  .xl\:hover\:text-purple-700:hover {
    color: #6b46c1;
  }

  .xl\:hover\:text-purple-800:hover {
    color: #553c9a;
  }

  .xl\:hover\:text-purple-900:hover {
    color: #44337a;
  }

  .xl\:hover\:text-pink-100:hover {
    color: #fff5f7;
  }

  .xl\:hover\:text-pink-200:hover {
    color: #fed7e2;
  }

  .xl\:hover\:text-pink-300:hover {
    color: #fbb6ce;
  }

  .xl\:hover\:text-pink-400:hover {
    color: #f687b3;
  }

  .xl\:hover\:text-pink-500:hover {
    color: #ed64a6;
  }

  .xl\:hover\:text-pink-600:hover {
    color: #d53f8c;
  }

  .xl\:hover\:text-pink-700:hover {
    color: #b83280;
  }

  .xl\:hover\:text-pink-800:hover {
    color: #97266d;
  }

  .xl\:hover\:text-pink-900:hover {
    color: #702459;
  }

  .xl\:focus\:text-transparent:focus {
    color: transparent;
  }

  .xl\:focus\:text-black:focus {
    color: #000;
  }

  .xl\:focus\:text-white:focus {
    color: #fff;
  }

  .xl\:focus\:text-gray-100:focus {
    color: #f7fafc;
  }

  .xl\:focus\:text-gray-200:focus {
    color: #edf2f7;
  }

  .xl\:focus\:text-gray-300:focus {
    color: #e2e8f0;
  }

  .xl\:focus\:text-gray-400:focus {
    color: #cbd5e0;
  }

  .xl\:focus\:text-gray-500:focus {
    color: #a0aec0;
  }

  .xl\:focus\:text-gray-600:focus {
    color: #718096;
  }

  .xl\:focus\:text-gray-700:focus {
    color: #4a5568;
  }

  .xl\:focus\:text-gray-800:focus {
    color: #2d3748;
  }

  .xl\:focus\:text-gray-900:focus {
    color: #1a202c;
  }

  .xl\:focus\:text-red-100:focus {
    color: #fff5f5;
  }

  .xl\:focus\:text-red-200:focus {
    color: #fed7d7;
  }

  .xl\:focus\:text-red-300:focus {
    color: #feb2b2;
  }

  .xl\:focus\:text-red-400:focus {
    color: #fc8181;
  }

  .xl\:focus\:text-red-500:focus {
    color: #f56565;
  }

  .xl\:focus\:text-red-600:focus {
    color: #e53e3e;
  }

  .xl\:focus\:text-red-700:focus {
    color: #c53030;
  }

  .xl\:focus\:text-red-800:focus {
    color: #9b2c2c;
  }

  .xl\:focus\:text-red-900:focus {
    color: #742a2a;
  }

  .xl\:focus\:text-orange-100:focus {
    color: #fffaf0;
  }

  .xl\:focus\:text-orange-200:focus {
    color: #feebc8;
  }

  .xl\:focus\:text-orange-300:focus {
    color: #fbd38d;
  }

  .xl\:focus\:text-orange-400:focus {
    color: #f6ad55;
  }

  .xl\:focus\:text-orange-500:focus {
    color: #ed8936;
  }

  .xl\:focus\:text-orange-600:focus {
    color: #dd6b20;
  }

  .xl\:focus\:text-orange-700:focus {
    color: #c05621;
  }

  .xl\:focus\:text-orange-800:focus {
    color: #9c4221;
  }

  .xl\:focus\:text-orange-900:focus {
    color: #7b341e;
  }

  .xl\:focus\:text-yellow-100:focus {
    color: #fffff0;
  }

  .xl\:focus\:text-yellow-200:focus {
    color: #fefcbf;
  }

  .xl\:focus\:text-yellow-300:focus {
    color: #faf089;
  }

  .xl\:focus\:text-yellow-400:focus {
    color: #f6e05e;
  }

  .xl\:focus\:text-yellow-500:focus {
    color: #ecc94b;
  }

  .xl\:focus\:text-yellow-600:focus {
    color: #d69e2e;
  }

  .xl\:focus\:text-yellow-700:focus {
    color: #b7791f;
  }

  .xl\:focus\:text-yellow-800:focus {
    color: #975a16;
  }

  .xl\:focus\:text-yellow-900:focus {
    color: #744210;
  }

  .xl\:focus\:text-green-100:focus {
    color: #f0fff4;
  }

  .xl\:focus\:text-green-200:focus {
    color: #c6f6d5;
  }

  .xl\:focus\:text-green-300:focus {
    color: #9ae6b4;
  }

  .xl\:focus\:text-green-400:focus {
    color: #68d391;
  }

  .xl\:focus\:text-green-500:focus {
    color: #48bb78;
  }

  .xl\:focus\:text-green-600:focus {
    color: #38a169;
  }

  .xl\:focus\:text-green-700:focus {
    color: #2f855a;
  }

  .xl\:focus\:text-green-800:focus {
    color: #276749;
  }

  .xl\:focus\:text-green-900:focus {
    color: #22543d;
  }

  .xl\:focus\:text-teal-100:focus {
    color: #e6fffa;
  }

  .xl\:focus\:text-teal-200:focus {
    color: #b2f5ea;
  }

  .xl\:focus\:text-teal-300:focus {
    color: #81e6d9;
  }

  .xl\:focus\:text-teal-400:focus {
    color: #4fd1c5;
  }

  .xl\:focus\:text-teal-500:focus {
    color: #38b2ac;
  }

  .xl\:focus\:text-teal-600:focus {
    color: #319795;
  }

  .xl\:focus\:text-teal-700:focus {
    color: #2c7a7b;
  }

  .xl\:focus\:text-teal-800:focus {
    color: #285e61;
  }

  .xl\:focus\:text-teal-900:focus {
    color: #234e52;
  }

  .xl\:focus\:text-blue-100:focus {
    color: #ebf8ff;
  }

  .xl\:focus\:text-blue-200:focus {
    color: #bee3f8;
  }

  .xl\:focus\:text-blue-300:focus {
    color: #90cdf4;
  }

  .xl\:focus\:text-blue-400:focus {
    color: #63b3ed;
  }

  .xl\:focus\:text-blue-500:focus {
    color: #4299e1;
  }

  .xl\:focus\:text-blue-600:focus {
    color: #3182ce;
  }

  .xl\:focus\:text-blue-700:focus {
    color: #2b6cb0;
  }

  .xl\:focus\:text-blue-800:focus {
    color: #2c5282;
  }

  .xl\:focus\:text-blue-900:focus {
    color: #2a4365;
  }

  .xl\:focus\:text-indigo-100:focus {
    color: #ebf4ff;
  }

  .xl\:focus\:text-indigo-200:focus {
    color: #c3dafe;
  }

  .xl\:focus\:text-indigo-300:focus {
    color: #a3bffa;
  }

  .xl\:focus\:text-indigo-400:focus {
    color: #7f9cf5;
  }

  .xl\:focus\:text-indigo-500:focus {
    color: #667eea;
  }

  .xl\:focus\:text-indigo-600:focus {
    color: #5a67d8;
  }

  .xl\:focus\:text-indigo-700:focus {
    color: #4c51bf;
  }

  .xl\:focus\:text-indigo-800:focus {
    color: #434190;
  }

  .xl\:focus\:text-indigo-900:focus {
    color: #3c366b;
  }

  .xl\:focus\:text-purple-100:focus {
    color: #faf5ff;
  }

  .xl\:focus\:text-purple-200:focus {
    color: #e9d8fd;
  }

  .xl\:focus\:text-purple-300:focus {
    color: #d6bcfa;
  }

  .xl\:focus\:text-purple-400:focus {
    color: #b794f4;
  }

  .xl\:focus\:text-purple-500:focus {
    color: #9f7aea;
  }

  .xl\:focus\:text-purple-600:focus {
    color: #805ad5;
  }

  .xl\:focus\:text-purple-700:focus {
    color: #6b46c1;
  }

  .xl\:focus\:text-purple-800:focus {
    color: #553c9a;
  }

  .xl\:focus\:text-purple-900:focus {
    color: #44337a;
  }

  .xl\:focus\:text-pink-100:focus {
    color: #fff5f7;
  }

  .xl\:focus\:text-pink-200:focus {
    color: #fed7e2;
  }

  .xl\:focus\:text-pink-300:focus {
    color: #fbb6ce;
  }

  .xl\:focus\:text-pink-400:focus {
    color: #f687b3;
  }

  .xl\:focus\:text-pink-500:focus {
    color: #ed64a6;
  }

  .xl\:focus\:text-pink-600:focus {
    color: #d53f8c;
  }

  .xl\:focus\:text-pink-700:focus {
    color: #b83280;
  }

  .xl\:focus\:text-pink-800:focus {
    color: #97266d;
  }

  .xl\:focus\:text-pink-900:focus {
    color: #702459;
  }

  .xl\:text-xs {
    font-size: 0.75rem;
  }

  .xl\:text-sm {
    font-size: 0.875rem;
  }

  .xl\:text-base {
    font-size: 1rem;
  }

  .xl\:text-lg {
    font-size: 1.125rem;
  }

  .xl\:text-xl {
    font-size: 1.25rem;
  }

  .xl\:text-2xl {
    font-size: 1.5rem;
  }

  .xl\:text-3xl {
    font-size: 1.875rem;
  }

  .xl\:text-4xl {
    font-size: 2.25rem;
  }

  .xl\:text-5xl {
    font-size: 3rem;
  }

  .xl\:text-6xl {
    font-size: 4rem;
  }

  .xl\:italic {
    font-style: italic;
  }

  .xl\:not-italic {
    font-style: normal;
  }

  .xl\:uppercase {
    text-transform: uppercase;
  }

  .xl\:lowercase {
    text-transform: lowercase;
  }

  .xl\:capitalize {
    text-transform: capitalize;
  }

  .xl\:normal-case {
    text-transform: none;
  }

  .xl\:underline {
    text-decoration: underline;
  }

  .xl\:line-through {
    text-decoration: line-through;
  }

  .xl\:no-underline {
    text-decoration: none;
  }

  .xl\:hover\:underline:hover {
    text-decoration: underline;
  }

  .xl\:hover\:line-through:hover {
    text-decoration: line-through;
  }

  .xl\:hover\:no-underline:hover {
    text-decoration: none;
  }

  .xl\:focus\:underline:focus {
    text-decoration: underline;
  }

  .xl\:focus\:line-through:focus {
    text-decoration: line-through;
  }

  .xl\:focus\:no-underline:focus {
    text-decoration: none;
  }

  .xl\:antialiased {
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
  }

  .xl\:subpixel-antialiased {
    -webkit-font-smoothing: auto;
    -moz-osx-font-smoothing: auto;
  }

  .xl\:tracking-tighter {
    letter-spacing: -0.05em;
  }

  .xl\:tracking-tight {
    letter-spacing: -0.025em;
  }

  .xl\:tracking-normal {
    letter-spacing: 0;
  }

  .xl\:tracking-wide {
    letter-spacing: 0.025em;
  }

  .xl\:tracking-wider {
    letter-spacing: 0.05em;
  }

  .xl\:tracking-widest {
    letter-spacing: 0.1em;
  }

  .xl\:select-none {
    -webkit-user-select: none;
       -moz-user-select: none;
        -ms-user-select: none;
            user-select: none;
  }

  .xl\:select-text {
    -webkit-user-select: text;
       -moz-user-select: text;
        -ms-user-select: text;
            user-select: text;
  }

  .xl\:select-all {
    -webkit-user-select: all;
       -moz-user-select: all;
        -ms-user-select: all;
            user-select: all;
  }

  .xl\:select-auto {
    -webkit-user-select: auto;
       -moz-user-select: auto;
        -ms-user-select: auto;
            user-select: auto;
  }

  .xl\:align-baseline {
    vertical-align: baseline;
  }

  .xl\:align-top {
    vertical-align: top;
  }

  .xl\:align-middle {
    vertical-align: middle;
  }

  .xl\:align-bottom {
    vertical-align: bottom;
  }

  .xl\:align-text-top {
    vertical-align: text-top;
  }

  .xl\:align-text-bottom {
    vertical-align: text-bottom;
  }

  .xl\:visible {
    visibility: visible;
  }

  .xl\:invisible {
    visibility: hidden;
  }

  .xl\:whitespace-normal {
    white-space: normal;
  }

  .xl\:whitespace-no-wrap {
    white-space: nowrap;
  }

  .xl\:whitespace-pre {
    white-space: pre;
  }

  .xl\:whitespace-pre-line {
    white-space: pre-line;
  }

  .xl\:whitespace-pre-wrap {
    white-space: pre-wrap;
  }

  .xl\:break-normal {
    overflow-wrap: normal;
    word-break: normal;
  }

  .xl\:break-words {
    overflow-wrap: break-word;
  }

  .xl\:break-all {
    word-break: break-all;
  }

  .xl\:truncate {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .xl\:w-0 {
    width: 0;
  }

  .xl\:w-1 {
    width: 0.25rem;
  }

  .xl\:w-2 {
    width: 0.5rem;
  }

  .xl\:w-3 {
    width: 0.75rem;
  }

  .xl\:w-4 {
    width: 1rem;
  }

  .xl\:w-5 {
    width: 1.25rem;
  }

  .xl\:w-6 {
    width: 1.5rem;
  }

  .xl\:w-8 {
    width: 2rem;
  }

  .xl\:w-10 {
    width: 2.5rem;
  }

  .xl\:w-12 {
    width: 3rem;
  }

  .xl\:w-16 {
    width: 4rem;
  }

  .xl\:w-20 {
    width: 5rem;
  }

  .xl\:w-24 {
    width: 6rem;
  }

  .xl\:w-32 {
    width: 8rem;
  }

  .xl\:w-40 {
    width: 10rem;
  }

  .xl\:w-48 {
    width: 12rem;
  }

  .xl\:w-56 {
    width: 14rem;
  }

  .xl\:w-64 {
    width: 16rem;
  }

  .xl\:w-auto {
    width: auto;
  }

  .xl\:w-px {
    width: 1px;
  }

  .xl\:w-1\/2 {
    width: 50%;
  }

  .xl\:w-1\/3 {
    width: 33.333333%;
  }

  .xl\:w-2\/3 {
    width: 66.666667%;
  }

  .xl\:w-1\/4 {
    width: 25%;
  }

  .xl\:w-2\/4 {
    width: 50%;
  }

  .xl\:w-3\/4 {
    width: 75%;
  }

  .xl\:w-1\/5 {
    width: 20%;
  }

  .xl\:w-2\/5 {
    width: 40%;
  }

  .xl\:w-3\/5 {
    width: 60%;
  }

  .xl\:w-4\/5 {
    width: 80%;
  }

  .xl\:w-1\/6 {
    width: 16.666667%;
  }

  .xl\:w-2\/6 {
    width: 33.333333%;
  }

  .xl\:w-3\/6 {
    width: 50%;
  }

  .xl\:w-4\/6 {
    width: 66.666667%;
  }

  .xl\:w-5\/6 {
    width: 83.333333%;
  }

  .xl\:w-1\/12 {
    width: 8.333333%;
  }

  .xl\:w-2\/12 {
    width: 16.666667%;
  }

  .xl\:w-3\/12 {
    width: 25%;
  }

  .xl\:w-4\/12 {
    width: 33.333333%;
  }

  .xl\:w-5\/12 {
    width: 41.666667%;
  }

  .xl\:w-6\/12 {
    width: 50%;
  }

  .xl\:w-7\/12 {
    width: 58.333333%;
  }

  .xl\:w-8\/12 {
    width: 66.666667%;
  }

  .xl\:w-9\/12 {
    width: 75%;
  }

  .xl\:w-10\/12 {
    width: 83.333333%;
  }

  .xl\:w-11\/12 {
    width: 91.666667%;
  }

  .xl\:w-full {
    width: 100%;
  }

  .xl\:w-screen {
    width: 100vw;
  }

  .xl\:z-0 {
    z-index: 0;
  }

  .xl\:z-10 {
    z-index: 10;
  }

  .xl\:z-20 {
    z-index: 20;
  }

  .xl\:z-30 {
    z-index: 30;
  }

  .xl\:z-40 {
    z-index: 40;
  }

  .xl\:z-50 {
    z-index: 50;
  }

  .xl\:z-auto {
    z-index: auto;
  }
}
"###;
