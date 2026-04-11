/*! coi-serviceworker v0.1.7 - MIT License - https://github.com/gzuidhof/coi-serviceworker */
if (typeof window === "undefined") {
  self.addEventListener("install", () => self.skipWaiting());
  self.addEventListener("activate", (event) => event.waitUntil(self.clients.claim()));

  self.addEventListener("message", (ev) => {
    if (ev.data && ev.data.type === "deregister") {
      self.registration
        .unregister()
        .then(() => self.clients.matchAll())
        .then((clients) => {
          clients.forEach((client) => client.navigate(client.url));
        });
    }
  });

  self.addEventListener("fetch", (event) => {
    if (event.request.cache === "only-if-cached" && event.request.mode !== "same-origin") {
      return;
    }

    event.respondWith(
      fetch(event.request)
        .then((response) => {
          if (response.status === 0) {
            return response;
          }

          const newHeaders = new Headers(response.headers);
          newHeaders.set("Cross-Origin-Embedder-Policy", "require-corp");
          newHeaders.set("Cross-Origin-Opener-Policy", "same-origin");

          return new Response(response.body, {
            status: response.status,
            statusText: response.statusText,
            headers: newHeaders,
          });
        })
        .catch((e) => console.error(e)),
    );
  });
} else {
  (() => {
    const script = document.currentScript;
    const reloadedBySelf = window.sessionStorage.getItem("coiReloadedBySelf");
    window.sessionStorage.removeItem("coiReloadedBySelf");

    const coepSupported = window.crossOriginIsolated !== undefined;
    const isHTTPS = window.location.protocol === "https:";
    const isLocalhost = window.location.hostname === "localhost" || window.location.hostname === "127.0.0.1";

    if (reloadedBySelf) {
      console.log("Reloaded by coi-serviceworker. Multi-threading should be active.");
    }

    if (coepSupported && window.crossOriginIsolated) {
      return;
    }

    if (!isHTTPS && !isLocalhost) {
      console.log("coi-serviceworker: Not on HTTPS or localhost. Multi-threading will be blocked by the browser.");
      return;
    }

    if (window.navigator.serviceWorker) {
      window.navigator.serviceWorker.register(script.src).then((registration) => {
        console.log("coi-serviceworker: Service worker registered with scope: ", registration.scope);

        registration.addEventListener("updatefound", () => {
          registration.installing.addEventListener("statechange", (ev) => {
            if (ev.target.state === "activated") {
              window.sessionStorage.setItem("coiReloadedBySelf", "true");
              window.location.reload();
            }
          });
        });

        if (registration.active && !window.crossOriginIsolated) {
          window.sessionStorage.setItem("coiReloadedBySelf", "true");
          window.location.reload();
        }
      });
    }
  })();
}
