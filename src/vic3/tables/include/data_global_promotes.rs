&[
    ("AIATTITUDE", Args::Args(&[]), Vic3(AIAttitude)),
    ("AISTRATEGY", Args::Args(&[]), Vic3(AIStrategy)),
    ("ALERT", Args::Args(&[]), Vic3(Alert)),
    ("ALERT_GROUP", Args::Args(&[]), Vic3(AlertGroup)),
    ("ALLY_COUNTRY", Args::Args(&[]), Vic3(Country)),
    ("AccessGameRules", Args::Args(&[]), Vic3(JominiGameRules)),
    ("AccessHighlightManager", Args::Args(&[]), Vic3(HighlightManager)),
    ("AccessLogViewer", Args::Args(&[]), Vic3(LogViewer)),
    ("AccessMetaPlayer", Args::Args(&[]), Vic3(Player)),
    ("AccessPlayer", Args::Args(&[]), Vic3(Country)),
    ("AccessPlayerJournalEntry", Args::Args(&[DType(Unknown)]), Vic3(JournalEntry)),
    ("Application", Args::Args(&[]), Vic3(Application)),
    ("BATTLE", Args::Args(&[]), Vic3(Battle)),
    ("BATTLE_CONDITION", Args::Args(&[]), Vic3(BattleCondition)),
    ("BUILDING", Args::Args(&[]), Vic3(Building)),
    ("BUILDING_TYPE", Args::Args(&[]), Vic3(BuildingType)),
    ("CANAL_TYPE", Args::Args(&[]), Vic3(CanalType)),
    ("CHARACTER", Args::Args(&[]), Vic3(Character)),
    ("CHARACTER_INTERACTION", Args::Args(&[]), Vic3(CharacterInteraction)),
    ("CHARACTER_TRAIT", Args::Args(&[]), Vic3(CharacterTrait)),
    ("CIVIL_WAR", Args::Args(&[]), Vic3(CivilWar)),
    ("COMBAT_UNIT", Args::Args(&[]), Vic3(CombatUnit)),
    ("COMMANDER_ORDER", Args::Args(&[]), Vic3(CommanderOrder)),
    ("COMMANDER_ORDER_TYPE", Args::Args(&[]), Vic3(CommanderOrderType)),
    ("COMMANDER_RANK", Args::Args(&[]), Vic3(CommanderRank)),
    ("CONSTRUCTION_QUEUE_ELEMENT", Args::Args(&[]), Vic3(ConstructionQueueElement)),
    ("CONTEXTUAL_DIPLOMATIC_ACTION_TYPE", Args::Args(&[]), Vic3(ContextualDiplomaticActionType)),
    ("CONTEXTUAL_DIPLOMATIC_PACT", Args::Args(&[]), Vic3(ContextualDiplomaticPact)),
    ("COUNTRY", Args::Args(&[]), Vic3(Country)),
    ("COUNTRY_CREATION", Args::Args(&[]), Vic3(CountryCreation)),
    ("COUNTRY_DEFINITION", Args::Args(&[]), Vic3(CountryDefinition)),
    ("COUNTRY_FORMATION", Args::Args(&[]), Vic3(CountryFormation)),
    ("COUNTRY_THIRD_PARTY", Args::Args(&[]), Vic3(Country)),
    ("CULTURE", Args::Args(&[]), Vic3(Culture)),
    ("DATE", Args::Args(&[]), Date),
    ("DATE_MAX", Args::Args(&[]), Date),
    ("DATE_MIN", Args::Args(&[]), Date),
    ("DECISION_TYPE", Args::Args(&[]), Vic3(Decision)),
    ("DECREE", Args::Args(&[]), Vic3(Decree)),
    ("DECREE_TYPE", Args::Args(&[]), Vic3(DecreeType)),
    ("DIPLOMATIC_ACTION", Args::Args(&[]), Vic3(DiplomaticAction)),
    ("DIPLOMATIC_ACTION_TYPE", Args::Args(&[]), Vic3(DiplomaticActionType)),
    ("DIPLOMATIC_PACT", Args::Args(&[]), Vic3(DiplomaticPact)),
    ("DIPLOMATIC_PLAY", Args::Args(&[]), Vic3(DiplomaticPlay)),
    ("DIPLOMATIC_PLAY_TYPE", Args::Args(&[]), Vic3(DiplomaticPlayType)),
    ("DIPLOMATIC_RELATIONS", Args::Args(&[]), Vic3(DiplomaticRelations)),
    ("DISCRIMINATION_TRAIT", Args::Args(&[]), Vic3(DiscriminationTrait)),
    ("DLC_METADATA", Args::Args(&[]), Vic3(Dlc)),
    ("ELECTION", Args::Args(&[]), Vic3(Election)),
    ("EMPLOYEE_TRANSFER", Args::Args(&[]), Vic3(EmployeeTransfer)),
    ("ENEMY_COUNTRY", Args::Args(&[]), Vic3(Country)),
    ("ERA", Args::Args(&[]), Vic3(Era)),
    ("EVENT", Args::Args(&[]), Vic3(Event)),
    ("EmptyScope", Args::Args(&[]), TopScope),
    ("FIRSTMARKET", Args::Args(&[]), Vic3(Market)),
    ("FRONT", Args::Args(&[]), Vic3(Front)),
    ("FRONTPARTICIPANT", Args::Args(&[]), Vic3(FrontParticipant)),
    ("GAME_CONCEPT_TYPE", Args::Args(&[]), Vic3(GameConceptType)),
    ("GAME_RULE", Args::Args(&[]), Vic3(GameRule)),
    ("GAME_RULE_SETTING", Args::Args(&[]), Vic3(GameRuleSetting)),
    ("GC", Args::Args(&[DType(Unknown)]), Vic3(GameConceptType)),
    ("GOODS", Args::Args(&[]), Vic3(Goods)),
    ("GOODSPRODUCTIONINFO", Args::Args(&[]), Vic3(GoodsProductionInfo)),
    ("GOVERNMENT_TYPE", Args::Args(&[]), Vic3(GovernmentType)),
    ("GetBattleCondition", Args::Args(&[DType(Unknown)]), Vic3(BattleCondition)),
    ("GetBuildingType", Args::Args(&[DType(Unknown)]), Vic3(BuildingType)),
    ("GetConstructionSectorType", Args::Args(&[]), Vic3(BuildingType)),
    ("GetCurrentGameDate", Args::Args(&[]), Date),
    ("GetDecreeType", Args::Args(&[DType(Unknown)]), Vic3(DecreeType)),
    ("GetDiplomaticActionType", Args::Args(&[DType(Unknown)]), Vic3(DiplomaticActionType)),
    ("GetDiplomaticPlayType", Args::Args(&[DType(Unknown)]), Vic3(DiplomaticPlayType)),
    ("GetExilePool", Args::Args(&[]), Vic3(ExilePool)),
    ("GetGameEndDate", Args::Args(&[]), Date),
    ("GetGameRules", Args::Args(&[]), Vic3(JominiGameRules)),
    ("GetGameStartDate", Args::Args(&[]), Date),
    ("GetGlobalVariable", Args::Args(&[DType(Unknown)]), Scope),
    ("GetGoods", Args::Args(&[DType(Unknown)]), Vic3(Goods)),
    ("GetIdeology", Args::Args(&[DType(Unknown)]), Vic3(Ideology)),
    ("GetInstitutionType", Args::Args(&[DType(Unknown)]), Vic3(InstitutionType)),
    ("GetInterestGroupVariant", Args::Args(&[DType(Unknown), DType(Unknown)]), Vic3(InterestGroup)),
    ("GetLawGroup", Args::Args(&[DType(Unknown)]), Vic3(LawGroup)),
    ("GetLawType", Args::Args(&[DType(Unknown)]), Vic3(LawType)),
    ("GetLegitimacyLevel", Args::Args(&[DType(Unknown)]), Vic3(LegitimacyLevel)),
    ("GetLensToolbar", Args::Args(&[]), Vic3(LensToolbar)),
    ("GetLoadingScreenManager", Args::Args(&[]), Vic3(LoadingScreenManager)),
    ("GetMetaPlayer", Args::Args(&[]), Vic3(Player)),
    ("GetPlayer", Args::Args(&[]), Vic3(Country)),
    ("GetPlayerJournalEntry", Args::Args(&[DType(Unknown)]), Vic3(JournalEntry)),
    ("GetPopType", Args::Args(&[DType(Unknown)]), Vic3(PopType)),
    ("GetRankModifier", Args::Args(&[DType(Unknown)]), Vic3(Modifier)),
    ("GetScriptedGui", Args::Args(&[IType(Item::ScriptedGui)]), Vic3(ScriptedGui)),
    ("GetServerInfo", Args::Args(&[]), Vic3(ServerInformation)),
    ("GetStaticModifier", Args::Args(&[IType(Item::Modifier)]), Vic3(Modifier)),
    ("GetTutorialJournalEntry", Args::Args(&[]), Vic3(JournalEntry)),
    ("GetVariableSystem", Args::Args(&[]), Vic3(VariableSystem)),
    ("GuiEditor", Args::Args(&[]), Vic3(GuiEditor)),
    ("GuiScope", Args::Args(&[]), TopScope),
    ("HQ", Args::Args(&[]), Vic3(Hq)),
    ("IDEOLOGY", Args::Args(&[]), Vic3(Ideology)),
    ("INITIATOR_COUNTRY", Args::Args(&[]), Vic3(Country)),
    ("INSTITUTION", Args::Args(&[]), Vic3(Institution)),
    ("INSTITUTION_TYPE", Args::Args(&[]), Vic3(InstitutionType)),
    ("INTEREST", Args::Args(&[]), Vic3(Interest)),
    ("INTEREST_GROUP", Args::Args(&[]), Vic3(InterestGroup)),
    ("INTEREST_GROUP_TRAIT", Args::Args(&[]), Vic3(InterestGroupTrait)),
    ("InformationPanelBar", Args::Args(&[]), Vic3(InformationPanelBar)),
    ("JOURNAL_ENTRY", Args::Args(&[]), Vic3(JournalEntry)),
    ("JOURNAL_ENTRY_TYPE", Args::Args(&[]), Vic3(JournalEntryType)),
    ("JominiPlayer", Args::Args(&[]), Vic3(Playable)),
    ("LAW", Args::Args(&[]), Vic3(Law)),
    ("LAW_GROUP", Args::Args(&[]), Vic3(LawGroup)),
    ("LAW_OTHER", Args::Args(&[]), Vic3(Law)),
    ("LAW_TYPE", Args::Args(&[]), Vic3(LawType)),
    ("LEGITIMACY_LEVEL", Args::Args(&[]), Vic3(LegitimacyLevel)),
    ("LabelingHelper", Args::Args(&[]), Vic3(LabelingHelper)),
    ("MARKET", Args::Args(&[]), Vic3(Market)),
    ("MARKET_GOODS", Args::Args(&[]), Vic3(MarketGoods)),
    ("MODIFIER", Args::Args(&[]), Vic3(Modifier)),
    ("MapInteractions", Args::Args(&[]), Vic3(MapInteractionManager)),
    ("MapListPanelManager", Args::Args(&[]), Vic3(MapListPanelManager)),
    ("MultiplayerSynchronizationInfo", Args::Args(&[]), Vic3(MultiplayerSynchronizationInfo)),
    ("NAVAL_INVASION", Args::Args(&[]), Vic3(NavalInvasion)),
    ("OBJECTIVE", Args::Args(&[]), Vic3(Objective)),
    ("OBJECTIVE_TYPE", Args::Args(&[]), Vic3(ObjectiveType)),
    ("OBJECT_SCOPE", Args::Args(&[]), Scope),
    ("Outliner", Args::Args(&[]), Vic3(Outliner)),
    ("PARTY", Args::Args(&[]), Vic3(Party)),
    ("PLAYER", Args::Args(&[]), Vic3(Player)),
    ("POLITICAL_MOVEMENT", Args::Args(&[]), Vic3(PoliticalMovement)),
    ("POP", Args::Args(&[]), Vic3(Pop)),
    ("POP_GOODS_CONSUMPTION", Args::Args(&[]), Vic3(PopConsumptionGoods)),
    ("POP_NEED", Args::Args(&[]), Vic3(PopNeed)),
    ("POP_TYPE", Args::Args(&[]), Vic3(PopType)),
    ("POP_WITH_IG", Args::Args(&[]), Vic3(PopWithIG)),
    ("PREV", Args::Args(&[]), Scope),
    ("PRODUCTION_METHOD", Args::Args(&[]), Vic3(ProductionMethod)),
    ("PRODUCTION_METHOD_GROUP", Args::Args(&[]), Vic3(ProductionMethodGroup)),
    ("PROPOSAL", Args::Args(&[]), Vic3(Proposal)),
    ("PROVINCE", Args::Args(&[]), Vic3(Province)),
    ("PROVINCE_OTHER", Args::Args(&[]), Vic3(Province)),
    ("PopupManager", Args::Args(&[]), Vic3(PopupManager)),
    ("RELIGION", Args::Args(&[]), Vic3(Religion)),
    ("ROOT", Args::Args(&[]), Scope),
    ("RightClickMenuManager", Args::Args(&[]), Vic3(RightClickMenuManager)),
    ("Root", Args::Args(&[]), Scope),
    ("SCOPE", Args::Args(&[]), TopScope),
    ("SCRIPTED_BUTTON", Args::Args(&[]), Vic3(ScriptedButton)),
    ("SECONDMARKET", Args::Args(&[]), Vic3(Market)),
    ("SHIPPING_LANE", Args::Args(&[]), Vic3(ShippingLane)),
    ("STATE", Args::Args(&[]), Vic3(State)),
    ("STATE_OTHER", Args::Args(&[]), Vic3(State)),
    ("STATE_REGION", Args::Args(&[]), Vic3(StateRegion)),
    ("STATE_TRAIT", Args::Args(&[]), Vic3(StateTrait)),
    ("STRATEGIC_REGION", Args::Args(&[]), Vic3(StrategicRegion)),
    ("SUBJECT_SCOPE", Args::Args(&[]), Scope),
    ("TARGET_AIATTITUDE", Args::Args(&[]), Vic3(AIAttitude)),
    ("TARGET_AISTRATEGY", Args::Args(&[]), Vic3(AIStrategy)),
    ("TARGET_ALERT", Args::Args(&[]), Vic3(Alert)),
    ("TARGET_ALERT_GROUP", Args::Args(&[]), Vic3(AlertGroup)),
    ("TARGET_BATTLE", Args::Args(&[]), Vic3(Battle)),
    ("TARGET_BATTLE_CONDITION", Args::Args(&[]), Vic3(BattleCondition)),
    ("TARGET_BUILDING", Args::Args(&[]), Vic3(Building)),
    ("TARGET_BUILDING_TYPE", Args::Args(&[]), Vic3(BuildingType)),
    ("TARGET_CANAL_TYPE", Args::Args(&[]), Vic3(CanalType)),
    ("TARGET_CHARACTER", Args::Args(&[]), Vic3(Character)),
    ("TARGET_CHARACTER_INTERACTION", Args::Args(&[]), Vic3(CharacterInteraction)),
    ("TARGET_CHARACTER_TRAIT", Args::Args(&[]), Vic3(CharacterTrait)),
    ("TARGET_CIVIL_WAR", Args::Args(&[]), Vic3(CivilWar)),
    ("TARGET_COMBAT_UNIT", Args::Args(&[]), Vic3(CombatUnit)),
    ("TARGET_COMMANDER_ORDER", Args::Args(&[]), Vic3(CommanderOrder)),
    ("TARGET_COMMANDER_ORDER_TYPE", Args::Args(&[]), Vic3(CommanderOrderType)),
    ("TARGET_COMMANDER_RANK", Args::Args(&[]), Vic3(CommanderRank)),
    ("TARGET_CONSTRUCTION_QUEUE_ELEMENT", Args::Args(&[]), Vic3(ConstructionQueueElement)),
    ("TARGET_CONTEXTUAL_DIPLOMATIC_ACTION_TYPE", Args::Args(&[]), Vic3(ContextualDiplomaticActionType)),
    ("TARGET_CONTEXTUAL_DIPLOMATIC_PACT", Args::Args(&[]), Vic3(ContextualDiplomaticPact)),
    ("TARGET_COUNTRY", Args::Args(&[]), Vic3(Country)),
    ("TARGET_COUNTRY_CREATION", Args::Args(&[]), Vic3(CountryCreation)),
    ("TARGET_COUNTRY_DEFINITION", Args::Args(&[]), Vic3(CountryDefinition)),
    ("TARGET_COUNTRY_FORMATION", Args::Args(&[]), Vic3(CountryFormation)),
    ("TARGET_CULTURE", Args::Args(&[]), Vic3(Culture)),
    ("TARGET_DECISION_TYPE", Args::Args(&[]), Vic3(Decision)),
    ("TARGET_DECREE", Args::Args(&[]), Vic3(Decree)),
    ("TARGET_DECREE_TYPE", Args::Args(&[]), Vic3(DecreeType)),
    ("TARGET_DIPLOMATIC_ACTION", Args::Args(&[]), Vic3(DiplomaticAction)),
    ("TARGET_DIPLOMATIC_ACTION_TYPE", Args::Args(&[]), Vic3(DiplomaticActionType)),
    ("TARGET_DIPLOMATIC_PACT", Args::Args(&[]), Vic3(DiplomaticPact)),
    ("TARGET_DIPLOMATIC_PLAY", Args::Args(&[]), Vic3(DiplomaticPlay)),
    ("TARGET_DIPLOMATIC_PLAY_TYPE", Args::Args(&[]), Vic3(DiplomaticPlayType)),
    ("TARGET_DIPLOMATIC_RELATIONS", Args::Args(&[]), Vic3(DiplomaticRelations)),
    ("TARGET_DISCRIMINATION_TRAIT", Args::Args(&[]), Vic3(DiscriminationTrait)),
    ("TARGET_DLC_METADATA", Args::Args(&[]), Vic3(Dlc)),
    ("TARGET_ELECTION", Args::Args(&[]), Vic3(Election)),
    ("TARGET_EMPLOYEE_TRANSFER", Args::Args(&[]), Vic3(EmployeeTransfer)),
    ("TARGET_ERA", Args::Args(&[]), Vic3(Era)),
    ("TARGET_EVENT", Args::Args(&[]), Vic3(Event)),
    ("TARGET_FRONT", Args::Args(&[]), Vic3(Front)),
    ("TARGET_GAME_CONCEPT_TYPE", Args::Args(&[]), Vic3(GameConceptType)),
    ("TARGET_GOODS", Args::Args(&[]), Vic3(Goods)),
    ("TARGET_GOVERNMENT_TYPE", Args::Args(&[]), Vic3(GovernmentType)),
    ("TARGET_HQ", Args::Args(&[]), Vic3(Hq)),
    ("TARGET_IDEOLOGY", Args::Args(&[]), Vic3(Ideology)),
    ("TARGET_INSTITUTION", Args::Args(&[]), Vic3(Institution)),
    ("TARGET_INSTITUTION_TYPE", Args::Args(&[]), Vic3(InstitutionType)),
    ("TARGET_INTEREST", Args::Args(&[]), Vic3(Interest)),
    ("TARGET_INTEREST_GROUP", Args::Args(&[]), Vic3(InterestGroup)),
    ("TARGET_INTEREST_GROUP_TRAIT", Args::Args(&[]), Vic3(InterestGroupTrait)),
    ("TARGET_JOURNAL_ENTRY", Args::Args(&[]), Vic3(JournalEntry)),
    ("TARGET_JOURNAL_ENTRY_TYPE", Args::Args(&[]), Vic3(JournalEntryType)),
    ("TARGET_LAW", Args::Args(&[]), Vic3(Law)),
    ("TARGET_LAW_GROUP", Args::Args(&[]), Vic3(LawGroup)),
    ("TARGET_LAW_TYPE", Args::Args(&[]), Vic3(LawType)),
    ("TARGET_LEGITIMACY_LEVEL", Args::Args(&[]), Vic3(LegitimacyLevel)),
    ("TARGET_MARKET", Args::Args(&[]), Vic3(Market)),
    ("TARGET_MARKET_GOODS", Args::Args(&[]), Vic3(MarketGoods)),
    ("TARGET_MODIFIER", Args::Args(&[]), Vic3(Modifier)),
    ("TARGET_NAVAL_INVASION", Args::Args(&[]), Vic3(NavalInvasion)),
    ("TARGET_OBJECTIVE", Args::Args(&[]), Vic3(Objective)),
    ("TARGET_OBJECTIVE_TYPE", Args::Args(&[]), Vic3(ObjectiveType)),
    ("TARGET_PARTY", Args::Args(&[]), Vic3(Party)),
    ("TARGET_PLAYER", Args::Args(&[]), Vic3(Player)),
    ("TARGET_POLITICAL_MOVEMENT", Args::Args(&[]), Vic3(PoliticalMovement)),
    ("TARGET_POP", Args::Args(&[]), Vic3(Pop)),
    ("TARGET_POP_GOODS_CONSUMPTION", Args::Args(&[]), Vic3(PopConsumptionGoods)),
    ("TARGET_POP_NEED", Args::Args(&[]), Vic3(PopNeed)),
    ("TARGET_POP_TYPE", Args::Args(&[]), Vic3(PopType)),
    ("TARGET_POP_WITH_IG", Args::Args(&[]), Vic3(PopWithIG)),
    ("TARGET_PRODUCTION_METHOD", Args::Args(&[]), Vic3(ProductionMethod)),
    ("TARGET_PRODUCTION_METHOD_GROUP", Args::Args(&[]), Vic3(ProductionMethodGroup)),
    ("TARGET_PROPOSAL", Args::Args(&[]), Vic3(Proposal)),
    ("TARGET_PROVINCE", Args::Args(&[]), Vic3(Province)),
    ("TARGET_RELIGION", Args::Args(&[]), Vic3(Religion)),
    ("TARGET_SCRIPTED_BUTTON", Args::Args(&[]), Vic3(ScriptedButton)),
    ("TARGET_SHIPPING_LANE", Args::Args(&[]), Vic3(ShippingLane)),
    ("TARGET_STATE", Args::Args(&[]), Vic3(State)),
    ("TARGET_STATE_REGION", Args::Args(&[]), Vic3(StateRegion)),
    ("TARGET_STATE_TRAIT", Args::Args(&[]), Vic3(StateTrait)),
    ("TARGET_STRATEGIC_REGION", Args::Args(&[]), Vic3(StrategicRegion)),
    ("TARGET_TECHNOLOGY", Args::Args(&[]), Vic3(Technology)),
    ("TARGET_THEATER", Args::Args(&[]), Vic3(Theater)),
    ("TARGET_THEME", Args::Args(&[]), Vic3(Theme)),
    ("TARGET_TRADE_ROUTE", Args::Args(&[]), Vic3(TradeRoute)),
    ("TARGET_WAR", Args::Args(&[]), Vic3(War)),
    ("TARGET_WAR_GOAL", Args::Args(&[]), Vic3(WarGoal)),
    ("TARGET_WAR_GOAL_TYPE", Args::Args(&[]), Vic3(WarGoalType)),
    ("TECHNOLOGY", Args::Args(&[]), Vic3(Technology)),
    ("THEATER", Args::Args(&[]), Vic3(Theater)),
    ("THEME", Args::Args(&[]), Vic3(Theme)),
    ("THIS", Args::Args(&[]), Scope),
    ("TRADE_ROUTE", Args::Args(&[]), Vic3(TradeRoute)),
    ("TimeKeeper", Args::Args(&[]), Vic3(TimeKeeper)),
    ("WAR", Args::Args(&[]), Vic3(War)),
    ("WAR_GOAL", Args::Args(&[]), Vic3(WarGoal)),
    ("WAR_GOAL_TYPE", Args::Args(&[]), Vic3(WarGoalType)),
    ("WarManager", Args::Args(&[]), Vic3(WarManager)),
]
