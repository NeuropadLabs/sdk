import { AgentConfig } from "./types"

export class NeuroAgent {
  constructor(public config: AgentConfig) {}

  describe() {
    return `Agent ${this.config.name} running on ${this.config.platform}`
  }
}
