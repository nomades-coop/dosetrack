import API_URL from "../settings";

export const operators_by_company = async (company_id) =>
{
  const res = await fetch(`${API_URL}/operators/${company_id}`);
    
  if (res.ok) {
    let operators = await res.json();
    return operators;
  } else {
    throw new Error("No se pudo obtener la lista de operadores");
  }
};


