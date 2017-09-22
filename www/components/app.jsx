import React from 'react';
import ReactDOM from 'react-dom';
import 'whatwg-fetch';
import Login from './login';
import Register from './register';

const LINKS_PER_PAGE = 30;

class App extends React.Component {
  fetch_links(page) {
    let that = this;

    fetch(`/api/feed/${page || 1}`)
      .then(response => response.json())
      .then(json => {
        that.setState({
          links: Array.from(json.links) || [],
          total: json.total,
          currentPage: page || 1
        });
        console.log(json);
      });
  }

  decodeEntities(encodedString) {
    var textArea = document.createElement('textarea');
    textArea.innerHTML = encodedString;
    return textArea.value;
  }

  constructor(props) {
    super(props);
    this.state = {
      links: [],
      total: 0,
      currentPage: 1,
      isLoggedIn: false,
      loggedInUser: null,
      showLogin: false,
      showRegister: false
    };
  }

  componentDidMount() {
    this.fetch_links();
    this.checkForLogin();
  }

  checkForLogin() {
    if (window.localStorage) {
      let store = window.localStorage;
      let last_login = store.getItem('kipalink_login_time');
      let user = store.getItem('kipalink_user');
      console.log(new Date(last_login));
      if (last_login && user) {
        let distance = Math.floor((new Date(last_login) - new Date()) / 86400000);
          console.log("DISTANCE", distance);
        if (distance <= 30) {
          this.setState({
            isLoggedIn: true,
            loggedInUser: user
          });
        }
      }
    }
  }

  showLoginComponent() {
    if (this.state.isLoggedIn) {
      return "";
    } else {
      if (this.state.showLogin) {
        return (<Login/>);
      }
      return "";
    }
  }

  showRegisterComponent() {
    if (this.state.showRegister) {
      return (<Register/>);
    }
    return "";
  }

  displayRegister() {
    this.setState({
      showRegister: true
    });
  }

  displayLogin() {
    this.setState({
      showLogin: true
    });
  }

  doLogout() {
    if (window.localStorage) {
      let store = window.localStorage;
      store.removeItem('kipalink_user');
      store.removeItem('kipalink_email');
      store.removeItem('kipalink_login_time');
      setTimeout(() => {
        window.location.reload();
      }, 100);
    }
  }

  showLoggedInUser() {
    if (this.state.isLoggedIn) {
      return [
        (<li><a><b>{this.state.loggedInUser}</b></a></li>),
        (<li><a onClick={this.doLogout}>Logout</a></li>)
      ];
    } else {
      return [
        (<li><a onClick={this.displayLogin.bind(this)}>Login</a></li>),
        (<li><a onClick={this.displayRegister.bind(this)}>Register</a></li>)
      ];
    }
  }
  
  render() {
    let list = this.state.links.map(link => {
      let date = new Date(link.time * 1000);
      return <li key={link.time}>
        <a href={link.url} target="_blank" rel="nofollow">
          <div className="post-title">{this.decodeEntities(link.title)}</div>
          <div className="post-meta">Đăng ngày <span>{date.toLocaleDateString()}</span> tại <span>{link.source}</span></div>
        </a>
      </li>
    });

    let totalPage = parseInt(this.state.total / LINKS_PER_PAGE);
    let pages = Array(totalPage).fill(0).map((page, idx) => {
      let id = idx + 1;
      return <li onClick={this.fetch_links.bind(this, id)} className={this.state.currentPage == id ? 'active' : ''} key={`page-${id}`}>
        { id }
      </li>
    });

    return (
      <div className="container">
        { this.showLoginComponent() }
        { this.showRegisterComponent() }
        <div className="header">
          <span>k</span> <a className="kipalog-link">kipalog links</a>
          <div className="user-control">
            <ul className="filter-list">
              <li><a href="#">Top</a></li>
              <li><a className="active" href="#">Latest</a></li>
              { this.showLoggedInUser() }
            </ul>
          </div>
        </div>
        <div className="content">
          <div className="main">
            <ul className="link-list">
              { list }
            </ul>
            <ul className="pagination">
              { pages }
            </ul>
          </div>
        </div>
      </div>
    );
  }
}

export default App;
