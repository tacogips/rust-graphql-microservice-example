import React from "react";
import ReactDOM from "react-dom";
import "./index.css";
import App from "./App";
import "./index.css";
import reportWebVitals from "./reportWebVitals";

import { ApolloClient, InMemoryCache } from "@apollo/client";
import { ApolloProvider } from "@apollo/client/react";

import { ReactKeycloakProvider } from "@react-keycloak/web";
import Keycloak from "keycloak-js";

export const apolloClient = new ApolloClient({
  uri: "http://localhost:5010",
  cache: new InMemoryCache(),
});

export const keycloakCli = Keycloak({
  url: "http://localhost:5014/auth/",
  //url: "http://localhost:8080/auth/",

  //realm: "testing_org",
  //clientId: "test_local_app",

  realm: "sso_org",
  clientId: "sso_client",
});

keycloakCli.init({ onLoad: "check-sso" });
console.log("userinfo", keycloakCli.loadUserInfo());
ReactDOM.render(
  <ReactKeycloakProvider
    authClient={keycloakCli}
    initOptions={{
      onLoad: "login-required",
    }}
  >
    <ApolloProvider client={apolloClient}>
      <React.StrictMode>
        <App />
      </React.StrictMode>
    </ApolloProvider>
  </ReactKeycloakProvider>,
  document.getElementById("root"),
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
