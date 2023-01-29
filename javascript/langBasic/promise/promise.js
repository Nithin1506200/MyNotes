let MyPromise = new Promise((resolve, reject) => {});

function loadScript(data) {
  return new Promise((resolve, reject) => {
    if (data === "Loaded") {
      resolve("Work is done");
    } else {
      reject("work is not done");
    }
  });
}

loadScript("fdj")
  .then((data) => {
    console.log(data);
  })
  .catch((err) => {
    console.log(err, "failed to pass");
  })
  .finally(() => {
    console.log("job ended");
  });
