type User = {username: string, password: string};

function addUser(user: User) {
  if (user.password.length > 8) {
    return true;
  } else {
    return false;
  }
}