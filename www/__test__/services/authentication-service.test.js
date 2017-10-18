import AuthenticationService from '../../services/authentication-service';
import fetchMock from 'fetch-mock';

describe('Authentication Service', () => {
  afterEach(() => {
    fetchMock.restore();
  });

  it('Should login success with the right password', async () => {
    const dummyUser = { username: 'dummy', password: 'dummy' };
    fetchMock.post('/api/users/login', {
      result: [ dummyUser ],
      user: dummyUser,
      token: '1234'
    });

    expect.assertions(2);
    const result = await AuthenticationService.doLogin(dummyUser.username, dummyUser.password);
    expect(result.success).toEqual(true);
    expect(result.user).toEqual(dummyUser);
  });

  it('Should login unsuccess with the wrong password', async () => {
    const dummyUser = { username: 'dummy', password: 'dummy' };
    fetchMock.post('/api/users/login', {
      result: [],
    });

    expect.assertions(1);
    const result = await AuthenticationService.doLogin(dummyUser.username, dummyUser.password);
    expect(result.success).toEqual(false);
  });

  it('Should register with provided username, email and password', async () => {
    const username = 'hoangcute';
    const email = 'email@mail.com';
    const password = 'password';
    fetchMock.post('/api/users/register', {
      username, email, password
    });

    expect.assertions(3);
    const result = await AuthenticationService.doRegister(username, email, password);
    expect(result.username).toEqual(username);
    expect(result.email).toEqual(email);
    expect(result.password).toEqual(password);
  });


});
