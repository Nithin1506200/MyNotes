const logger = window.parent.logger(document.getElementById("microapp1-logs"));
window.version = "1.0.0";
logger("internal")("Initializing microapp..")("");
window["onMerchantEvent"] = function () {};
window["onEvent"] = function (payload) {
  logger("external")(
    'Event from child microapp i.e. "microapp2" through onEvent event listenter!'
  )(payload);
};
JOS.addEventListener("onMerchantEvent")();
JOS.addEventListener("onEvent")();
var payload = {
  action: "DUI_READY",
};
logger("external")(
  'Informing parent i.e. "' +
    JOS.parent +
    '" that microapp bundles are loaded by sending DUI_READY action to JOS!'
)({
  eventFunction: "onEvent",
  payload: payload,
  receiver: JOS.parent,
});
JOS.emitEventWithLog(JOS.parent)("onEvent")(JSON.stringify(payload))()();
logger("internal")("Microapp Initialised!!")("");
payload = {
  event: "initiate_result",
  payload: {
    service: JOS.self,
  },
};
logger("external")(
  'Informing parent i.e. "' +
    JOS.parent +
    '" that microapp is initialised and preprocssing (e.g. api calls / other background calls) is completed by sending initiate_result event!'
)({
  eventFunction: "onEvent",
  payload: payload,
  receiver: JOS.parent,
});
JOS.emitEventWithLog(JOS.parent)("onEvent")(JSON.stringify(payload))()();
var requestButton = document.getElementById("requestButton");
setTimeout(function () {
  logger("internal")(
    'Click on "Make Request" button to make a request to microapp2!!'
  )("");
}, 0);
requestButton.addEventListener("click", function () {
  payload = {
    action: "reqeust",
    message: "Reqeust from microapp1",
  };
  var success = function (code) {
    return function (result) {
      return function () {
        logger("external")("Response from microapp2!")(result);
      };
    };
  };
  logger("external")("Sending request to microapp2!")({
    eventFunction: "onMerchantEvent",
    eventName: "process",
    payload: payload,
    receiver: "microapp2",
  });
  window.JOS.emitEventWithLog("microapp2")("onMerchantEvent")([
    "process",
    JSON.stringify(payload),
  ])(success)();
});
