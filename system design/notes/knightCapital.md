
https://en.wikipedia.org/wiki/Knight_Capital_Group


Knight Capital Group was an American global financial services firm engaging in market making, electronic execution, and institutional sales and trading. In 2012 Knight was the largest trader in U.S. equities with a market share of around 17 percent on the New York Stock Exchange (NYSE) as well as on the Nasdaq Stock Market. Knight’s Electronic Trading Group (ETG) managed an average daily trading volume of more than 3.3 billion trades, trading over $21 billion … daily.

It took 17 years of dedicated work to build Knight Capital Group into one of the leading trading houses on Wall Street. And it all nearly ended in less than one hour.

What happened to Knight on the morning of August 1, 2012, is every CEO’s nightmare: A simple human error, easily spotted with hindsight but nearly impossible to predict in advance, threatened to end the firm.

At Knight, some new trading software contained a flaw that became apparent only after the software was activated when the New York Stock Exchange (NYSE) opened that day. The errant software sent Knight on a buying spree, snapping up 150 different stocks at a total cost of around $7 billion, all in the first hour of trading.

Under stock exchange rules, Knight would have been required to pay for those shares three days later. However, there was no way it could pay, since the trades were unintentional and had no source of funds behind them. The only alternatives were to try to have the trades canceled, or to sell the newly acquired shares the same day.

Knight tried to get the trades canceled. Securities and Exchange Commission (SEC) chairman Mary Schapiro refused to allow this for most of the stocks in question, and this seems to have been the right decision. Rules were established after the “flash crash” of May 2010 to govern when trades should be canceled. Knight’s buying binge did not drive up the price of the purchased stocks by more than 30 percent, the cancellation threshold, except for six stocks. Those transactions were reversed. In the other cases, the trades stood.

This was very bad news for Knight but was only fair to its trading partners, who sold their shares to Knight’s computers in good faith. Knight’s trades were not like those of the flash crash, when stocks of some of the world’s largest companies suddenly began trading for as little as a penny and no buyer could credibly claim the transaction price reflected the correct market value.

Once it was clear that the trades would stand, Knight had no choice but to sell off the stocks it had bought. Just as the morning’s buying rampage had driven up the price of those shares, a massive sale into the market would likely have forced down the price, possibly to a point so low that Knight would not have been able to cover the losses.

`Goldman Sachs` stepped in to buy Knight’s entire unwanted position at a price that cost Knight $440 million – a staggering blow, but one the firm might be able to absorb. And if Knight failed, the only injured party, apart from Knight’s shareholders (including Goldman), would have been Goldman itself.

Disposing of the accidentally purchased shares was only the first step in Knight CEO Thomas Joyce’s battle to save his company. The trades had sapped the firm’s capital, which would have forced it to greatly cut back its business, or maybe to stop operating altogether, without a cash infusion. And as word spread about the software debacle, customers were liable to abandon the company if they did not trust its financial and operational capacities.

A week later, Knight received a $400 million cash infusion from a group of investors, and by the next summer, it was acquired by a rival, Getco LLC. This case study will discuss the events leading up to this catastrophe, what went wrong, and how this could be prevented.

If you want to make sure your business critical project is off to a great start instead of on its way on my list with project failures? Then a New Project Audit is what you are looking for.

If you want to know where you are standing with that large, multi-year, strategic project? Or you think one of your key projects is in trouble? Then a Project Review is what you are looking for.

If you just want to read more project failure case studies? Then have a look at the overview of all case studies I have written here.
Timeline of Events
Some of Knight’s biggest customers were the discount brokers and online brokerages such as TD Ameritrade, E*Trade, Scottrade, and Vanguard. Knight also competed for business with financial services giants like Citigroup, UBS, and Citadel. However, these larger competitors could internalize increasingly larger amounts of trading away from the public eye in their own exclusive markets or shared private markets, so-called dark pools. Since 2008, the portion of all stock trades in the U.S. taking place away from public markets has risen from 15 percent to more than 40 percent.

In October 2011, the NYSE proposed a dark pool of its own, called the `Retail Liquidity Program (RLP)`. The RLP would create a private market of traders within the NYSE that could anonymously transact shares for fractions of pennies more or less than the displayed bid and offer prices, respectively. The RLP was controversial even within NYSE Euronext, the parent company of the NYSE; its CEO, Duncan Niederauer, had written a public letter in the Financial Times criticizing dark pools for shifting “more and more information… outside the public view and excluded from the price discovery process.”

The SEC decision benefited large institutional investors who could now buy or sell large blocks of shares with relative anonymity and without moving the public markets; however, it came again at the expense of market makers. During the months of debate, Joyce had not given the RLP much chance for approval, saying in one interview, “Frankly, I don’t see how the SEC can be possibly OK it.” In early June 2012, the NYSE received SEC approval of its RLP, and it quickly announced the RLP would go live on August 1, 2012, giving market makers just over 30 days to prepare. Joyce insisted on participating in the RLP because giving up the order flow without a fight would have further dented profits in its best line of business.
What Went Wrong
With only a month between the RLP’s approval and its go-live, Knight’s software development team worked feverishly to make the necessary changes to its trade execution systems – including SMARS, its algorithmic, high-speed order router. SMARS stands for `Smart Market Access Routing System.`

SMARS was able to execute thousands of orders per second and could compare prices between dozens of different trading venues within fractions of a second.

A core feature of SMARS is to receive orders from other upstream components in Knight’s trading platform (“parent” orders) and then, as needed based on the available liquidity and price, sends one or more representative (“child”) orders to downstream, external venues for execution.

## Power Peg

The new RLP code in SMARS replaced some unused code in the relevant portion of the order router; the old code previously had been used for an order algorithm called “Power Peg,” which Knight had stopped using since 2003. Power Peg was a test program that bought high and sold low; it was specifically designed to move stock prices higher and lower in order to verify the behavior of its other proprietary trading algorithms in a controlled environment. It was not to be used in the live, production environment.

There were grave problems with Power Peg in the current context. First, the Power Peg code remained present and executable at the time of the RLP deployment despite its lack of use. Keeping such “dead code” is bad practice, but common in large software systems maintained for years. Second, the new RLP code had repurposed a flag that was formerly used to activate the Power Peg code; the intent was that when the flag was set to “yes,” the new RLP component – not Power Peg –  would be activated. Such repurposing often creates confusion, had no substantial benefit, and was a major mistake, as we shall see shortly.

Code refactoring

There had been substantial code refactorings in SMARS over the years without thorough regression testing; in 2005, Knight changed the cumulative quantity function that counted the number of shares of the parent order that had been executed and filled to decide whether to route another child order. The cumulative quantity function was now invoked earlier in the SMARS workflow, which in theory was a good idea to prevent excess system activity; in practice, it was now disconnected from Power Peg (which used to call it directly), could no longer throttle the algorithm when orders were filled, and Knight never retested Power Peg after this change.

Manual deployment

In the week before go-live, a Knight engineer manually deployed the new `RLP code in SMARS to its eight servers`. However, the engineer made a mistake and did not copy the new code to `one of the servers`. Knight did not have a second engineer review the deployment, and neither was there an automated system to alert anyone to the discrepancy. Knight also had no written procedures requiring a supervisory review, all facts we shall return to later.

The crash

On August 1, 8:01 a.m. EST, an internal system called BNET generated 97 email messages that referenced SMARS and identified an error described as “Power Peg disabled.” These obscure, internal messages were sent to Knight personnel, but their channel was not designated for high-priority alerts and the staff generally did not review them in real-time; however, they were the proverbial smoke of the smoldering code and deployment bits about to burn, and it was a missed opportunity to identify and fix the DevOps issue prior to market open.

At 9:30 a.m. EST, Knight began receiving RLP orders from broker-dealers, and SMARS distributed the incoming work to its servers. The seven servers that had the new RLP code processed the orders correctly. However, orders sent to the eighth server with the defective Power Peg code activated by the repurposed flag soon triggered the fault line of a financial tectonic plate. This server began to continuously send child orders for each incoming parent order without regard to the number of confirmed executions Knight had already received from other trading venues.

The results were immediately catastrophic. For the 212 incoming parent orders processed by the defective Power Peg code, SMARS sent thousands of child orders per second that would buy high and sell low, resulting in 4 million executions in 154 stocks for more than 397 million shares in approximately 45 minutes. For 75 of these stocks, Knight’s executions jostled prices more than 5% and comprised more than 20% of trading volume; for 37 stocks, prices lurched more than 10% and Knight’s executions constituted more than 50% of trading volume.

Following the flash crash of May 6, 2010, in which the Dow Jones Industrial Average (DJIA) lost over 1000 points in minutes, the SEC announced several new rules to regulate securities trading.

1) Circuit breakers were required to stop trading if the market experienced what was labeled as “significant price fluctuations” of more than 10 percent during a five-minute period.

2) The SEC required more specific conditions governing the cancellation of trades. For events involving between five and 20 stocks, trades could be cancelled if they were at least 10 percent away from the “reference price,” the last sale before pricing was disrupted; for events involving more than 20 stocks, trades could be cancelled if they deviated more than 30 percent from the reference price.

3) Securities Exchange Act Rule C.F.R 240.15c3–5 (“Rule”) went into effect, requiring the exchanges and broker-dealers to implement risk management controls to ensure the integrity of their systems as well as executive review and certification of the controls.

Since the flash crash rules were designed for price swings, not trading volume, they did not kick in as intended and stop trading because few of the stocks traded by Knight on that fateful day exceeded the 10 percent price change threshold.

By 9:34 a.m., NYSE computer analysts noticed that market volumes were double the normal level and traced the volume spike back to Knight. Niederauer tried calling Joyce, but Joyce was still at home recovering from knee surgery.

The NYSE then alerted Knight’s chief information officer, who gathered the firm’s top IT people; most trading shops would have flipped a kill switch in their algorithms or would have simply shut down systems. However, Knight had no documented procedures for incident response, again, another fact we shall return to later. So, it continued to fumble in the dark for another 20 minutes, deciding next that the problem was the new code.

Because the “old” version allegedly worked, Knight reverted back to the old code still running on the eighth server and reinstalled it on the others. As it turned out, this was the worst possible decision because all eight servers now had the defective Power Peg code activated by the misappropriated RLP flag and executing without a throttle.

It was not until 9:58 a.m. that Knight engineers identified the root cause and shut down SMARS on all the servers; however, the damage had been done. Knight had executed over 4 million trades in 154 stocks totaling more than 397 million shares; it assumed a net long position in 80 stocks of approximately $3.5 billion as well as a net short position in 74 stocks of approximately $3.15 billion.
How Knight Capital Could Have Done Things Differently
This case study contains several lessons useful for project managers, IT professionals, and business leaders. Knight could have prevented the failure and minimized the damage with a variety of modern software development and operating practices (DevOps). Below, I describe eight of these measures and how they could have made a difference for Knight Capital.

Use of Version Control

Do not run dead code. Instead, always prune dead code and use version control systems to track the changes. You should not re-purpose configuration flags; rather, activate new features with new flags.

Version control is any kind of practice that tracks and provides control over changes to source code. Teams can use version control software to maintain documentation and configuration files as well as source code.

As teams design, develop, and deploy software, it is common for multiple versions of the same software to be deployed in different sites and for the software's developers to be working simultaneously on updates. Bugs or features of the software are often only present in certain versions (because of the fixing of some problems and the introduction of others as the program develops).

Therefore, for the purposes of locating and fixing bugs, it is vitally important to be able to retrieve and run different versions of the software to determine in which version(s) the problem occurs. It may also be necessary to develop two versions of the software concurrently: for instance, where one version has bugs fixed, but no new features (branch), while the other version is where new features are worked on (trunk).

Writing Unit Tests

The purpose of unit testing is not for finding bugs. It is a specification for the expected behavior of the code under test. The code under test is the implementation for those expected behaviors. So unit tests and the code under test are used to check the correctness of each other and protect each other. Later, when someone changes the code under test, and it changes the behavior that is expected by the original author, the test will fail. If your code is covered by a reasonable amount of unit tests, you can maintain the code without breaking the existing feature. That’s why Michael Feathers defines legacy code in his seminal book "Working Effectively with Legacy Code" as code without unit tests. Without unit tests your development efforts will be a major risk every time you change your legacy code.

Code Reviews

Code review is a systematic examination (sometimes referred to as peer review) of source code. It is intended to find mistakes overlooked in software development, improving the overall quality of software. Reviews are done in various forms such as pair programming, informal walkthroughs, and formal inspections.

Automated Tests and Test Automation

In the world of testing in general, and continuous integration and delivery in particular, there are two types of automation:

1) Automated Tests
2) Test Automation

While it might just seem like two different ways to say the same thing, these terms actually have very different meanings.

Automated tests are tests that can be run automatically, often developed in a programming language. In this case, we talk about the individual test cases, either unit-tests, integration/service, performance tests, end-2-end tests, or acceptance tests. The latter is also known as specification by example.

Test automation is a broader concept and includes automated tests. From my perspective, it should be about the full automation of test cycles, from check-in up to deployment – also called continuous testing. Both automated testing and test automation are important to continuous delivery, but it's really the latter that makes continuous delivery of a high quality even possible.

Had Knight implemented automated tests and test automation for the new and existing SMARS functionalities they would have caught the error before deploying it in production.

Automated Deployment Process

It is not enough to build great software and test it; you also have to ensure it is delivered to market correctly so that your customers get the value you are delivering (and so you don’t bankrupt your company). The engineer(s) who deployed SMARS are not solely to blame here – the process Knight had set up was not appropriate for the risk they were exposed to. Additionally, their process (or lack thereof) was inherently prone to error. Any time your deployment process relies on humans reading and following instructions you are exposing yourself to risk. Humans make mistakes. The mistakes could be in the instructions, in the interpretation of the instructions, or in the execution of the instructions.

Deployments need to be automated and repeatable and as free from potential human error as possible. Had Knight implemented an automated deployment system – complete with configuration, deployment, and test automation – the error that caused the nightmare would have been avoided.

Step-by-Step Deployment Process Guide 

Anybody (even somebody who is usually not doing this) should be able to deploy on production with this guide on the table. Of course, the more you go into the direction of automated deployment, the smaller this guide becomes, because all documentation of this process is coded in your automated processes. The probability of doing something wrong with a step-by-step guide (or a checklist) is a multitude smaller as without. This has been proven many times in the medical space.

Timeline

The timeline was another reason Knight failed to deliver the RLP solution. Knight’s IT project managers and CIO should have pushed back on the hyper-aggressive delivery schedule and countered its business leaders with an alternative phased schedule instead of the Big Bang – pun intended. Thirty days to implement, test, and deploy major changes to an algorithmic trading system that is used to make markets daily worth billions of dollars is impulsive, naive, and reckless.

Risk Management

Risk management is a vital capability for a modern organization, especially for financial services companies. The SEC’s report (see References) concluded: “Although automated technology brings benefits to investors, including increased execution speed and some decreased costs, automated trading also amplifies certain risks. As market participants increasingly rely on computers to make order routing and execution decisions, it is essential that compliance and risk management functions at brokers or dealers keep pace… Good technology risk management practices include quality assurance, continuous improvement, controlled user acceptance testing, process measuring, management and control, regular and rigorous review for compliance with applicable rules and regulations, an independent audit process, technology governance that prevents software malfunctions, system errors and failures, service outages, and when such issues arise, a prompt, effective, and risk-mitigating response.”

While Knight had order controls in other systems, it did not compare orders exiting SMARS with those that entered it. Knight’s primary risk monitoring tool, known as “PMON,” is a post-execution position monitoring system. At the opening of the market, senior Knight personnel observed a large volume of positions in a special account called 33 that temporarily held multiple types of positions, including positions resulting from executions that Knight received back from markets that its systems could not match to the unfilled quantity of a parent order. There was a $2 million gross limit to the 33 account, but it was not linked to any automated controls concerning Knight’s overall financial exposure.

Furthermore, PMON relied entirely on human monitoring, did not generate automated alerts, and did not highlight the display of account exposures based on whether a limit had been exceeded. Moreover, Knight also had no circuit breakers, which is a standard pattern and practice for financial services companies.
Closing Thoughts
Although Knight was one of the most experienced companies in automated trading at the time (and the software that goes with it), it failed to implement many of the standard DevOps best practices that could have prevented this disaster at any number of intervals.

Knight Capital Group Holdings was eventually acquired by another market making rival, Virtu LLC, in July 2017 for $1.4 billion. The silver lining to the story was that Knight was not too big to fail, and the market handled the failure with a relatively organized rescue without the help of taxpayers. However, a dark cloud remains: market data suggests that 70 percent of U.S. equity trading is now executed by high-frequency trading firms, and one can only wonder when, not if, the next flash crash will occur.

If you want to make sure your business critical project is off to a great start instead of on its way on my list with project failures? Then a New Project Audit is what you are looking for.

If you want to know where you are standing with that large, multi-year, strategic project? Or you think one of your key projects is in trouble? Then a Project Review is what you are looking for.

If you just want to read more project failure case studies? Then have a look at the overview of all case studies I have written here.