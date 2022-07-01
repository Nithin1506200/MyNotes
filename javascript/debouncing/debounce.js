const input = document.getElementById("input");
const debounced = document.getElementById("debounce");
const normal = document.getElementById("default");
let [normalCount, debounceCount] = [0, 0];
const update = debounce((text) => {
  debounced.innerText = text;
});
function debounce(cb, delay = 1000) {
  let timeout;
  return (...args) => {
    clearTimeout(timeout);
    timeout = setTimeout(() => {
      cb(...args);
      debounceCount += 1;
      document.querySelector("#debounceCount").innerText = debounceCount;
    }, delay);
  };
}

function throtle(cb, wait = 1000) {
  let shouldwait = false;
  let cacheArgs;
  return (...args) => {
    cacheArgs = args;
    if (shouldwait) {
      return;
    }
    cb(cacheArgs);
    shouldwait = true;
    setTimeout(() => {
      shouldwait = false;
    });
  };
}

input.oninput = (e) => {
  normal.innerText = e.target.value;
  normalCount += 1;
  update(e.target.value);
  document.querySelector("#defaultCount").innerText = normalCount;
};
