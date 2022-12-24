const { invoke } = window.__TAURI__.tauri;

let greetMsgEl;
let currentTimeMsgEl;
let currentDateMsgEl;

const dateFormat =
    new Intl.DateTimeFormat("en-US", {dateStyle : "long"});
const timeFormat =
    new Intl.DateTimeFormat("en-US", {timeStyle : "long"});

function updateTime() {
  const currentDate = new Date();
  const newDateString = dateFormat.format(currentDate);
  const newTimeString = timeFormat.format(currentDate);
  currentDateMsgEl.textContent = newDateString;
  currentTimeMsgEl.textContent = newTimeString;
}

window.addEventListener("DOMContentLoaded", () => {
  currentDateMsgEl = document.querySelector("#current-date-msg");
  currentTimeMsgEl = document.querySelector("#current-time-msg");

  updateTime();
  setInterval(updateTime, 1000);
});
