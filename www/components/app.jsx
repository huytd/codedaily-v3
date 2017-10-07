import React from 'react';
import ReactDOM from 'react-dom';
import 'whatwg-fetch';

import Login from './login';
import Register from './register';

import utils from '../services/util';
import AuthenticationService from '../services/authentication-service';
import LinkService from '../services/link-service';


const LINKS_PER_PAGE = 30;


class App extends React.Component {
  constructor(props) {
    super(props);
    ReactDOM; // remove unused warning

    this.state = {
      links: [],
      total: 0,
      currentPage: 1,
      isLoggedIn: false,
      loggedInUser: null,
      showLogin: false,
      showRegister: false,
      menuActive: false,
    };
  }


  componentDidMount() {
    this.fetch_links();
    this.checkForLogin();
  }


  fetch_links(page) {
    let that = this;

    LinkService
      .getLinks(page)
      .then(result => {
        const { links, total, currentPage } = result;
        that.setState({ links, total, currentPage });
      });
  }


  toggleMenu() {
    this.setState({
      menuActive: ! this.state.menuActive,
    });
  }


  checkForLogin() {
    const { isLoggedIn, loggedInUser } = AuthenticationService.checkForLogin();
    this.setState({ isLoggedIn, loggedInUser });
  }


  showLoginComponent() {
    if (this.state.showLogin) {
      Login; // remove unused warning
      return (<Login />);
    }

    return "";
  }


  showRegisterComponent() {
    if (this.state.showRegister) {
      Register; // remove unused warning
      return (<Register />);
    }

    return "";
  }


  displayRegister() {
    this.setState({
      showRegister: true,
    });
  }


  displayLogin() {
    this.setState({
      showLogin: true,
    });
  }


  doLogout() {
    AuthenticationService.doLogout();
    utils.reloadAfter(100);
  }


  showLoggedInUser() {
    if (this.state.isLoggedIn) {
      return [
        (
          <li key="user-name">
            <a>
              <b>{this.state.loggedInUser}</b>
            </a>
          </li>
        ),

        (
          <li key="logout-button">
            <a onClick={this.doLogout}>
              Logout
            </a>
          </li>
        ),
      ];
    }

    return [
      (
        <li key="login-button">
          <a onClick={this.displayLogin.bind(this)}>
            Login
          </a>
        </li>
      ),

      (
        <li key="register-button">
          <a onClick={this.displayRegister.bind(this)}>
            Register
          </a>
        </li>
      ),
    ];
  }


  render() {
    let list = this.state.links.map((link, idx) => {
      let date = new Date(link.time * 1000);

      return (
        <li key={link.time + "-" + idx}>
          <a href={link.url} target="_blank" rel="nofollow">
            <div className="post-title">
              {utils.decodeEntities(link.title)}
            </div>

            <div className="post-meta">
              Post date <span>{date.toLocaleDateString()}</span> at <span>{link.source}</span>
            </div>
          </a>
        </li>
      );
    });

    let totalPages = Math.floor(this.state.total / LINKS_PER_PAGE);

    let pages = [];
    for (let id=0; id < totalPages; id++) {
      pages.push((
        <li onClick={this.fetch_links.bind(this, id)} className={this.state.currentPage == id ? 'active' : ''} key={`page-${id}`}>
          { id }
        </li>
      ));
    }


    return (
      <div className="container">
        { this.showLoginComponent() }
        { this.showRegisterComponent() }

        <div className="header">
          <span>k</span> kipalog links

          <div className="user-control">
            <button className="toggle-icon" onClick={this.toggleMenu.bind(this)}></button>
            <ul className={"filter-list " + (this.state.menuActive ? "show": "hide")}>
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
