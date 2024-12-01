# tucant

## How does it work

This software consists of the tucan-connector component that extracts information from the html of [TUCaN](https://www.tucan.tu-darmstadt.de) and provides it as a nicer to use programming API. The tucan-injector component can then be used to show that data with a nicer UI that is written using the Rust frontend library [Yew](https://yew.rs/) and that is compiled to [WebAssembly](https://webassembly.org/). This WebAssembly can be injected into the actual TUCaN website using [Tampermonkey](https://www.tampermonkey.net/). Then, some pages provide an overlay with the information in a nicer format and caching.

## Features

Currently, the following TUCaN pages have a nicer UI and caching:
- Veranstaltungen -> Anmeldung
  ![Veranstaltungen -> Anmeldung with nicer UI](./.github/veranstaltungen_anmeldung.png)

## Usage

Install Tampermonkey.
Add a Tampermonkey script with the following content:
```javascript
// ==UserScript==
// @name         tucant
// @namespace    https://www.tucan.tu-darmstadt.de
// @version      2024-10-24
// @description  A nicer, faster and more featureful frontend to TUCaN
// @author       Moritz Hedtke <Moritz.Hedtke@t-online.de>
// @match        https://www.tucan.tu-darmstadt.de/*
// @run-at       document-start
// @grant        GM_addElement
// ==/UserScript==

GM_addElement('script', {
    src: 'https://tucant.github.io/tucant/tucan-injector.js',
    type: 'text/javascript'
});
```

## Development

```
nix develop
cd tucan-injector
bacon

cd tucan-injector/dist
python -m http.server
```

Install Tampermonkey.
Add a Tampermonkey script with the following content:

```javascript
// ==UserScript==
// @name         New Userscript
// @namespace    https://www.tucan.tu-darmstadt.de
// @version      2024-10-18
// @description  try to take over the world!
// @author       You
// @match        https://www.tucan.tu-darmstadt.de/*
// @run-at       document-start
// @grant        GM_addElement
// ==/UserScript==

GM_addElement('script', {
    src: 'http://localhost:8000/tucan-injector.js',
    type: 'text/javascript'
});
```
