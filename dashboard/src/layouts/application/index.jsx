import React, {Component} from 'react'
import {Route} from "react-router"
import {Switch} from 'react-router-dom'

import Header from './Header'
import Footer from './Footer'
import {createLoading} from '../'

class Widget extends Component {
  render() {
    return (<div>
      <Header/>
      <Route exact={true} path="/users/sign-in" component={createLoading(() => import ('../../routes/users/SignIn'))}/>
      <Route exact={true} path="/users/sign-up" component={createLoading(() => import ('../../routes/users/SignUp'))}/>
      <Footer/>
    </div>);
  }
}

export default Widget;