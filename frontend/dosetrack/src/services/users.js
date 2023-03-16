import API_URL from "../settings";

/// Obtiene un usuario de dosetrack de la base de datos de dosetrack segun su email de auth0
export const getUser = async (email) => {

  const myHeaders = new Headers({
    "Content-Type": "application/json",
  });

  const fetchConfig = {
    method: "GET",
    headers: myHeaders,
    cache: "no-cache",
  };

  const user = await fetchUserByEmail(email);

  if (user) {
    console.log(user);
  } else {
    console.log('Failed to fetch user by email.');
  }

  return user
}

/// Crea un nuevo usuario en dosetrack segun los datos de auth0
export const newUser = async (user) => {

  const myHeaders = new Headers({
    "Content-Type": "application/json",
  });

  const fetchConfig = {
    method: "POST",
    headers: myHeaders,
    cache: "no-cache",
  };

  const res = await fetch(
    `${API_URL}/user/`
  );

  const new_user = await res.json();

  if (res.ok) {
    return new_user
  } else {
    return false;
  }
}

async function fetchUserByEmail(email) {
  try {
    const response = await fetch(`${API_URL}/user/email/${email}/`)

    if (response.ok) {
      const user = await response.json();
      return user;
    } else {
      console.log(`Error: ${response.status}`);
      return null;
    }
  } catch (error) {
    console.error('Error fetching user by email:', error);
    return null;
  }
}