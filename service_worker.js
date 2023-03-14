const transformUrl = (str) => "/".concat("", str.split("/").slice(3).join("/"));

const cacheName = "portfolio_cv_pwa";

const filesToCache = [
  "/portfolio_cv_pwa/",
  "/portfolio_cv_pwa/index.html",
  "/portfolio_cv_pwa/portfolio_cv_pwa.js",
  "/portfolio_cv_pwa/portfolio_cv_pwa_bg.wasm",
  "/portfolio_cv_pwa/tailwind_output.css",
  "/portfolio_cv_pwa/manifest.json",
  "/portfolio_cv_pwa/fonts/Kanit-Regular.ttf",
  "/portfolio_cv_pwa/img/book-half.svg",
  "/portfolio_cv_pwa/img/left-swipe.svg",
  "/portfolio_cv_pwa/img/github.svg",
  "/portfolio_cv_pwa/img/list.svg",
  "/portfolio_cv_pwa/img/logo.png",
  "/portfolio_cv_pwa/img/my-gps-app.jpg",
  "/portfolio_cv_pwa/img/my-pwa.jpg",
  "/portfolio_cv_pwa/img/profile.jpeg",
  "/portfolio_cv_pwa/img/todoapp-tauri-yew.jpg",
  "/portfolio_cv_pwa/img/white-eye.svg",
  "/portfolio_cv_pwa/img/white-github.svg",
  "/portfolio_cv_pwa/img/x-lg.svg",
  "/portfolio_cv_pwa/img/qr-code.svg",
  "/portfolio_cv_pwa/favicon-256.png",
];

/* Start the service worker and cache all of the app's content */
self.addEventListener("install", function (e) {
  self.skipWaiting();

  e.waitUntil(
    caches.open(cacheName).then(function (cache) {
      return cache.addAll(filesToCache);
    })
  );
});

self.addEventListener("activate", (event) => {
  event.waitUntil(clients.claim());
});

// Network first, falling back to cache. VER:
// https://developer.chrome.com/docs/workbox/caching-strategies-overview/#network-first-falling-back-to-cache
self.addEventListener("fetch", function (event) {
  if (event.request.mode === "navigate") {
    // console.log(transformUrl(event.request.url));
    event.respondWith(caches.match("/portfolio_cv_pwa/"));
    return;
  }

  event.respondWith(
    caches.open(cacheName).then((cache) => {
      // Go to the network first
      return fetch(event.request.url)
        .then((fetchedResponse) => {
          cache.put(transformUrl(event.request.url), fetchedResponse.clone());

          return fetchedResponse;
        })
        .catch(() => {
          // If the network is unavailable, get
          return cache.match(transformUrl(event.request.url));
        });
    })
  );
});

// https://christianheilmann.com/2022/01/13/turning-a-github-page-into-a-progressive-web-app/
// https://stackoverflow.com/questions/45309959/service-worker-offline-support-with-pushstate-and-client-side-routing
