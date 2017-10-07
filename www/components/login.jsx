import React from 'react';
import ReactDOM from 'react-dom';

import utils from '../services/util';
import AuthenticationService from '../services/authentication-service';


class Login extends React.Component {
  constructor(props) {
    super(props);
    ReactDOM; // remove unused warning

    this.state = {
      step: 1,
      message: '',
    };


    this.doLogin = this.doLogin.bind(this);
    this.setUserInput = this.setUserInput.bind(this);
    this.setPasswordInput = this.setPasswordInput.bind(this);
    this.detectEnterKey = this.detectEnterKey.bind(this);
  }


  componentDidMount() {
    this.userInput && this.userInput.focus();
  }


  setPasswordInput(passwordInput) {
    this.passwordInput = passwordInput;
  }


  setUserInput(userInput) {
    this.userInput = userInput;
  }


  render() {
    return (
      <div key="login" className="login-box">
        <p><b>$ kipalog --version</b><br/> kipalink-0.1.0c build 2017.09.21b</p>

        <div><b>$ kipalog login</b><br/>
          <div>
            What is your username?&nbsp;
            <input ref={this.setUserInput} type="text" onKeyPress={(event) => this.detectEnterKey(1, event)} />
          </div>

          <div className={this.showPasswordStep()}>
            Enter your password:&nbsp;
            <input ref={this.setPasswordInput} type="password" onKeyPress={(event) => this.detectEnterKey(2, event)}/>
          </div>
        </div>

        <p>{this.state.message}</p>
      </div>
    );
  }


  doLogin() {
    if (this.userInput == null || this.passwordInput == null) {
      return;
    }

    let username = this.userInput.value;
    let password = this.passwordInput.value;


    AuthenticationService
      .doLogin(username, password)
      .then((result) => {
        if (result.success) {
          this.setState({
            message: `Hey ${result.user.username}! Welcome back, dude!`,
          }, () => { utils.reloadAfter(500); });
        } else {
          this.setState({
            message: `Sorry, wrong username and password!`,
          });
        }
      });
  }


  showPasswordStep() {
    return this.state.step > 1 ? 'step-show' : 'step-hide';
  }


  detectEnterKey(id, event) {
    let enterPressed = utils.enterKeyPressed(event);

    if (! enterPressed) {
      return;
    }

    if (id == 1) {
      this.setState({
        step: 2,
      });

      setTimeout(() => {
        this.passwordInput.focus();
      }, 100);
    }
    else if (id == 2) {
      this.doLogin();
    }
  }
}


export default Login;
