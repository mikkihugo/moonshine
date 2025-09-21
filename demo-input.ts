// Demo file with multiple code quality issues for moon-shine to fix

interface userdata { // C043: Interface should start with 'I'
  name: string;
  active: boolean; // C042: Boolean should have descriptive prefix
  age: number;
}

function processUser(data: userdata,) { // C017: Should avoid complex constructor logic
  let valid = true; // C042: Boolean naming issue
  let count = 0;

  // C029: Missing error logging in catch block
  try {
    if (data.name.length > 0) { // Style: Missing space after if
      count++;
      valid = data.age > 0;
    }
    return { success: valid, total: count, };
  } catch (e) {
    // Missing error logging
    return { success: false, total: 0, };
  }
}

class UserProcessor {
  constructor(config: any,) { // C017: Constructor doing too much work
    this.validateConfig(config,);
    this.setupDatabase();
    this.initializeCache();
    this.startBackgroundJobs(); // Too much in constructor
  }

  private validateConfig(cfg: any,) {}
  private setupDatabase() {}
  private initializeCache() {}
  private startBackgroundJobs() {}
}
