export const AuthStepType = {
    CallIn: 'CallIn',
    CallInWaiting: 'CallInWaiting',

    Loading: 'Loading',

    NickName: 'NickName',

    Password: "Password",

    RegisterStep1: "RegisterStep1",
    RegisterStep1Duplicate: "RegisterStep1Duplicate",
    RegisterStep1Success: 'RegisterStep1Success',
    RegisterStep2: "RegisterStep2",

    SuccessFull: 'SuccessFull',
    SuccessShort: 'SuccessShort',
    
    TokenDevicePairMiss: 'TokenDevicePairMiss',
    TryLater: 'TryLater',
} as const;