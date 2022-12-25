const {invoke} = window.__TAURI__.tauri;
const {listen} = window.__TAURI__.event;

let currentTimeMsgEl;
let currentDateMsgEl;

const dateFormat = new Intl.DateTimeFormat("en-US", {dateStyle : "long"});
const timeFormat = new Intl.DateTimeFormat("en-US", {timeStyle : "long"});

function updateTime() {
  const currentDate = new Date();
  const newDateString = dateFormat.format(currentDate);
  const newTimeString = timeFormat.format(currentDate);
  currentDateMsgEl.textContent = newDateString;
  currentTimeMsgEl.textContent = newTimeString;
}

function close() {
  // fade out
  document.querySelector(":root").classList.add("fade-out");

  // wait 200 milliseconds
  setTimeout(() => {
    // close the window
    invoke("close_backend");
  }, 250);
}

window.addEventListener("DOMContentLoaded", () => {
  currentDateMsgEl = document.querySelector("#current-date-msg");
  currentTimeMsgEl = document.querySelector("#current-time-msg");

  updateTime();
  setInterval(updateTime, 1000);

  window.addEventListener("keyup", (event) => {
    // if it's 'q'
    if (event.isComposing || event.keyCode == 81) {
      close();
    }
  })

  // listen to the `close` event and get a function to remove the event listener
  // there's also a `once` function that subscribes to an event and
  // automatically unsubscribes the listener on the first event NOTE: for some
  // reason, only works with -'s, not_'s in event name
  const unlistenClose = listen("close-frontend", (event) => {
    // event.event is the event name (useful if you want to use a single
    // callback fn for multiple event types) event.payload is the payload object
    close();
  });
});
