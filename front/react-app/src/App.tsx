import React from "react";
import logo from "./logo.svg";
import "./App.css";
import { useQuery, gql } from "@apollo/client";
import { useKeycloak } from "@react-keycloak/web";
import ArticleList from "./components/ArticleList";

const App = () => {
  //const { keycloak, initialized } = useKeycloak();

  //if (!keycloak.authenticated) {
  //return <div>authenticating</div>;
  //}

  return <ArticleList />;
};

export default App;
