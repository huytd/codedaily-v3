import React from 'react';
import ReactDOM from 'react-dom';
import 'whatwg-fetch';

let email_input = null;
let user_input = null;
let password_input = null;

class Register extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      step: 1,
      message: ''
    };
  }

  doRegister() {
    let that = this;
    let email = email_input.value;
    let username = user_input.value;
    let password = password_input.value;

    fetch(`/api/users/register`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        id: 1,
        username: username,
        email: email,
        password: password,
        enable: 1
      })
    })
      .then(response => response.json())
      .then(json => {
        if (json.result != false) {
          this.setState({
            message: `Register! You're now able to login!`
          });
          setTimeout(() => {
            window.location.reload();
          }, 700);
        } else {
          this.setState({
            message: 'Sorry, there is something wrong! Your username or email address is already being used.'
          });
        }
      });
  }

  componentDidMount() {
    user_input.focus();
  }

  showPasswordStep() {
    return this.state.step >= 3 ? "step-show" : "step-hide";
  }

  showEmailStep() {
    return this.state.step >= 2 ? "step-show" : "step-hide";
  }

  detectEnterKey(id, event) {
    let key = event.keyCode || event.which || event.code;
    if (key === 13) {
      if (id === 1) {
        this.setState({ step: 2 });
        setTimeout(() => {
          email_input.focus();
        }, 100);
      }
      if (id === 2) {
        this.setState({ step: 3 });
        setTimeout(() => {
          password_input.focus();
        }, 100);
      }
      if (id === 3) {
        this.doRegister();
      }
    }
  }
  
  render() {
    return (
      <div key="register" className="login-box">
        <p><b>$ kipalog --version</b><br/> kipalink-0.1.0c build 2017.09.21b</p>
        <p><b>$ kipalog register</b><br/>
          <div>Hey! Welcome to Kipalog System. You're here for a new account, right?</div>
          <div>What is your username? <input ref={(input) => { user_input = input; }} type="text" onKeyPress={this.detectEnterKey.bind(this, 1)} /></div>
          <div className={this.showEmailStep()}>Enter your email address: <input ref={(einput) => { email_input = einput; }} type="email" onKeyPress={this.detectEnterKey.bind(this, 2)}/></div>
          <div className={this.showPasswordStep()}>Enter your password: <input ref={(pinput) => { password_input = pinput; }} type="password" onKeyPress={this.detectEnterKey.bind(this, 3)}/></div>
        </p>
        <p>{this.state.message}</p>
      </div>
    );
  }
}

export default Register;
