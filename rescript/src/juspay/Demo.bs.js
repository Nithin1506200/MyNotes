// Generated by ReScript, PLEASE EDIT WITH CARE
'use strict';

var Core__Option = require("@rescript/core/src/Core__Option.bs.js");

var myEle = document.getElementById("microapp1-logs");

if (myEle !== undefined) {
  console.log("the element is found");
} else {
  console.log("the element is not found");
}

var logger = window.parent.logger(myEle);

window.version = "1.0.0";

window.onMerchantEvent = (function (param) {
    
  });

window.onEvent = (function (payload) {
    logger("external")("Event from child microapp i.e. microapp2 through onEvent enent listenter! ")(payload);
  });

var Jos = {};

JOS.addEventListner("onMerchantEvent")();

JOS.addEventListner("onEvent")();

var payload = {
  TAG: /* P2 */1,
  _0: {
    action: "DUI_READY"
  }
};

logger("external")("Informing parent i.e " + JOS.parent + "that microapp bundles are loaded by sending DUI_READY action to jos")({
      TAG: /* Obj1 */0,
      eventFunction: "onEvent",
      payload: payload,
      receiver: JOS.parent
    });

JOS.emitEventWithLog(JOS.parent)("onEvent")("todo")(function (param) {
        
      })(function (param) {
      
    });

logger("internal")("Microapp Initialised!!")({
      TAG: /* String */1,
      _0: ""
    });

var payload_1 = {
  service: JOS.self
};

var payload$1 = {
  TAG: /* P1 */0,
  event: "initiate_result",
  payload: payload_1
};

logger("external")("informing parent i.e. " + JOS.parent + "that microapp is initialised and preprocessing background calls is completed by sendeing initiate result event!")({
      TAG: /* Obj1 */0,
      eventFunction: "onEvent",
      payload: payload$1,
      receiver: JOS.parent
    });

JOS.emitEventWithLog(JOS.parent)("onEvent")("todo")(function (param) {
        
      })(function (param) {
      
    });

setTimeout((function (param) {
        logger("internal")("Click on Make Request button to make a request to microapp2")({
              TAG: /* String */1,
              _0: ""
            });
      }), 0);

var requestButton = Core__Option.getExn(document.getElementById("requestButton"));

requestButton.addEventListner("click", (function ($$event) {
        var success = function (param) {
          return function (result) {
            return function () {
              return logger("external")("Response from microapp2!")({
                          TAG: /* String */1,
                          _0: result
                        });
            };
          };
        };
        logger("external")("sending request to microapp2!")({
              TAG: /* Obj1 */0,
              eventFunction: "onMerchantEvent",
              eventName: "Process",
              payload: payload$1,
              receiver: "microapp2"
            });
        window.Jos.emitEventWithLog("microapp2")("onMerchantEvent")(["process"])(success)();
      }));

exports.myEle = myEle;
exports.logger = logger;
exports.Jos = Jos;
exports.payload = payload$1;
exports.requestButton = requestButton;
/* myEle Not a pure module */
