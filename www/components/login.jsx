import React from 'react';
import ReactDOM from 'react-dom';
import 'whatwg-fetch';

let user_input = null;
let password_input = null;

class Login extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      step: 1,
      message: ''
    };
  }

  doLogin() {
    let that = this;
    let username = user_input.value;
    let password = password_input.value;

    fetch(`/api/users/login`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        username: username,
        password: password
      })
    })
      .then(response => response.json())
      .then(json => {
        console.log(json);
        if (json.result && json.user && json.token) {
          let user = json.user;
          this.setState({
            message: `Hey ${user.username}! Welcome back, dude!`
          });
          if (window.localStorage) {
            let store = window.localStorage;
            store.setItem('kipalink_user', user.username);
            store.setItem('kipalink_email', user.email);
            store.setItem('kipalink_login_time', new Date());
            store.setItem('kipalink_token', json.token);
            setTimeout(() => {
              window.location.reload();
            }, 500);
          }
        } else {
          this.setState({
            message: 'Sorry, wrong username and password!'
          });
        }
      });
  }

  componentDidMount() {
    user_input.focus();
  }

  showPasswordStep() {
    return this.state.step > 1 ? "step-show" : "step-hide";
  }

  detectEnterKey(id, event) {
    let key = event.keyCode || event.which || event.code;
    if (key === 13) {
      if (id === 1) {
        this.setState({ step: 2 });
        setTimeout(() => {
          password_input.focus();
        }, 100);
      }
      if (id === 2) {
        this.doLogin();
      }
    }
  }
  
  render() {
    return (
      <div key="login" className="login-box">
        <p><b>$ kipalog --version</b><br/> kipalink-0.1.0c build 2017.09.21b</p>
        <p><b>$ kipalog login</b><br/>
          <div>What is your username? <input ref={(input) => { user_input = input; }} type="text" onKeyPress={this.detectEnterKey.bind(this, 1)} /></div>
          <div className={this.showPasswordStep()}>Enter your password: <input ref={(pinput) => { password_input = pinput; }} type="password" onKeyPress={this.detectEnterKey.bind(this, 2)}/></div>
        </p>
        <p>{this.state.message}</p>
      </div>
    );
  }
}

export default Login;
