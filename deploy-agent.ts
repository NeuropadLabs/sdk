import { NeuroAgent, deployAgent } from "../sdk"

const agent = new NeuroAgent({
  name: "AlphaAgent",
  platform: "Local AI"
})

deployAgent(agent)
