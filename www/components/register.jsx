import React from 'react';
import ReactDOM from 'react-dom';

import utils from '../services/util';
import AuthenticationService from '../services/authentication-service';


class Register extends React.Component {
  constructor(props) {
    super(props);
    ReactDOM; // remove unused warning

    this.state = {
      step: 1,
      message: '',
    };


    this.doRegister = this.doRegister.bind(this);
    this.setUserInput = this.setUserInput.bind(this);
    this.setEmailInput = this.setEmailInput.bind(this);
    this.setPasswordInput = this.setPasswordInput.bind(this);
    this.detectEnterKey = this.detectEnterKey.bind(this);
  }


  componentDidMount() {
    this.userInput && this.userInput.focus();
  }


  setUserInput(userInput) {
    this.userInput = userInput;
  }


  setEmailInput(emailInput) {
    this.emailInput = emailInput;
  }


  setPasswordInput(passwordInput) {
    this.passwordInput = passwordInput;
  }


  doRegister() {
    if (this.emailInput == null || this.userInput == null || this.passwordInput == null) {
      return;
    }

    let username = this.userInput.value;
    let email = this.emailInput.value;
    let password = this.passwordInput.value;


    AuthenticationService
      .doRegister(username, email, password)
      .then((result) => {
        if (result.result) {
            this.setState({
              message: `Registered succesfully. You are now able to login`
            });

            utils.reloadAfter(700);
          }
          else {
            this.setState({
              message: `Sorry, something went wrong. Your username or email might have already been used.`
            });
          }
      });
  }


  showPasswordStep() {
    return this.state.step >= 3 ? 'step-show' : 'step-hide';
  }


  showEmailStep() {
    return this.state.step >= 2 ? 'step-show' : 'step-hide';
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
        this.emailInput.focus();
      }, 100);
    }

    else if (id == 2) {
      this.setState({
        step: 3,
      });

      setTimeout(() => {
        this.passwordInput.focus();
      }, 100);
    }

    else if (id == 3) {
      this.doRegister();
    }
  }


  render() {
    return (
      <div key="register" className="login-box">
        <p><b>$ kipalog --version</b><br/> kipalink-0.1.0c build 2017.09.21b</p>
        <p><b>$ kipalog register</b><br/></p>

        <p>
          <div>Hey! Welcome to Kipalog System. You're here for a new account, right?</div>

          <div>
            What is your username?&nbsp;
            <input ref={this.setUserInput} type="text" onKeyPress={(event) => this.detectEnterKey(1, event)} />
          </div>

          <div className={this.showEmailStep()}>
            Enter your email address:&nbsp;
            <input ref={this.setEmailInput} type="email" onKeyPress={(event) => this.detectEnterKey(2, event)}/>
          </div>

          <div className={this.showPasswordStep()}>
            Enter your password:&nbsp;
            <input ref={this.setPasswordInput} type="password" onKeyPress={(event) => this.detectEnterKey(3, event)}/>
          </div>
        </p>

        <p>{this.state.message}</p>
      </div>
    );
  }
}


export default Register;
