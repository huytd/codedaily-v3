import 'whatwg-fetch';

class AuthenticationService {
  static doLogin(username, password) {
    return fetch('/api/users/login', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({username: username, password: password})
    }).then(response => response.json()).then(json => {
      console.log(json);
      if (json.result && json.user && json.token) {
        let user = json.user;
        AuthenticationService.storeUserToLocalStorage(user, json.token);
        return {success: true, user: user};
      } else {
        return {success: false}
      }
    });
  }

  static doRegister(username, email, password) {
      return fetch('/api/users/register', {
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
      }).then(response => response.json());
  }

  static storeUserToLocalStorage(user, token) {
    debugger;
    if (window.localStorage) {
      let store = window.localStorage;
      store.setItem('kipalink_user', user.username);
      store.setItem('kipalink_email', user.email);
      store.setItem('kipalink_login_time', new Date());
      store.setItem('kipalink_token', token);
    }
  }

  static checkForLogin() {
    if (window.localStorage) {
      let store = window.localStorage;
      let last_login = store.getItem('kipalink_login_time');
      let user = store.getItem('kipalink_user');
      if (last_login && user) {
        let distance = Math.floor((new Date(last_login) - new Date()) / 86400000);
        if (distance <= 30) {
          return {isLoggedIn: true, loggedInUser: user}
        }
      }
      return {isLoggedIn: false, loggedInUser: null}
    }
  }

  static doLogout() {
    if (window.localStorage) {
      let store = window.localStorage;
      store.removeItem('kipalink_user');
      store.removeItem('kipalink_email');
      store.removeItem('kipalink_login_time');
    };
  }
}

module.exports = AuthenticationService;
