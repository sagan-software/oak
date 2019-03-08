import("../pkg/hello")
    .then(pkg => pkg.main())
    .catch(console.error);
