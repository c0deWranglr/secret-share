import React from 'react';
import { BrowserRouter, Route, Switch } from 'react-router-dom';
import Home from './routes/Home';
import Save from './routes/Save';
import View from './routes/View';
import Header from './components/Header';

function App() {

  const name = window["config"].siteName;
  document.title = name;

  return (
    <>
    <BrowserRouter>
      <Header brand={name} />
      <Switch>
        <Route exact path="/" render={(props) => { return <Home {...props} name={name} /> }} />
        <Route path="/save" render={(props) => { return <Save {...props} /> }} />
        <Route path="/view" render={(props) => { return <View {...props} /> }} />
      </Switch>
    </BrowserRouter>
    </>
  );
}

export default App;
