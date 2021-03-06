import * as React from "react";
import { render } from "react-dom";
import { Provider } from "react-redux";
import "./bulma.js";
import "./index.html";
import App from "./pages/app";
import store from "./store";
import "./tile.png";

render(
    <Provider store={store} >
        <App />
    </Provider>,
    document.getElementById("root"));
