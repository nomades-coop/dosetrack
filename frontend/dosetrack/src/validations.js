export const isRequiredRule = (value) => {
  return value != null && value !== "";
};

export const isCommaSeparated = (value) => {
  return /.*,{1}.*/.test(value)
}

export const isDateRule = (value) => {
  let d = new Date(value);
  return d instanceof Date && !isNaN(d);
};

export const isLowerThan = (valueA, valueB) => {
  return valueA < valueB
};

export const isGreaterThan = (valueA, valueB) => {
  return valueA > valueB
};

export const isEqualTo = (valueA, valueB) => {
  return valueA === valueB
};


export const isFormValid = (errors) => {
  return !Object.keys(errors).some((inputName) =>
    Object.keys(errors[inputName]).some(
      (errorName) => errors[inputName][errorName]
    )
  );
};

export const setError = (errors, data, fieldName, callBack, msgError) => {

  // si la funcion validadora tiene 1 parametro pasando ese parametro
  if (callBack.length < 2) {
    if (!callBack(data[fieldName])) {
      errors[fieldName] = errors[fieldName] || { error: [] };
      errors[fieldName] = {
        ...errors[fieldName],
      };
      errors[fieldName]["error"].push(msgError);
    }
  } else {
    // si la funcion validadora tiene mas de un parametro le paso un array
    if (!callBack(data[fieldName[0]], data[fieldName[1]])) {
      errors[fieldName[0]] = errors[fieldName[0]] || { error: [] };
      errors[fieldName[0]] = {
        ...errors[fieldName[0]],
      };
      errors[fieldName[0]]["error"].push(msgError);
    }
  }

};