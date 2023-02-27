// var express = require("express");
// var https = require("https");
// var http = require("http");
// var fs = require("fs");
import express from "express";
import https from "https";

import fs from "fs";
// This line is from the Node.js HTTPS documentation.
var options = {
  key: fs.readFileSync("key.pem"),
  cert: fs.readFileSync("cert.pem"),
};

// Create a service (the app object is just a callback).
let app = express();

// Create an HTTP service.
// http.createServer(app).listen(8080);
app.listen(8080);
// Create an HTTPS service identical to the HTTP service.
https.createServer(options, app).listen(443);
